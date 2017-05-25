
use std::clone::Clone;

use entity::{Entity, EntityDescr, Property};
use service::{Error, Res};

use serde_json::Value;


/// defEntitySet!(name: ident, entity_type: ident)
/// Defines a macro for declaring EntitySets. For instance, 
/// ```
/// defEntitySet!(Dogs, Dog); 
/// ```
/// Declares an EntitySet named "Dogs" that consistes of elements
/// of Entity Dog. CRUD-Q operations are then exposed via implementation of the
/// EntitySet trait for struct. 
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

        impl Clone for $name {
            fn clone (&self) -> $name
            {
                $name {}
            }
        }
    }
} 

/// Internal descriptor for an EntitySet for usage by the Model. Provides access
/// to the underlying desciptor for the EntityType.
pub trait EntitySetDescr {
    fn name(&self) -> String;
    fn descriptor(&self) -> EntityDescr; 
}


#[derive(Debug)]
pub enum SvcError {
    NoImplementation
}


/// Trait for declaring CRUD-Q implementation. 
pub trait EntitySet {

    fn create (&self, _value: Value) -> Res
    {
        unimplemented!();
    }
    

    fn read (&self, _key: String) -> Res
    {
        unimplemented!();
    }

    
    fn read_list (&self) -> Res
    {
        Res::Succ(Some(json!(null)))
    }
    

    // fn update<E> (&self, _json: Value) -> Result<E, SvcError>
    //     where E: Entity
    // {
    //     unimplemented!();
    // }
    

    // fn delete<E> (&self, _key: Property) -> Result<E, SvcError>
    //     where E: Entity
    // {
    //     unimplemented!();
    // }

    
    // fn query (&self, params: Vec<QueryOpt>) -> Result<Vec<E>, SvcError>
    // where Self: Sized
    // {
    //     unimplemented!();
    // }
}

// impl Clone for Box<EntitySet> {
//     fn clone(&self) -> Box<EntitySet> {
//         self.clone()
//     }
// }

// impl Clone for Box<EntitySetDescr> {
//     fn clone(&self) -> Box<EntitySetDescr> {
//         self.clone()
//     }
// }



// impl<T: ?Sized> EntitySet for Box<T>
//     where T: EntitySet
// {
//     fn read_list<E: Entity + ?Sized> (&self) -> Res
//         where Self: Sized
//     {
//         (**self).read_list::<E>()
//     }
// }
