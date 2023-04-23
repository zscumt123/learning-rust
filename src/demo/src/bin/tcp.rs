use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").unwrap();

    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        thread::spawn(move || {
            let mut buffer = [0u8; 12];
            stream.read_exact(&mut buffer).unwrap();
            println!("data: {:?}", String::from_utf8_lossy(&buffer));
            stream.write_all(b"glad to meet you!").unwrap();
        });
    }
}
