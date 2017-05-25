
use std::io::Write;
use std::sync::Arc;
use std::collections::HashMap;
use std::clone::Clone;
use std::mem;

use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::uri::RequestUri;
use serde_json::{Value, to_vec, to_string};

use model::{Model, EsEntry};
use service::Error;
use entity::{EntityDescr, EntitySetDescr};

type SetDescr = Box<EntitySetDescr>;


pub enum Res {
    Succ(Option<Value>),
    Err(Error)
}


/// An oData parameter (i.e. starts with $)
#[derive(Clone)] // derive Clone is temporary only (see workaround)
enum Param {
    Metadata,
    None
}


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
    fn parse_params()
    {
        
    }

    
    /// Validates the selected uri, returning a tuple (model, set, parameters)
    /// for representing the request  or corresponding error.
    fn validate(&self, uri: RequestUri)
                -> Result<(&Model, Option<&EsEntry>, Vec<Param>), Error>
    {
        let uri = uri.to_string();
        let mut parts = uri.split('/');
        let size = parts.clone().count();
        parts.next(); // parts: nil, root, model, opt(set), opt(query)

        if size < 3 || (parts.next().unwrap() != *self.root) {
            return Err(Error::InvalidRoot);
        }

        let model = self.lookup(parts.next().unwrap());
        let mut entity_set = None;  
        let mut params = Vec::new(); // - unimplmented
        
        if model.is_none() {
            return Err(Error::InvalidModel);
        }

        // Parse remaing portion of uri. 
        if let Some(part) = parts.next()  {

            // First check if request points to an EntitySet
            entity_set = model.unwrap().lookup(part);

            // Otherwise check that the part points to $metadata
            if entity_set.is_none() {
                if part == "$metadata"  {
                    params.push(Param::Metadata);
                }
            }
        }

        
        
        
        // unimplemented - do params parsing
        Ok((model.unwrap(), entity_set, params))
    }


    /// Routes request to designated set's CRUD-Q implementations
    fn satisfy(&self, body: (Method, &Model, Option<&EsEntry>, &[Param])) -> Res
    {
        // Wanted to use slice matches for this, but it looks like they are an
        // experimental only feature. This is a workaround (only because we have
        // a limited number of allowable params currently) that extracts the slice
        // as a tuple :

        let model = body.1; 
        let body = match body.3.len() {
            0 => (body.0, body.2, Param::None),
            1 => (body.0, body.2, body.3[0].clone()), // cloning is ugly but temporary
            _ => unimplemented!()
        };
        
        // end workaround
        
        match body {
            // Metadata document request
            (Method::Get, None, Param::Metadata) => Res::Succ(Some(model.get_metadata().clone())),
            
            // Model document request
            (Method::Get, None, Param::None) => panic!("requires entity document!"),

            // ReadList
            (Method::Get, Some(set), _) => set.1.read_list(), 

            // Get
            // (Method::Get, None, _) => ResType::Succ(None),
            // (Method::Post, Some(_), _) => ResType::Succ(None),
            _ => panic!(),
        }
    }

    /// Responds to request based on ResType
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
            Res::Err(Error::InvalidModel) => println!("invalidModel"),
            Res::Err(Error::InvalidRoot) => println!("invalid root!"),
            _ => println!("unimplemented!"),
        }
    }
}


impl Handler for ServiceHandler {
    fn handle(&self, req: Request, res: Response)
    {
        let action = match self.validate(req.uri) {     
            Ok((model, set, params)) => self.satisfy((req.method, model, set, params.as_slice())),
            Err(e) => Res::Err(e),
        };

        self.respond(action, res);

        //         io::copy(&mut req, &mut res.start().unwrap()).unwrap(); 
        //     _ => *res.status_mut() = StatusCode::MethodNotAllowed
    }
}

