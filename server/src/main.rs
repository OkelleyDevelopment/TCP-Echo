use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::process::exit;
use std::thread;
use std::{env::args, net::Ipv4Addr};
use std::{io::*, net::Shutdown};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <port number>");
        exit(1);
    }

    // Get the port
    let port = args[1].parse::<u16>().unwrap();

    // Establish the socket
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    // Create the listener
    let listener = TcpListener::bind(&socket).unwrap();

    println!(
        "Server is listening on port {}",
        listener.local_addr().unwrap().port()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // socket address of remote peer of the connection
                let peer_address = stream.peer_addr().unwrap();
                println!("New Connection: {}", peer_address);
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed... {}", e);
            }
        }
    }
    drop(listener);
    drop(port);
    drop(socket);

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];

    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "ERROR: Terminating connection with {}",
                stream.peer_addr().unwrap(),
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
