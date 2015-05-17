mod server;
use server::Server;

fn main() {
    Server::new("127.0.0.1:4000")
           .expect("Could not connect to Server")
           .serve();
}
