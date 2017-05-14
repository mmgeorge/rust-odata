

use std::net::TcpListener;
use entity::Entity;


// Success Codes
// const OK: u16 = 200;
// const CREATED: u16 = 201;
// const ACCEPTED: u16 = 202;
// const NO_CONTENT: u16 = 204;
// const NOT_MODIFIED: u16 = 304;

// // Error Codes
// const BAD_REQUEST: u16 = 400;
// const FORBIDDEN: u16 = 403; 
// const NOT_FOUND: u16 = 404;
// const METHOD_NOT_ALLOWED: u16 = 405;
// const GONE: u16 = 410;
// const PRECONDITION_FAILEd: u16 = 412;

// const NOT_IMPL: u16 = 501;



pub struct Service {
    listener: TcpListener,
//    sets: Vec<Entity>
}


// fn handle_error(&mut self, code: u16) {
//     let mut response = String::from("HTTP/1.0 ");
//     match code {
//         BAD_REQUEST => response += "400 Bad Request\n",
//         FORBIDDEN   => response += "403 Forbidden\n",
//         NOT_FOUND   => response += "404 Not Found\n",
//         _           => panic!("Cannot handle unknown error!")
//     }
//     self.stream.write(response.as_bytes()).unwrap();
// }


