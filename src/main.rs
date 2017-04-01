mod server;
mod client;

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let client = client::Client::new("127.0.0.1:8000");
        client.send();
    });
    let mut server = server::Server::new("127.0.0.1:8000");
    server.listen();
}

