use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::cli::ServerArgs;

pub fn start_server(server_args: ServerArgs) -> std::io::Result<()> {
    let listen_addr = create_listen_addr(server_args);
    let listener = TcpListener::bind(&listen_addr)?;

    println!("Starting IRC server at listen address: {}", &listen_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr()?);
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn create_listen_addr(server_args: ServerArgs) -> String {
    let address = match server_args.address {
        Some(a) => a,
        None => "192.168.1.232".to_string(),
    };

    let port = match server_args.port {
        Some(p) => p,
        None => "8080".to_string(),
    };

    format!("{address}:{port}")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 2048];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => return, // Connection closed
            Ok(n) => {
                println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        }
    }
}
