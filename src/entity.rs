
// Property::new("name", "edm.string");
// entityset.entity("dksd-sdfz").property("value").get("name");
// entity.property("value").set("x");
// entity.property("is20").declare(Edm::Boolean);
// entity.add("is20", Edm::Boolean);
// pub fn<T: Debug> getType (T: value) {
//     println!("{:?}", value);
// }


use std::collections::HashMap;
use edm::Edm;
use std::clone::Clone;

/// Structure for holding property values for an Entity
#[derive(Clone)]
pub struct Property {
    name: String,
    value: Edm::Value, 
}


type PropertyMap = HashMap<String, Property>;

/// Entity type definition. Create using EntityBuilder.
pub struct Entity {
    name: String, 
    properties: PropertyMap,
}

impl Entity {

    /// Entity constructor. Generally this shoudl not be called directly
    /// and EntityBuilder used instead. 
    pub fn new(name: &str, properties: PropertyMap) -> Self {
        Entity {
            name: String::from(name),
            properties: properties
        }
    }
    
    pub fn property(&self, key: &str) -> &Property {
        &self.properties[key]
    }
}


/// Construct a new Entity type. For use with in the declare function of an
/// EntitySet
pub struct EntityBuilder {
    name: String, 
    properties: PropertyMap,
}


impl EntityBuilder {

    pub fn new(name: &str) -> Self {
        EntityBuilder {
            name: String::from(name),
            properties: PropertyMap::new()
        }
    }
    
    pub fn add (&mut self, key: &str, ty: Edm::Type) -> &mut Self {
        self.properties.insert(
            String::from(key),
            Property {
                name: String::from(key),
                value: Edm::toValue(&ty)
            });
        self
    }

    pub fn build(&self) -> Entity {
        let entity = Entity::new(&self.name, self.properties.clone());
        entity
    }
}


