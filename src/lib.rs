//! Rust oData is library for building [OData][ODATA] REST services in Rust.
//! This library is referenced and utilized by the [ConnectFour][CONNECTFOUR] project in the [game_server][GSERVE] repository for service generation.
//!
//! ## What is oData? 
//! OData (Open Data Protocol) is an ISO/IEC approved, OASIS standard that defines a set of best practices for building and consuming RESTful APIs. OData helps you focus on your business logic while building RESTful APIs without having to worry about the various approaches to define request and response headers, status codes, HTTP methods, URL conventions, media types, payload formats, query options, etc. OData also provides guidance for tracking changes, defining functions/actions for reusable procedures, and sending asynchronous/batch requests. [[1]][ODATA]

//! ## Usage Overview 
//! The library can be used to build REST API based using the [Entity Data Model][EDM]. EntityTypes and EntitySets are declared using macros which provide additional information used to generate metadata and create resource paths. See README for sample usage.

//! [EDM]: https://msdn.microsoft.com/en-us/library/ee382825(v=vs.110).aspx
//! [GSERVE]: https://github.com/mmgeorge/game_server
//! [ODATA]: http://www.odata.org
//! [DOC]: https://mmgeorge.github.io/rust-odata/rust_odata/
//! [CONNECTFOUR]: https://github.com/eecs395rust/ConnectFour

// Because we provide macros and intentially unimplemented function bodies,
// we get all kinds of warnings. Should move these directly to where they are required. 
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

