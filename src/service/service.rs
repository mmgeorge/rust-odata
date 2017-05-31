
use std::collections::HashMap;
use std::sync::Arc;
use hyper::server::Server;

use model::Model;
use service::handler::ServiceHandler;


/// An instance of an oData service that will serve each model it contains. Constructed
/// using a ServiceBuilder. 
pub struct Service {
    root : Arc<String>,
    models: Arc<HashMap<String, Model>>,
}


impl Service {

    /// Start accepting requests. 
    pub fn start(&self)
    {
        // let test = "helloworld";
        let handler = ServiceHandler {
            root  : self.root.clone(),
            models: self.models.clone()
        };
        
        Server::http("0.0.0.0:8080").unwrap().handle(handler).unwrap();
    }
}


/// Constructs an instance of a Service. 
pub struct ServiceBuilder {
    name: String,
    models: HashMap<String, Model>
}


impl ServiceBuilder {
    /// Creates a new ServiceBuilder to build a Service with the passed name
    pub fn new(name: &str) -> ServiceBuilder
    {
        ServiceBuilder {
            name: String::from(name),
            models: HashMap::new()
        }
    }

    /// Add a model to the service.
    pub fn add(mut self, model: Model) -> Self
    {
        self.models.insert(String::from(model.get_name()), model);
        self
    }
    
    /// Construct the service
    pub fn build(self) -> Service
    {
        Service {
            root: Arc::new(self.name),
            models: Arc::new(self.models),
        }
    }
}
