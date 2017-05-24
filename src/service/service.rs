
use std::collections::HashMap;
use std::sync::Arc;
use hyper::server::Server;

use model::Model;
use service::handler::ServiceHandler;


/// An instance of an oData service that will serve each model it possesses. Construct
/// using a ServiceBuilder. 
pub struct Service {
    name : Arc<String>,
    models: Arc<HashMap<String, Model>>,
}


impl Service {

    /// Begin accepting requests. 
    pub fn start(&self)
    {
        // let test = "helloworld";
        let handler = ServiceHandler {
            models: self.models.clone()
        };
        
        Server::http("0.0.0.0:8080").unwrap().handle(handler).unwrap();
    }
}


pub struct ServiceBuilder {
    name: String,
    models: HashMap<String, Model>
}


impl ServiceBuilder {
    pub fn new(name: &str) -> ServiceBuilder
    {
        ServiceBuilder {
            name: String::from(name),
            models: HashMap::new()
        }
    }


    pub fn add(mut self, model: Model) -> Self
    {
        self.models.insert(String::from("test"), model);
        self
    }


    pub fn build(self) -> Service
    {
        Service {
            name: Arc::new(self.name),
            models: Arc::new(self.models),
        }
    }
}
