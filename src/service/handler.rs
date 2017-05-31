
use std::io::{Read, Write};
use std::sync::Arc;
use std::collections::HashMap;
use std::clone::Clone;
use std::mem;

use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::uri::RequestUri;
use serde_json::{Value, to_vec, to_string, from_slice, from_str};

use model::{Model, EsEntry};
use service::Error;
use entity::{EntityDescr, EntitySetDescr};

type SetDescr = Box<EntitySetDescr>;

/// Used for routing requests and for simplifying CRUD-Q implementation. Providing the
/// correct HTTP StatusCode is handled by the server.
pub enum Res {
    Succ(Option<Value>),
    Created(Value),
    Err(Error)
}


/// An oData parameter (i.e. starts with $)
#[derive(Clone)] // derive Clone is temporary only (see workaround)
enum Param {
    Metadata,
    Key(String),
    None
}

/// An internal type used by the Service class for handling requests.
pub struct ServiceHandler {
    pub root: Arc<String>,
    pub models: Arc<HashMap<String, Model>>
}

impl ServiceHandler {

    /// Lookup specified model
    fn lookup(&self, name: &str) -> Option<&Model>
    {
        self.models.get(name)
    }


    /// Parses query parameters ($)
    fn parse_params() {} // unimplemented

    
    /// Validates the selected uri, returning a tuple (model, set, parameters)
    /// for representing the request  or corresponding error.
    fn validate(&self, uri: RequestUri)
                -> Result<(&Model,
                           Option<&EsEntry>,
                           Option<&Box<Fn(Value) -> Res>>, // Action
                           Vec<Param>), Error>
    {
        let uri = uri.to_string();
        let mut parts = uri.split('/');
        let size = parts.clone().count();
        parts.next(); // parts: nil, root, model, opt(set), opt(query)

        if size < 3 || (parts.next().unwrap() != *self.root) {
            return Err(Error::InvalidRoot);
        }

        let model;
        match self.lookup(parts.next().unwrap()) {
            Some(m) => model = m,
            _       => return Err(Error::InvalidModel)
        }
        
        let mut entity_set = None;  
        let mut params = Vec::new();
        let mut action = None;

        // Parse remaing portion of uri. 
        if let Some(part) = parts.next()  {
            let mut set = part; 
            
            // First check if part contains a key lookup, i.e. /customer.svc/Customers(1234)
            let sub_parts: Vec<&str> = part.split("(").collect();
            if sub_parts.len() == 2 { // e.g. Customers, 1234)
                set = sub_parts[0];
                let key: String = sub_parts[1].chars().filter(|x| x.is_numeric()).collect();
                params.push(Param::Key(key));
            }
            
            // Next see if request points to an EntitySet
            entity_set = model.lookup(set);
            
            if entity_set.is_none() {
                // Otherwise check that the part points to $metadata
                if part == "$metadata"  {
                    params.push(Param::Metadata);
                }
            }
            // Otherwise check that part points to an unbounded action
            action = model.lookup_action(part);
        }
        Ok((model, entity_set, action, params))
    }


    /// Routes request to designated set's CRUD-Q implementations
    fn satisfy(&self, req_body: &[u8],
               body: (Method,
                      &Model, Option<&EsEntry>,
                      Option<&Box<Fn(Value) -> Res>>, // Action
                      &[Param]))
               -> Res
    {
        // Wanted to use slice matches for this, but it looks like they are an
        // experimental only feature. This is a workaround (only because we have
        // a limited number of allowable params currently) that extracts the slice
        // as a tuple :

        let model = body.1;
        let body_comps = match body.4.len() {
            0 => (body.0, body.2, body.3, Param::None),
            1 => (body.0, body.2, body.3, body.4[0].clone()), // cloning is ugly but temporary
            _ => unimplemented!()
        };
        
        // end workaround
        
        match body_comps {
            // Metadata document request
            (Method::Get, None, None, Param::Metadata)
                => Res::Succ(Some(model.get_metadata().clone())),

            // Model document request
            (Method::Get, None, None, Param::None) => panic!("requires entity document!"),

            // Read List
            (Method::Get, Some(set), None, Param::None) => set.1.read_list(), 

            // Read Entity
            (Method::Get, Some(set), None, Param::Key(key)) => set.1.read(key),

            // Create Entity
            (Method::Post, Some(set), None, Param::None)
                => set.1.create(from_slice(req_body).expect("Unable to parse request body!")),

            // Action
            (Method::Post, None, Some(action), Param::None)
                => action(from_slice(req_body).expect("Unable to parse request body!")),
            
            _ => panic!(), // Triggers a 500
        }
    }

    /// Responds to a request based on ResType
    fn respond(&self, rt: Res, mut res: Response)
    {
        use hyper::header::*;
        use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
        use hyper::status::StatusCode;
        use time;
        
        // Write shared headers
        res.headers_mut().set(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                               vec![(Attr::Charset, Value::Utf8)])));
        res.headers_mut().set(Date(HttpDate(time::now())));
        
        
        match rt {
            Res::Succ(None) => println!("Success! : No response"),
            Res::Succ(value) => {
                mem::swap(res.status_mut(), &mut StatusCode::Ok); // ugly...
                
                let body = to_vec(&value).unwrap();
                let content = body.as_slice();
                res.headers_mut().set(ContentLength(content.len() as u64));
                
                res.start().unwrap().write_all(content).unwrap();
            },
            Res::Created(value) => {
                mem::swap(res.status_mut(), &mut StatusCode::Created); // ugly...
                
                let body = to_vec(&value).unwrap();
                let content = body.as_slice();
                res.headers_mut().set(ContentLength(content.len() as u64));
                
                res.start().unwrap().write_all(content).unwrap();
            },
            Res::Err(Error::NotFound(resource)) => {
                mem::swap(res.status_mut(), &mut StatusCode::NotFound); // ugly...

                let value = json!({
                    "odata.error": {
                        "code": "",
                        "message": {
                            "lang": "en-US",
                            "value": String::from("Resource not found for the segment '")
                                + &resource + "'."
                        }
                    }
                });
                
                let body = to_vec(&value).unwrap();
                let content = body.as_slice();
                res.headers_mut().set(ContentLength(content.len() as u64));
                
                res.start().unwrap().write_all(content).unwrap();
            },
            Res::Err(Error::InvalidModel) => println!("invalidModel"),
            Res::Err(Error::InvalidRoot) => println!("invalid root!"),
            _ => {
                mem::swap(res.status_mut(), &mut StatusCode::BadRequest); // ugly...

                let value = json!({
                    "odata.error": {
                        "code": "400",
                        "message": {
                            "lang": "en-US",
                            "value": String::from("Bad request")
                        }
                    }
                });
                
                let body = to_vec(&value).unwrap();
                let content = body.as_slice();
                res.headers_mut().set(ContentLength(content.len() as u64));
                
                res.start().unwrap().write_all(content).unwrap();
            }
        }
    }
}


impl Handler for ServiceHandler {
    fn handle(&self, mut req: Request, res: Response)
    {
        use std;
        
        // Read contents of message
        let mut buf = Vec::new();
        req.read_to_end(&mut buf).unwrap();
        
        let action = match self.validate(req.uri) {     
            Ok((model, set, action, params)) =>
                self.satisfy(buf.as_slice(), (req.method, model, set, action, params.as_slice())),
            
            Err(e) => Res::Err(e),
        };

        self.respond(action, res);
    }
}

