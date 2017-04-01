
mod tcp;

fn main() {
    let client = tcp::Client::new("127.0.0.1:8000");
    client.send();
}
