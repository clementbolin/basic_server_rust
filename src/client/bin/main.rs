use std::net::TcpStream;
use std::io::{Write, Read, stdin};

fn get_entry() -> String {
    let mut buf = String::new();

    match stdin().read_line(&mut buf) {
        Ok(_) => {}
        Err(e) => { println!("{}", e); }
    }
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_server(mut stream: TcpStream) {
    let stdoutt = std::io::stdout();
    let mut io = stdoutt.lock();
    let buf = &mut [0; 3];

    println!("Tap Quit for leave");
    loop {
        match write!(io, "> ") {
            Ok(_) => {}
            Err(e) => { println!("{}", e); }
        };
        match io.flush() {
            Ok(_) => {}
            Err(e) => { println!("{}", e); }
        };
        match &*get_entry() {
            "Quit" => {
                println!("leave server");
                return;
            }
            input => {
                match write!(stream, "{}\n", input) {
                    Ok(_) => {}
                    Err(e) => { println!("{}", e); }
                };
                match stream.read(buf) {
                    Ok(res) => {
                        if res < 1 {
                            println!("Connection with server lost");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Connection with server lost");
                        return;
                    }
                }
                println!("Server response: {:?}", buf);
            }
        }
    }
}

fn main() {
    println!("Try to connect in Server...");
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(stream) => {
            println!("successful login !");
            exchange_server(stream);
        }
        Err(e) => {
            println!("Connection failed ! {}", e);
        }
    };
}
