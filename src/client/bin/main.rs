use std::net::TcpStream;

fn main() {
    println!("Try to connect in Server...");
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(_) => {
            println!("successful login !");
        }
        Err(e) => {
            println!("Connection failed ! {}", e);
        }
    };
}
