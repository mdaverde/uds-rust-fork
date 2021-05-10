#![feature(unix_socket_abstract)]
use std::os::unix;
use std::io::prelude::*;

fn main() {
    let addr = unix::net::SocketAddr::from_abstract_namespace(b"hello").unwrap();
    println!("{:?}", addr);
    let socket = unix::net::UnixListener::bind_addr(&addr).unwrap();
    for stream in socket.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut response = String::new();
                stream.read_to_string(&mut response).unwrap();
                println!("Connection successful: {}", response);
            }
            Err(_err) => {
                println!("Not successful");
            }
        }
    }
}
