use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::net::Shutdown;
use std::io::{Write, Read};
use std::vec::Vec;

const CR   : u8 = '\r' as u8;
const LF   : u8 = '\n' as u8;
const CRLF : [u8; 2] = [CR, LF];

pub struct Server {
    listener : TcpListener,
}

impl Server {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Option<Server> {
        match TcpListener::bind(addr) {
            Ok(list) => { Some(Server { listener : list }) }
            Err(_e)  => { None }//TODO
        }
    }

    pub fn serve(&self) {
        for conn in self.listener.incoming() {
            match conn {
                Ok(mut stream) => {
                    let headers = self.get_headers(&stream);
                    let last_conn =  headers[0] == "GET /exit HTTP/1.1";

                    self.respond(&mut stream, headers);
                    stream.shutdown(Shutdown::Both)
                          .ok().expect("Failed to close stream");

                    if last_conn { break; }
                }
                Err(_e) => {}
            }
        }

    }

    fn read_line(&self, stream: &TcpStream) -> String {
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

    fn get_headers(&self, stream: &TcpStream) -> Vec<String> {
        let mut query : Vec<String> = Vec::with_capacity(10);

        loop {
            let line = self.read_line(stream);

            if line.is_empty() { break; }

            query.push(line);
        }

        query.shrink_to_fit();
        query
    }

    fn write(&self, stream: &mut TcpStream, response: &str) {
        stream.write(response.as_bytes()).ok();
        stream.write(&CRLF).ok();
    }

    // TODO: take in headers and parse it
    fn respond(&self, stream: &mut TcpStream, headers: Vec<String>) {
        for head in headers {
            println!("HEAD: {}", head);
        }

        self.write(stream, "HTTP/1.1 418 I'm a teapot");
        self.write(stream, "Connection: close");
        self.write(stream, "");
        self.write(stream, "I'm a teapot");

        stream.flush().ok();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("dropping");
        drop(&self.listener)
    }
}
