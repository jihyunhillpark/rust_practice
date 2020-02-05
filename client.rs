use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("192.168.0.214:9999") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9999");

            let msg = b"ls";
            stream.write(msg).unwrap();

            let msg2 = from_utf8(msg).unwrap();
            println!("Sent {}", msg2);

            let mut data = [0 as u8; 2]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                        let text = from_utf8(&data).unwrap();
                        println!("Reply from SERVER : {}", text);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
