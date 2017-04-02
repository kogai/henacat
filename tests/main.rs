extern crate henacat;

mod client;

use henacat::server;
use std::thread::spawn;

#[test]
fn e2e_should_get_response() {
    spawn(|| {
        let mut server = server::Server::new("127.0.0.1:8000");
        server.listen();
    });
    let client = client::Client::new("127.0.0.1:8000");
    let result = client.send();

    assert!(result.contains("It works!"));
}
