
use std::collections::HashMap;
use edm::Edm;
use std::clone::Clone;
use property::Property;


macro_rules! rust_type {
    (Int64) => { i64 };
}


/// Declare a new Entity
#[macro_export]
macro_rules! defEntity {
    ($name:ident {
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
            //pub fn get_key 
            fn describe() -> Vec<Property> {
                println!("Describing entity!");
                vec![$(Property::new(
                    stringify!($key),
                    stringify!($prop_type))),*]
            }
        }
    }
}


pub trait Entity {
    /// Used to expose fields to model. Passed-up to Model through EntitySet
    fn describe() -> Vec<Property>; 
}


