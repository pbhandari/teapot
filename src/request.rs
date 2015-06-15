#[derive(Debug)]
enum RequestType {
    GET,
    POST,
    INVALID,
}

#[derive(Debug)]
struct HttpVersion {
    major : u32,
    minor : u32,
}

pub struct Request {
    file_name : String,
    rtype     : RequestType,
    version   : HttpVersion,
}

impl Request {
    pub fn new(line : String) -> Request {
        let mut token = line.split(" ");

        let request_type = match token.next() {
            Some("GET")  => { RequestType::GET },
            Some("POST") => { RequestType::POST },
            Some(_)      => { RequestType::INVALID },
            None         => { RequestType::INVALID },
        };

        let file = token.next().unwrap().to_string();
        let http_version = match token.next() {
            Some("HTTP/1.0") => { HttpVersion { major: 1, minor: 0 } },
            Some("HTTP/1.1") => { HttpVersion { major: 1, minor: 1 } },
            Some("HTTP/2.0") => { HttpVersion { major: 1, minor: 1 } },
            _                => { HttpVersion { major: 1, minor: 1 } },
        };

        Request {
            rtype     : request_type,
            file_name : file,
            version   : http_version,
        }
    }

    pub fn status(&mut self) -> u32 {
        200
    }

    pub fn get_file(&mut self) -> String {
        self.file_name.clone()
    }
}
