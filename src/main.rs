// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write, Read};
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
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).unwrap();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        headers.push(trimmed.to_string());
        line.clear();
    }
    let content_length = headers.iter()
        .find(|h| h.to_lowercase().starts_with("content-length:"))
        .and_then(|h| h.split(':').nth(1))
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).unwrap();
    let body_str = String::from_utf8(body).unwrap();
    println!("Body: {}", body_str);
    let streq: Vec<&str> = http_request.split_whitespace().collect();
    let mut command = "".to_string();
    let mut path = "".to_string();
    let mut status_line: String = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    if streq[0] == "GET" {
        if streq[1][1..].contains('/') {
            command = streq[1][1..streq[1][1..].find('/').unwrap()+1].to_string();
            path = streq[1][(streq[1][1..].find("/").unwrap() + 2)..].to_string();
        } else {
            command = streq[1][1..].to_string();
        };
        println!("{}/{}", command, path);
        if command == "" {
            status_line = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
        if command == "echo" {
            let content_encoding = headers.iter().find(|header| header.starts_with("Accept-Encoding"));
            status_line = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", path.len(), path);
            if content_encoding.is_some() {
                let encoding = content_encoding.unwrap().to_string().split_whitespace().collect::<Vec<&str>>()[1..].join(" ");
                println!("{}", encoding);
            if encoding == "gzip" {
                // let mut decoder = flate2::read::GzDecoder::new(body_str.as_bytes());
                // let mut decoded = String::new();
                // decoder.read_to_string(&mut decoded).unwrap();
                status_line = format!("HTTP/1.1 200 OK\r\nContent-Encoding: {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", encoding, path.len(), path);
            }}
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
    } else if streq[0] == "POST"{
        if streq[1][1..].contains('/') {
            command = streq[1][1..streq[1][1..].find('/').unwrap()+1].to_string();
            path = streq[1][(streq[1][1..].find("/").unwrap() + 2)..].to_string();
        } else {
            command = streq[1][1..].to_string();
        };
        println!("{}/{}", command, path);
        if command == "files" {
            let env_args: Vec<String> = std::env::args().collect();
            let mut dir = env_args[2].clone();
            dir = dir + "/" + &path;
            let file_path = std::path::Path::new(&dir);
            fs::write(file_path, body_str).expect("Failed to write to file");
            status_line = "HTTP/1.1 201 Created\r\n\r\n".to_string();
        }
    }
    
    stream
        .write_all(status_line.as_bytes())
        .expect("Failed to write to stream");
}

