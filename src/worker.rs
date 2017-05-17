//! # Worker module
//! service spawns a thread that creates a new Worker to litsen to the TCP
//! socket and handle the request.

use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::{File};
use std::sync::mpsc::SyncSender;
use chrono::{DateTime, UTC};


/// Parse the passed string as a path, seeing if resource exists
fn parse_path(pname: &str) -> Result<String, u16> { 
    Err(0)
}


/// An internal helper class that encapsulates the data required for each
/// worker thread. 
pub struct Worker {
    stream: TcpStream,
    addr: SocketAddr,
    name: String,
    log_handler: SyncSender<String>,
    birth: DateTime<UTC>,
}


impl Worker {
    pub fn new (stream: TcpStream, addr: SocketAddr,
                name: String,
                log_handler: SyncSender<String>)
                -> Self {
        Worker {
            stream: stream,
            addr: addr,
            name: name,
            log_handler: log_handler,
            birth: UTC::now()
        }
    }

    
    /// Instruct the worker to begin processing the TCPStream
    /// passed to it during construction. 
    pub fn handle_request(&mut self) {
        // Parse the request and attempt to extract the path
        // if it is valid. Otherwise pass through errors. 
        let maybe_path = match self.parse_request() {
            Ok(pname) => parse_path(&pname),
            Err(e) => Err(e),
        };

        // See if we were able to parse the path, if so,
        // serve the file and log the request. 
        match maybe_path {
            Ok(path) => {
                self.log_request(&path);
            },
            Err(e) => self.handle_error(e)
        }
    }


    /// Log the result of a successful query by exclusively locking
    /// the target file and modifying it. If another thread has already
    /// acquire the lock, the thread spins.
    /// 
    /// The log takes the form of a csv with:
    ///     timestamp: time of request
    ///            ip: ip address SocketAddr
    ///          port: port of the SocketAddr
    ///          type: type of request (always GET here)
    ///          path: the path of the requested file
    fn log_request(&self, path: &str) {
        let entry = format!("{},{},{},GET,{}\n",
                            self.birth.format("%F %T"),
                            self.addr.ip(),
                            self.addr.port(),
                            path);
        self.log_handler.send(entry).unwrap();
    }


    /// Parse a given request, validating the the request is valid.
    /// Returns either a path string or a BAD REQUEST error
    fn parse_request(&mut self) -> Result<String, u16> {
        let stream = BufReader::new(&mut self.stream);
        let line = stream.lines().next().unwrap().unwrap();
        let tokens: Vec<&str> = line.split(' ').collect();

        // Verify header
        if  (tokens.len() != 3) ||
            (tokens[0] != "GET") ||
            !((tokens[2] == "HTTP") || (tokens[2] == "HTTP/1.1")) {
                return Err(0); 
            }
        
        Ok(String::from(tokens[1]))
    }


    /// Handle errors with a response to the client.
    fn handle_error(&mut self, code: u16) {
        let mut response = String::from("HTTP/1.0 ");
        match code {
            // BAD_REQUEST => response += "400 Bad Request\n",
            // FORBIDDEN   => response += "403 Forbidden\n",
            // NOT_FOUND   => response += "404 Not Found\n",
            _           => panic!("Cannot handle unknown error!")
        }
        self.stream.write(response.as_bytes()).unwrap();
    }
}

