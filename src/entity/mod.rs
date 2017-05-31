//! This module includes core functionality needed for declaring EntityTypes
//! and EntitySets


#[macro_use] mod entity;
#[macro_use] mod entity_set;
mod property;

pub use self::entity::Entity;
pub use self::entity::EntityDescr;
pub use self::entity_set::EntitySet; 
pub use self::entity_set::EntitySetDescr; 
pub use self::property::Property;

