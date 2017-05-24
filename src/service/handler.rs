
use std::io;
use std::sync::Arc;
use std::collections::HashMap;

use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::uri::RequestUri;
use serde_json::Value;

use model::Model;
use service::Error;
use entity::{EntityDescr, EntitySetDescr};

type SetDescr = Box<EntitySetDescr>;


enum ResType {
    Succ(Option<Value>),
    Err(Error)
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

    
    /// Validates the selected uri, returning a tuple (model, set, parameters)
    /// for representing the request  or corresponding error.
    fn validate(&self, uri: RequestUri)
                -> Result<(&Model, Option<&SetDescr>, Option<String>), Error>
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
        let params = None; // - unimplmented
        
        if model.is_none() {
            return Err(Error::InvalidModel);
        }
        
        if let Some(set) = parts.next()  {
            entity_set = model.unwrap().lookup(set);
        }
        // unimplemented - do params parsing
        Ok((model.unwrap(), entity_set, params))
    }

    // WIP!
    /// Routes request to designated set's CRUD-Q implementations
    fn satisfy(&self, body: (Method, &Model, Option<&SetDescr>, Option<String>))
               -> ResType
    {
        match body {
            (Method::Get, _, Some(_), _) => ResType::Succ(None),
            _ => panic!(),
        }
    }

        
    /// Responds to request based on ResType
    fn respond(&self, rt: ResType, _res: Response)
    {
        match rt {
            ResType::Succ(_) => println!("Success!"),
            ResType::Err(Error::InvalidModel) => println!("invalidModel"),
            ResType::Err(Error::InvalidRoot) => println!("invalid root!"),
            _ => println!("unimplemented!"),
        }
    }
}


impl Handler for ServiceHandler {
    fn handle(&self, req: Request, res: Response)
    {
        let action = match self.validate(req.uri) {     
            Ok((model, set, params)) => self.satisfy((req.method, model, set, params)),
            Err(e) => ResType::Err(e),
        };

        self.respond(action, res);

        //         io::copy(&mut req, &mut res.start().unwrap()).unwrap(); 
        //     _ => *res.status_mut() = StatusCode::MethodNotAllowed
    }
}

