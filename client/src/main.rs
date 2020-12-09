use std::net::TcpStream;
use std::process::exit;
use std::str::from_utf8;
use std::{env::args, net::Ipv4Addr};
use std::{
    io::{Read, Write},
    print,
};

pub fn main() {
    // Collect the command line args
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <address> <port>");
        exit(1);
    }

    let ip = &args[1];

    let port = &args[2];

    let address = String::from(ip) + &String::from(":") + &String::from(port);

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Successfully connected to server on port {}", port);

            let to_sender = b"Start";

            stream.write(to_sender).unwrap();
            println!("First transmission sent. Waiting...");

            let mut buffer = [0 as u8; 5]; // 22 byte buffer

            while to_sender != b":exit" {
                match stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        if &buffer == to_sender {
                            println!("Rcvd: Repeated Transmission");
                        } else {
                            let rcvd = from_utf8(&buffer).unwrap();
                            println!("Rcvd: {}", rcvd);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Session closed.");
}
