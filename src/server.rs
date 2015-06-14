use std::net::{TcpListener, ToSocketAddrs};
use request::Request;

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
                Ok(stream) => { Request::new(&stream).respond(); }
                Err(_e) => {}
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        drop(&self.listener)
    }
}
