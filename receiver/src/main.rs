use std::{net::TcpListener, io::Read};

fn main() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        
        let mut buffer = [0; 34];
        stream.read(&mut buffer).unwrap();
        println!("{:?}", String::from_utf8_lossy(&buffer[..]));
        let id = String::from_utf8_lossy(&buffer[..]);
        println!("Id: {}", id)
    }
}