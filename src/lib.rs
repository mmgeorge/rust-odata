//! Rust oData is library for building [OData][ODATA] REST services in Rust.
//! This library is referenced and utilized by the [ConnectFour][CONNECTFOUR] project in the [game_server][GSERVE] repository for service generation.
//!
//! ## What is oData? 
//! OData (Open Data Protocol) is an ISO/IEC approved, OASIS standard that defines a set of best practices for building and consuming RESTful APIs. OData helps you focus on your business logic while building RESTful APIs without having to worry about the various approaches to define request and response headers, status codes, HTTP methods, URL conventions, media types, payload formats, query options, etc. OData also provides guidance for tracking changes, defining functions/actions for reusable procedures, and sending asynchronous/batch requests. [[1]][ODATA]

//! ## Usage Overview 
//! The library can be used to build REST API based using the [Entity Data Model][EDM]. EntityTypes and EntitySets are declared using macros which provide additional information used to generate metadata and create resource paths. For instance, we can declare a Game EntityType
//! ```rust
//! // Creates a new Game struct and generates serveral methods used internally
//! defEntity!(Game(keys => id) {
//!     id: Int64,
//!     width: Int64,
//!     height: Int64,
//!     k: Int16,
//!     curr_player: Int16,
//!     status: String,
//!     board: String
//! });
//! ```
//! and and EntitySet Games which consists of these types. 
//! ```rust
//! /// Declares Games as an EntitySet containing entities of type Game. Once added to
//! /// the model, it will be reachable via <...>/Games(I) 
//! defEntitySet!(Games, Game);
//! ```
//! Now we can implemented CRUD operations for our sevice via the `EntitySet` trait:
//! ```rust
//!  fn create(&self, value: serde_json::Value) -> Res { ... } 
//!  fn read(&self, key: String) -> Res { ... } 
//!  fn read_list(&self) -> Res { ... } 
//!  fn update ...
//!  fn delete ...
//! ```
//! The EntitySet trait provides a default implementation for these methods, meaning that we only need to directly implement those that we plan on using. Lastly we need to instantiate the `model` using our `EntitySet` (here we only have one), as well as declare the name of our service and start listening for requests. 
//! ```rust
//! let model = ModelBuilder::new("connect_four.svc")
//!     .add(Games::declare())
//!     .build()
//!    
//! let svc = ServiceBuilder::new("api")
//!     .add(model)
//!     .build();
//!
//! svc.start();
//! ```
//! Now our clients can access our API using `<hostname:port>/api/connect_four.svc` to, for example, get a list of games by going to `<hostname:port>/api/connect_four.svc/Games`
//!
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

