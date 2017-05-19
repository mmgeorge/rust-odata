
use entity::Entity;
use model::Model;

//pub trait Ke // Property type ?

//type Json = serde_json::Value;
//

#[macro_export]
macro_rules! defEntitySet {
    ($name:ident, $entity:ident) => {
        struct $name {
            data: Vec<$entity>
        }

        impl $name {
            fn describe() -> (String, Vec<Property>) {
                (String::from(stringify!($name)), $entity::describe())
            }
        }
    }
} 


pub trait EntitySet {
    
}


type QueryOpt = String;


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
