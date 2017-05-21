
use entity::{Entity, EntityDescr};
use model::Model;
use serde_json;

type Json = serde_json::Value;
type QueryOpt = String;


#[macro_export]
macro_rules! defEntitySet {
    ($name:ident, $entity:ident) => {

        struct $name { }
        
        impl $name {
            fn declare() -> $name
            {
                $name { }
            } 
            
            fn parse(value: &str) -> $entity
            {
                serde_json::from_str(value).expect("Could not deserialize entity")
            }
        }

        impl EntitySetDescr for $name {
            fn name(&self) -> String
            {
                String::from(stringify!($name))
            }

            fn descriptor(&self) -> EntityDescr
            {
                $entity::describe()
            }
        }
    }
} 


pub trait EntitySetDescr {
    fn name(&self) -> String;
    fn descriptor(&self) -> EntityDescr; 
}


pub trait EntitySet {
    
}



//pub trait EntitySet {

    // fn create<E> (&self, object: E) -> Result(E)
    //     where E: Entity
    // {
    //     false
    // }
    

    // fn read<K, E> (&self, key: K) -> Result(E)
    //     where K: Key,
    //           E: Entity
    // {
    //     // ... do something
    // }

    
    // fn read_list<E: Entity> (&self) -> Result(Vec<E>)
    // {
    //     Err(NOT_IMPL)
    // }
    

    // fn update (&self, raw: Json) -> Result(None)
    // {
    //     false
    // }
    

    // fn delete<K> (&self, key: K) -> Result(None)
    //     where K: Key
    // {
    //     false
    // }

    
    // fn query (&self, params: Vec<QueryOpt>) -> Result(Vec<E>)
    // {
    //     false
    // }
//}
