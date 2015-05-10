use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::vec::Vec;

const CR   : u8 = '\r' as u8;
const LF   : u8 = '\n' as u8;
const CRLF : [u8; 2] = [CR, LF];

fn read_line(stream: &TcpStream) -> String {
    // an initial capacity of 64 should be enough for most cases to not
    // trigger a resize
    let mut line = String::with_capacity(64);

    for byte in stream.bytes() {
        match byte {
            Ok(b) => {
                if b == LF { break; }

                line.push(b as char);
            },
            Err(_e) => {
                break; // FIXME: handle error
            }
        };
    }

    // remove the trailing CR
    match line.pop() {
        // and put it back in if it's not CR
        Some(c) if c as u8 != CR => { line.push(c); }
        _ => {}
    }

    line.shrink_to_fit();
    line
}

fn get_headers(stream: &TcpStream) -> Vec<String> {
    let mut query : Vec<String> = Vec::with_capacity(10);

    loop {
        let line = read_line(stream);

        if line.is_empty() { break; }

        query.push(line);
    }

    query.shrink_to_fit();
    query
}

fn write(stream: &mut TcpStream, response: &str) {
    stream.write(response.as_bytes()).ok();
    stream.write(&CRLF).ok();
}

// TODO: take in headers and parse it
fn respond(stream: &mut TcpStream, headers: Vec<String>) {
    for head in headers {
        println!("HEAD: {}", head);
    }

    write(stream, "HTTP/1.1 418 I'm a teapot");
    write(stream, "Connection: close");
    write(stream, "");
    write(stream, "I'm a teapot");

    stream.flush().ok();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    for conn in listener.incoming() {
        match conn {
            Ok(mut stream) => {
                let headers = get_headers(&stream);
                respond(&mut stream, headers);
                stream.shutdown(std::net::Shutdown::Both)
                      .ok().expect("Failed to close stream");
            }
            Err(_e) => {}
        }
    }
    drop(listener);
}
