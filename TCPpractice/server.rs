use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::fs::File;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::process::Command;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    let _first_ls = "l".to_string();
    let _first_get = "g".to_string();

    loop{
        match stream.read(&mut data) {
            Ok(_size) => {
                // echo
                let clientc = from_utf8(&data).unwrap();
                let cmp = String::from(&clientc[0..1]);

                match &cmp[..] {
                    "l" => {
                        println!("...Listing current directory in Client...");
                        let output = Command::new("ls").output()
                                                       .expect("ls command failed to start");
                        stream.write(&output.stdout).unwrap();
                    },
                    "g" => {
                        let second_argument = &clientc[4..].trim();
                        let filename = second_arg(&second_argument);
                        println!("...Find {} asap!!...", filename);

                        let filecontents = read_a_file(&filename);
                        if  &filecontents[..] == "error" {
                            stream.write(b"error").unwrap();
                        }
                        else{
                            stream.write(&filecontents.as_bytes()).unwrap();
                        }
                    },
                    _ => {},
                }
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                //false
            }
        }
    }
}
fn read_a_file(filename : &str) -> String {
    match File::open(filename.trim()){
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();
            content
            // The file is automatically closed when is goes out of scope.
        },
        // Error handling.
        Err(_error) => {
            println!("There is no such file");
            let error_msg = String::from("error");
            error_msg
        },
    }
}
fn second_arg(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return &s[0..i];
        }
    }
    &s[..]
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
