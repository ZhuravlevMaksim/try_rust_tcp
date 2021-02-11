use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{fs, thread};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    let contents= if buffer.starts_with( b"GET / HTTP/1.1\r\n") {
        format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Header</title></head><body><pre>{}</pre></body></html>", request)
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned()
    };

    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ).to_owned();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}