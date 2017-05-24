

use entity::{Entity, EntityDescr, Property};
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

    fn create<E> (&self, _object: E) -> Result<E, SvcError>
        where E: Entity
    {
        unimplemented!();
    }
    

    fn read<E> (&self, _key: Property) -> Result<E, SvcError>
        where E: Entity
    {
        unimplemented!();
    }

    
    fn read_list<E: Entity> (&self) -> Result<Vec<E>, SvcError>
    {
        unimplemented!();
    }
    

    fn update<E> (&self, _json: Value) -> Result<E, SvcError>
        where E: Entity
    {
        unimplemented!();
    }
    

    fn delete<E> (&self, _key: Property) -> Result<E, SvcError>
        where E: Entity
    {
        unimplemented!();
    }

    
    // fn query (&self, params: Vec<QueryOpt>) -> Result<Vec<E>, SvcError>
    // {
    //     unimplemented!();
    // }
}
