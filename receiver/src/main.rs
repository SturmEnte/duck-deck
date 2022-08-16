use std::{net::TcpListener, io::Read};
use enigo::{Enigo, Key, KeyboardControllable};

fn main() {
    let mut enigo = Enigo::new();
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        
        let mut buffer = [0; 34];
        let size = stream.read(&mut buffer).unwrap();
        let id = String::from_utf8_lossy(&buffer[..size]);
        println!("Id: {}", id);

        if id == "1" {
            enigo.key_click(Key::Layout('^'));
            println!("Key click");
        } else {
            println!("Not 1");
        }
    }
}