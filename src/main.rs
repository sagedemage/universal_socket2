use std::net::{TcpListener, TcpStream};
use std::io::{self};
use std::io::prelude::*;
use universal_socket2::green_message;

//const IPV4_ADDRESS:&str = "127.0.0.1";
const IPV4_ADDRESS:&str = "192.168.1.72";
const PORT: &str = "6379";

fn main() -> io::Result<()> {
    let addr: String = IPV4_ADDRESS.to_owned() + ":" + PORT;
    let listener: TcpListener = TcpListener::bind(addr)?;
    
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
