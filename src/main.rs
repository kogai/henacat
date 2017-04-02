extern crate chrono;

mod server;
mod method;
mod content_type;
mod status;

fn main() {
    let mut server = server::Server::new("127.0.0.1:8000");
    server.listen();
}

