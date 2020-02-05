use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            let text = from_utf8(&data).unwrap(); //println!을 위해서 바꿔줄 것.
            println!("client says {}", text);
            stream.write(&data[0..size]).unwrap(); //data[]안에 0..size안쓰면 thread error뜬다.
            //Q?여기서 write의 인자가 slice여야 하기 때문 data의 일부분 즉 slice를 쓰는 것인가.
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9999").unwrap();
    // accept connections and process them, spawning a new thread for each one

    println!("Server listening on port 9999");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
