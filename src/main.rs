// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

pub fn handle_connection (mut stream: std::net::TcpStream) {
    let reader = BufReader::new(&mut stream);
    let http_request = reader.lines().next().unwrap().unwrap();
    let status_line = match &http_request[..] {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
    };
    stream
        .write_all(status_line.as_bytes())
        .expect("Failed to write to stream");
}

