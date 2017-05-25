
use std::marker::Sync;

use entity::*;
use serde_json::Value;

/// Trait object for accessing both an EntitySet's descriptor and CRUD-Q implementation
pub type EsEntry = (Box<EntitySetDescr>, Box<EntitySet>); 

/// Registry for storing metadata for each included
/// EntitySet. Keys denote EntitySets, Values each EntitySets'
/// respective properties.
pub struct Model {
    name: String,
    registry: Vec<EsEntry>, 
    metadata: Value
}

unsafe impl Sync for Model {}
unsafe impl Send for Model {}

impl Model {

    /// Lookup specified descriptor
    pub fn lookup(&self, name: &str) -> Option<&EsEntry>
    {
        for entry in &self.registry {
            if entry.0.name() == name {
                return Some(&entry)
            }
        }
        None
    }

    
    /// Renders the metadata document description of the oData Model
    pub fn render(&mut self)
    {
        self.metadata = json!({
            "$schema": "http://docs.oasis-open.org/odata/odata-json-csdl/v4.0/edm.json#",
            "odata-version": "4.0",
            "definitions": {},
            "actions": "",
            "functions": "",
            "terms": "",
            "entityContainer": {
                "name": "ServiceName",
                "entitySets": {}
            },
            "schemas": "",
            "references": ""
        });

        for set in &self.registry  {
            let descr = set.0.descriptor(); // Load Entity decscriptor

            // Add EntitySet to metadata
            self.metadata["entityContainer"]["entitySets"][set.0.name()] = json!({
                "entityType": {
                    "$ref": String::from("#/definitions/") + descr.name()
                }
            });

            // Add Entity type to metadata
            self.metadata["definitions"][descr.name()] = json!({
                "type": "object",
                "keys": descr.keys(),
                "properties": {}
            });
            
            for property in descr.properties() {
                self.metadata["definitions"][descr.name()]["properties"][property.name()] = json!({
                    "type": property.types(),
                    "format": property.format()
                });
            }
        }
    }


    /// Getter for the metadata document
    pub fn get_metadata(&self) -> &Value
    {
        &self.metadata
    }


    /// Getter for model name
    pub fn get_name(&self) -> &str
    {
        &self.name
    }
}


pub struct ModelBuilder {
    name: String,
    registry: Vec<EsEntry>
}


impl ModelBuilder {
    pub fn new(name: &str) -> ModelBuilder
    {
        ModelBuilder {
            name: String::from(name),
            registry: Vec::new()
        }
    }

    
    /// Add a new EntitySet
    pub fn add<E> (mut self, set: E) -> ModelBuilder
        where E: EntitySetDescr + EntitySet + 'static + Clone
    {
        self.registry.push((Box::new(set.clone()), Box::new(set)));
        self
    }
    

    pub fn build(self) -> Model {
        let mut model = Model {
            name: self.name,
            registry: self.registry,
            metadata: json!({"None": "None"})
        };
        model.render();
        model
    }
}
