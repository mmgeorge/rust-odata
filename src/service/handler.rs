
use std::io;
use std::sync::Arc;
use std::collections::HashMap;

use hyper::server::{Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;

use model::Model;


pub struct ServiceHandler {
    pub models: Arc<HashMap<String, Model>>
        
}


impl ServiceHandler {}


impl Handler for ServiceHandler {
    fn handle(&self, mut req: Request, mut res: Response)
    {
        // See if path points to a valid oData server
        let uri = req.uri.to_string();
        let components = uri.split('/');
        let size = components.clone().count();

        //get_model(&url) -> Result<model, Err(No resource)>
        
        println!("{}", uri);
        
        //println!("{:?}", req.uri);
        match req.method {
            Method::Post => {
                io::copy(&mut req, &mut res.start().unwrap()).unwrap(); 
            },
            _ => *res.status_mut() = StatusCode::MethodNotAllowed
        }
    }
}

