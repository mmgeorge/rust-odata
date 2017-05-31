#### This repository is outdated and maintains an old version of this project for compatibility with the [Connect Four][CONNECTFOUR] library which depends on it. As major revisions are planned, future development [has been moved](https://github.com/mmgeorge/odata-rs). 

# rust-odata
Rust oData is library for building [OData][ODATA] REST services in Rust.
This library is referenced and utilized by the [ConnectFour][CONNECTFOUR] project in the [game_server][GSERVE] repository for service generation.

The online documentation for rust-odata can be found [here][DOC]. 

## What is oData? 
OData (Open Data Protocol) is an ISO/IEC approved, OASIS standard that defines a set of best practices for building and consuming RESTful APIs. OData helps you focus on your business logic while building RESTful APIs without having to worry about the various approaches to define request and response headers, status codes, HTTP methods, URL conventions, media types, payload formats, query options, etc. OData also provides guidance for tracking changes, defining functions/actions for reusable procedures, and sending asynchronous/batch requests. [[1]][ODATA]

[GSERVE]: https://github.com/mmgeorge/game_server
[ODATA]: http://www.odata.org
[DOC]: https://mmgeorge.github.io/rust-odata/rust_odata/
[CONNECTFOUR]: https://github.com/eecs395rust/ConnectFour
