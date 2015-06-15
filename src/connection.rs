use std::net::{TcpStream, Shutdown};
use std::io::{Write, BufRead, BufReader};
use std::collections::HashMap;
use request::Request;

const CR   : u8 = '\r' as u8;
const LF   : u8 = '\n' as u8;
const CRLF : [u8; 2] = [CR, LF];

pub struct Connection<'a> {
    stream  : &'a TcpStream,
    reader  : BufReader<&'a TcpStream>,
    request : Request,
}

impl<'a> Connection<'a> {
    pub fn new(tcp_stream: &'a TcpStream) -> Connection<'a> {
        let mut buf_reader = BufReader::new(tcp_stream);

        let mut line : String = String::new();
        buf_reader.read_line(&mut line).ok();

        let http_request = Request::new(line);

        Connection {
            stream  : tcp_stream,
            reader  : buf_reader,
            request : http_request,
        }
    }

    // TODO: take in headers and parse it
    pub fn handle(&mut self) {
        let headers = self.get_headers();
        let status = self.request.status();

        self.write_headers(status, headers);

        match status {
            200 => {
                let file = self.request.get_file();
                self.write_file(file)
            },
            _ => { self.write_line("I'm a teapot"); },
        }

        self.stream.flush().ok();
    }

    fn write_headers(&mut self, status: u32, headers: HashMap<String, String>) {
        match status {
            200 => { self.write_line("HTTP/1.1 200 OK") },
            _   => { self.write_line("HTTP/1.1 418 I'm a teapot"); }
        }

        self.write_line("Connection: close");
        self.write_line("");
    }

    fn write_line(&mut self, message: &str) {
        self.stream.write(message.as_bytes()).ok();
        self.stream.write(&CRLF).ok();
    }

    fn write_file(&mut self, file_name : String) {
        self.stream.write(&CRLF).ok();
    }

    fn get_headers(&mut self) -> HashMap<String, String> {
        let mut query : HashMap<String, String> = HashMap::with_capacity(10);

        for ln in (&mut self.reader).lines() {
            match ln {
                Ok(ref line) if line.as_bytes()[0] == CR => { break; },
                Ok(line) => {
                    let pair : Vec<&str> = line.split(":").collect();
                    query.insert(pair[0].to_string(), pair[1].to_string());
                }
                Err(error) => { panic!("{}", error); }
            }
        }

        query.shrink_to_fit();
        query
    }
}

impl<'a> Drop for Connection<'a> {
    fn drop(&mut self) {
        println!("Dropping");
        self.stream.shutdown(Shutdown::Both)
            .ok().expect("Failed to close stream");
    }
}
