use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:1234").unwrap();

    println!("wait client...");
    match listener.accept() {
        Ok((_client, addr)) => {
            println!("New client (ip: {})", addr);
        }
        _ => {
            println!("A client tried to connect...");
        }
    };
}
