use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;


pub struct Tracker {
  port: u32,
  ip_address: String,
}

impl Tracker {
  pub fn new (ip: String, port: u32) -> Self {
    Tracker {
      port: port,
      ip_address: ip,
    }
  }

  fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
  }

  fn handle_write(mut stream: TcpStream) {
      let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
      match stream.write(response) {
          Ok(_) => println!("Response sent"),
          Err(e) => println!("Failed sending response: {}", e),
      }
  }

  fn handle_client(stream: TcpStream) {
      Tracker::handle_read(&stream);
      Tracker::handle_write(stream);
  }

  pub fn start(&self) {
      let bind_location = format!("{}:{}", self.ip_address, self.port);
      let listener = TcpListener::bind(bind_location).unwrap();
      println!("Listening for connections on port {}", self.port);
      for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    Tracker::handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
      }
  }
}