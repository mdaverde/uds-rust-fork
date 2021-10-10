#![feature(unix_socket_abstract)]
use std::os::{unix};
use std::io::prelude::*;

fn main() {
    // TODO: move namespace config into param
    let addr = unix::net::SocketAddr::from_abstract_namespace(b"hello").unwrap();
    println!("Starting session on namespace: {:?}", addr);

    let socket = unix::net::UnixListener::bind_addr(&addr).unwrap();
    for (i, stream) in socket.incoming().enumerate() {
        match stream {
            Ok(mut stream) => {
                // std::thread::spawn(|| handle_client(stream));
                let mut received = String::new();
                stream.read_to_string(&mut received).unwrap();
                println!("[received ({})] {}", i, received);
                // TODO: add ability to write from stdin
                if let Ok(()) = stream.write_all(b"hello from server") {
                    stream.flush().unwrap();
                }
            }
            Err(_err) => {
                println!("Not successful");
            }
        }
    }
}
