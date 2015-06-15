use std::net::{TcpListener, ToSocketAddrs, Shutdown};
use connection::Connection;

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
                Ok(mut stream) => { Connection::new(&mut stream).handle(); }
                Err(error)     => { panic!("{}", error); }
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        drop(&self.listener)
    }
}
