
use std::collections::HashMap;
use std::iter::repeat;
use std::mem;
use std::marker::Sync;

use property::Property; 
use entity_set::{EntitySet, EntitySetDescr};
use serde_json::*;


pub struct Model {
    /// Registery for storing metadata for each included
    /// EntitySet. Keys denote EntitySets, Values each EntitySets'
    /// respective properties.
    name: String,
    registry: Vec<Box<EntitySetDescr>>,
    metadata: Value
}

unsafe impl Sync for Model { }
unsafe impl Send for Model { }

impl Model {

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
            let descr = set.descriptor(); // Load Entity decscriptor

            // Add EntitySet to metadata
            self.metadata["entityContainer"]["entitySets"][set.name()] = json!({
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
}


pub struct ModelBuilder {
    name: String,
    registry: Vec<Box<EntitySetDescr>>
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
        where E: EntitySetDescr + EntitySet + 'static
    {
        self.registry.push(Box::new(set));
        self
    }
    

    pub fn build(&mut self) -> Model {
        let reg = mem::replace(&mut self.registry, Vec::new());
        let name = mem::replace(&mut self.name, String::new());
        
        let mut model = Model {
            name: name,
            registry: reg,
            metadata: json!({"None": "None"})
        };
        model.render();
        model
    }
}
