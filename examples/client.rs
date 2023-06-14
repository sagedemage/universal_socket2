use std::net::{TcpStream};
use std::io::{self};
use std::io::prelude::*;
use universal_socket2::{green_message, red_message};
use std::env;

const SERVER_IPV4_ADDRESS:&str = "127.0.0.1";
const SERVER_PORT: &str = "6379";

fn main() -> io::Result<()> {
    let msg: String = String::from("Hello");

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let buf: &[u8] = msg.as_bytes();
            send_message_to_server(buf)?;
        }
        2 => {
            let option = &args[1];

            if option == "i" {
                let msg: String = red_message("Missing input!");
                eprintln!("{msg}");
            }
            else {
                let msg: String = red_message("Option does not exist!");
                eprintln!("{msg}");
            }
        }
        3 => {
            let option: &String = &args[1];
            let input: &String = &args[2];

            if option == "i" {
                if input == "" {
                    let msg: String = red_message("Input is empty!");
                    eprintln!("{msg}");
                }
                else {
                    let msg: &[u8] = input.as_bytes();
                    send_message_to_server(msg)?;
                }
            }
            else {
                let msg: String = red_message("Option does not exist!");
                eprintln!("{msg}");
            }
        }
        _ => {}
    }

    Ok(())
}

fn send_message_to_server(message: &[u8]) -> io::Result<()> {
    //! Sends a message to the 
    let server_addr = SERVER_IPV4_ADDRESS.to_owned() + ":" + SERVER_PORT;
    let mut client = TcpStream::connect(server_addr)?;

    client.write(message)?;

    let msg: String = green_message("Sent a message to the server!");
    println!("{msg}");

    Ok(())
}
