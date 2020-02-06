use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::fs::File;
use std::fs;
use std::path::Path;
//use std::env;

fn printmenu(){
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("ls : lists current directory");
    println!("get <file name> : get a file from server");
    println!("q : quit the program");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
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
    /* 인자 받기 */
    //let args: Vec<String> = env::args().collect();
    //let option = String::from(&args[1]);
    //println!("{:?}", args);

    match TcpStream::connect("192.168.0.214:9999") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9999");
            printmenu();

            let mut option = String::new();

            std::io::stdin().read_line(&mut option).expect("Failed to read line");

            if option.len() > 2 {
                match &option[0..3] {
                    "ls\n" =>
                    {
                        let msg = b"ls";
                        stream.write(msg).unwrap();

                        //let msg2 = from_utf8(msg).unwrap();
                        //println!("Sent {}", msg2);

                        let mut data = [0 as u8; 512]; // using 6 byte buffer
                        match stream.read(&mut data) {
                            Ok(_) => {
                                let text = from_utf8(&data).unwrap();
                                println!("{}",text);
                            },
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
                            }
                        }
                    }
                    "get" =>
                    {
                        /*TO DO*/
                        stream.write(option.as_bytes()).unwrap();
                        let mut data2 = [0 as u8; 10000];
                        let second_argument = &option[4..].trim();
                        let filename = second_arg(&second_argument);

                        match stream.read(&mut data2) {
                            Ok(_) => {
                                if &data2[0..5] == b"error" {
                                    println!("Failed to get {} - Not existing", filename);
                                }
                                else{
                                    match fs::create_dir("out") {
                                        Err(why) => println!("Out put direcotry {:?}. The file will be located in ./out", why.kind()),
                                        Ok(_) => {},
                                    }
                                    let path = Path::new("out/");
                                    let temp_file = path.join(filename);

                                    let mut file = File::create(temp_file).unwrap();
                                    match file.write_all(&data2[..]) {
                                        Err(why) => println!("! {:?}", why.kind()),
                                        Ok(_) => {},
                                    }
                                    //let text = from_utf8(&data2).unwrap();
                                    //println!("{}",text);
                                }
                            },
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
                            }
                        }

                    }
                    _=> println!("wrong input. Try again."),
                }
            }
            else{
                if option.trim() == String::from("q"){
                    println!("Quit the program");
                }
                else{
                    println!("wrong input. Try again.")
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
