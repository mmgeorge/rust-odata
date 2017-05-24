
use std::collections::HashMap;
use std::sync::Arc;
use std::mem;

use hyper::server::Server;

use model::Model;
use service::handler::ServiceHandler;


pub struct Service {
    name : Arc<String>,
    models: Arc<HashMap<String, Model>>,
}


impl Service {

    pub fn start(&self)
    {
        // let test = "helloworld";
        let handler = ServiceHandler {
            name : "test",
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


    pub fn build(mut self) -> Service
    {
        let mut models = HashMap::new();
        let mut name   = String::new();
        mem::swap(&mut self.models, &mut models);
        mem::swap(&mut self.name, &mut name);
        
        Service {
            name: Arc::new(name),
            models: Arc::new(models),
        }

    }
}
