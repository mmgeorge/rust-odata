

use std::net::TcpListener;
use std::io::{Write, BufWriter};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::thread;

use worker::Worker;

pub struct Service {
    name: String, 
    listener: TcpListener,
    log_path: String,
}


impl Service {
    /// Create a new oData service. Uses a TcpListener for underlying
    /// socket represetnation.
    pub fn new (name: &str, port: u32) -> Self {
        Service {
            name: String::from(name),
            log_path: String::from("log.csv"),
            listener: TcpListener::bind(String::from("0.0.0.0:")
                                        + &port.to_string()).unwrap(),
        }
    }

    
    /// Creates a BufWriter to the log file. Creates the file if it does
    /// not exist.
    fn create_log(&self) -> BufWriter<File> {
        // Create/overwrite the log file if it does not exist
        if !Path::new(&self.log_path).exists() {
            let header = "timestamp,ip,port,type,path\n";
            let log = OpenOptions::new()
                .create(true).truncate(true).write(true)
                .open(&self.log_path)
                .unwrap();
            let mut writer = BufWriter::new(log);
            writer.write(header.as_bytes()).unwrap();
        }

        let log = OpenOptions::new()
            .write(true).append(true)
            .open(&self.log_path)
            .unwrap();
        let writer = BufWriter::new(log);
        writer
    }

    
    /// Creates a dedicated logging thread and returns its handler
    fn start_logging(&mut self) -> SyncSender<String> {
        let mut log = self.create_log();

        // Create a channel with a buffer of size 10
        // Senders block when buffer is full.
        let (tx, rx) = sync_channel::<String>(10);
        thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                log.write(msg.as_bytes()).unwrap();
                log.flush().unwrap();
            }
        });

        tx
    }


    /// Spawn a worker thread to process the request
    pub fn serve(&mut self) {
        // Create log file with header
        // Start dedicated logging handler
        let log_handler = self.start_logging(); 
        
        // Loop indefinately spawning worker threads to handle requests
        loop {
            let (stream, addr) = self.listener.accept().unwrap();
            let mut worker = Worker::new(
                stream, addr,
                self.name.clone(),
                log_handler.clone(),
            );
            thread::spawn(move || { worker.handle_request(); });
        }
    }
}


