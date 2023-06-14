use std::net::{TcpListener, TcpStream};
use std::io::{self};
use std::io::prelude::*;
use universal_socket2::green_message;

fn main() -> io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379")?;
    
    let msg1: String = green_message("Server is running!");
    let msg2: String = green_message("------------------");

    println!("{msg1}");
    println!("{msg2}");
    loop {
        let (socket, _) = listener.accept()?;
        read_message(socket);
    }
}

fn read_message(mut socket: TcpStream) {
    let mut buf: [u8; 4096] = [0; 4096];
    match socket.read(&mut buf) {
        Ok(0) => {
            eprintln!("Buffer is empty!");
        }
        Ok(_size) => {
            match std::str::from_utf8(&buf) {
                Ok(msg) => {
                    println!("{msg}")
                }
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
        }

    }
}
