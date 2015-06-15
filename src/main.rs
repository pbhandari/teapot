mod connection;
mod request;
mod server;

fn main() {
    server::Server::new("127.0.0.1:4000")
                    .expect("Could not connect to Server")
                    .serve();
}
