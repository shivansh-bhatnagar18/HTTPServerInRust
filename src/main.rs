// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};
use std::fs;

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

pub fn handle_connection(mut stream: std::net::TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut http_request = String::new();
    reader.read_line(&mut http_request).unwrap();
    let mut headers = Vec::new();
    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        headers.push(line);
    }
    let streq: Vec<&str> = http_request.split_whitespace().collect();
    let mut command = "".to_string();
    let mut path = "".to_string();
    if streq[1][1..].contains('/') {
        command = streq[1][1..streq[1][1..].find('/').unwrap()+1].to_string();
        path = streq[1][(streq[1][1..].find("/").unwrap() + 2)..].to_string();
    } else {
        command = streq[1][1..].to_string();
    };
    println!("{}/{}", command, path);
    let mut status_line: String = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    if command == "" {
        status_line = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    }
    if command == "echo" {
        status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", path.len(), path);
    }
    if command == "user-agent" {
        let user_agent = headers.iter().find(|header| header.starts_with("User-Agent"));
        let agent = user_agent.unwrap().to_string().split_whitespace().collect::<Vec<&str>>()[1..].join(" ");
        println!("{}", agent);
        status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", agent.len(), agent);
    }
    if command == "files" {
        let env_args: Vec<String> = std::env::args().collect();
        let mut dir = env_args[2].clone();
        dir.push_str(&path);
        let file = fs::read(dir);
        match file {
            Ok(fc) => {
                status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), String::from_utf8(fc).expect("file content"));
            }
            Err(_) => {
                status_line = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
            }
        }
    }
    stream
        .write_all(status_line.as_bytes())
        .expect("Failed to write to stream");
}

