use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream, ip: &str) {
    let mut msg: Vec<u8> = Vec::new();

    loop {
        let buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(res) => {
                if res < 1 {
                    println!("Client disconnect {}", ip);
                    return;
                }
                let mut x = 0;
                for c in buf {
                    if x >= res {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        println!(
                            "message received {}: {}",
                            ip,
                            String::from_utf8(msg).unwrap()
                        );
                        match stream.write(b"ok\n") {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err);
                            }
                        };
                        msg = Vec::new();
                    } else {
                        msg.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Client {} Disconnect", ip);
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:1234").unwrap();

    println!("wait client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ip = match stream.peer_addr() {
                    Ok(addr) => format!("(ip: {})", addr),
                    Err(_) => "inconnu".to_owned(),
                };
                println!("New client: {}", ip);
                thread::spawn(move || handle_client(stream, &*ip));
            }
            Err(_) => {
                println!("Connection Failed");
            }
        };
    }
}
