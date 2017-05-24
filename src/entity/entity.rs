
// use std::collections::HashMap;
// use edm::edm;
// use std::clone::Clone;
use entity::Property;


pub struct EntityDescr {
    pub name: String,
    pub keys: Vec<String>, 
    pub properties: Vec<Property>,
}

impl EntityDescr {
    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn keys(&self) -> &[String]
    {
        &self.keys
    }

    pub fn properties(&self) -> &[Property]
    {
        &self.properties
    }
}


/// Declare a new Entity
#[macro_export]
macro_rules! defEntity {
    ($name:ident( keys => $($key_name:ident),* ) {
        $( $key:ident : $prop_type:ident ),*
    }) => {
        
        #[derive(Serialize, Deserialize, Debug)]
        struct $name {
            $( $key : rust_type!($prop_type) ),* // add OR clause here? 
        }
        
        impl $name {
            pub fn new($($key : rust_type!($prop_type)),*) -> $name {
                $name {
                    $( $key : $key ),*
                }
            }
        }

        impl Entity for $name {
            fn describe() -> EntityDescr {
                EntityDescr {
                    name: String::from(stringify!($name)),
                    keys: vec![$(String::from(stringify!($key_name))),*],
                    properties: vec![$(Property::new(
                        stringify!($key),
                        edm::from(stringify!($prop_type)))),*] 
                }
            }
        }
    }
}


pub trait Entity {
    /// Used to expose fields to model. Passed-up to Model through EntitySet
    fn describe() -> EntityDescr; 
}


