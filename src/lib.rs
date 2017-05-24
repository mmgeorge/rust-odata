//! # Rust oData
//! Rust oData is library for building [oData][ODATA] REST services in Rust.
//! This library is referenced and utilized by the Connect Four repository for client-server
//! communication. 
//! 
//! Resources are declared using the [Entity Data Model][EDM]
//!
//! ## Implementation Status
//! For the project, likely only a subset of oData 4.0 will be implemented, with more advanced
//! features potentially implemented later or for the project if time allows. The focus here is
//! on core functionality.
//! 
//! ### Current status (0-3)
//! 
//! - Entity Class - 1
//! - Entity Set Class - 2
//! - Edm Types - 2
//! - Service Class - 0
//!     - Model registration & hookup
//!     - Validation
//!     - Create request
//!     - Read request (by ID, by property, by value)
//!     - Update request
//!     - Delete request
//!     - Error codes
//! - Metadata document - 0
//! - Relationship Operations - 0
//! - Singleton requests - 0
//! - Singleton property requests -0
//! - Contained Entities - 0
//! 
//! 
//! ### Potential additional features
//!
//! - Query & parameters
//!     - filter
//!     - expand 
//!     - top 
//!     - limit
//!     - count
//!     - select
//!     - search
//!     - lambda operators
//! - Functions/actions
//! - Etag modifcation/delete requests
//! - Derived entity types
//! - Open Entity/Complex Types
//! - Batch requests
//!
//! ## Interface
//! The original idea was to have users declare derived classes of entity/entity set
//! respectively, but because Rust does not have C++ style inheritence, we are currently
//! in the process of rethinking this.
//!
//! One idea is to instead use the Rust builder paradigm to specify entity types, ie.
//!
//! ```
//! ```
//!
//! where each .add() creates a new property for the specified Entity. This could also
//! be expanded to include a validation policies:
//!
//! ```
//! ```
//! 
//! EntitySet could then be a trait, with a `declare` function used to specify the entity
//! type to which it refers. CRUD-Q methods would then optionally be defined, with
//! unimplemented stubs defaulting to a function which returns false denoting no
//! implementation (perhaps this should instead return an error type? This would allow stubs
//! to specify some kind of error and return it). 
//!
//! ```
//! 
//!
//! ```
//!
//! This is all very tenative though! Currently exploring macros as a way to potentially
//! improve interface. Right now specifiy properties in this fashion appears to require
//! the use of a `HashMap`. This is less than ideal as we give up quite a bit of performance
//! than we would otherwise have by using native Rust structs.
//!
//! [ODATA]: http://www.odata.org
//! [EDM]: https://msdn.microsoft.com/en-us/library/ee382825(v=vs.110).aspx

#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate crossbeam;
extern crate chrono;
extern crate hyper;
    
#[macro_use]
pub mod edm;

#[macro_use]
pub mod entity;

#[macro_use]
pub mod model;
pub mod service;
pub mod test;

