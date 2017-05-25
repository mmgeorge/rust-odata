//! # Rust oData
//! Rust oData is library for building [oData][ODATA] REST services in Rust.
//! This library is referenced and utilized by the Connect Four repository for client-server
//! communication. 
//! 
//! Resources are declared using the [Entity Data Model][EDM]
//!
//! ## Dependencies
//! This project uses Hyper for it's underlying HTTP server, and because it uses the master branch,
//! (the released version of hyper lacks critical functionality), it references in its dependencies
//! a forked version of that project. This will change once Hyper 1.0 is released. 
//!
//! ## Implementation Status
//! For the project, likely only a subset of oData 4.0 will be implemented, with more advanced
//! features potentially implemented later or for the project if time allows. The focus here is
//! on core functionality.
//! 
//! ### Current status (0-3)
//! 
//! - Entity Class - 3
//! - Entity Set Class - 3
//! - Edm Types - 3
//! - Service Class - 2
//!     - Model registration & hookup
//!     - Validation
//!     - Create request
//!     - Read request (by ID, by property, by value)
//!     - Update request
//!     - Delete request
//!     - Error codes
//! - Metadata document - 3
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
//! [ODATA]: http://www.odata.org
//! [EDM]: https://msdn.microsoft.com/en-us/library/ee382825(v=vs.110).aspx

#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use] extern crate lazy_static;
#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate serde_json;
pub extern crate serde;

extern crate time;
extern crate hyper;
    
#[macro_use] pub mod edm;
#[macro_use] pub mod entity;
#[macro_use] pub mod model;
pub mod service;
pub mod test;

