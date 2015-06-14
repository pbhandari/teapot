use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};

const CR   : u8 = '\r' as u8;
const LF   : u8 = '\n' as u8;
const CRLF : [u8; 2] = [CR, LF];

pub struct Request<'a> {
    stream : &'a TcpStream
}

impl<'a> Request<'a> {
    pub fn new(tcp_stream: &'a TcpStream) -> Request<'a> {
        Request { stream : tcp_stream }
    }

    // TODO: take in headers and parse it
    pub fn respond(&mut self) {
        for head in self.get_headers() {
            println!("HEAD: {}", head);
        }

        self.write("HTTP/1.1 418 I'm a teapot");
        self.write("Connection: close");
        self.write("");
        self.write("I'm a teapot");

        self.stream.flush().ok();
    }

    fn write(&mut self, response: &str) {
        self.stream.write(response.as_bytes()).ok();
        self.stream.write(&CRLF).ok();
    }

    fn read_line(&self) -> String {
        // an initial capacity of 64 should be enough for most cases to not
        // trigger a resize
        let mut line = String::with_capacity(64);

        for byte in self.stream.bytes() {
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

    fn get_headers(&self) -> Vec<String> {
        let mut query : Vec<String> = Vec::with_capacity(10);

        loop {
            let line = self.read_line();

            if line.is_empty() { break; }

            query.push(line);
        }

        query.shrink_to_fit();
        query
    }
}

impl<'a> Drop for Request<'a> {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both)
            .ok().expect("Failed to close stream");
    }
}
