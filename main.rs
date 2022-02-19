use std::env;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::exit;
use std::thread;

fn reacive_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1460]; // asafos TCP send byte buffer size
    while match stream.read(&mut data) {
        Ok(size) => {
			
                let input: String = String::from_utf8_lossy(&data).to_string();
                //input = input.replace("\u{0}", "");
				
                println!(">{}", input);
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn send_client(mut stream: TcpStream) -> std::io::Result<()> {

    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string != "exit" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        stream.write(input_string.as_bytes())?;
    }

    return Ok(());
}

fn main() {
	
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[2].len() < 4 {
        println!("Usage:\n\narg[1] = IP\narg[2] = port\n");
        exit(-1);
    }
	
    const LIS_PORT:u32 = 6969;
	let CON_PORT = &args[2];
    let IP = &args[1];

    let sender = TcpStream::connect(format!("{}:{}", IP, CON_PORT)).unwrap();

    thread::spawn(move || send_client(sender));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", LIS_PORT)).unwrap();

    println!("AsafOS control server listening on port {}", LIS_PORT);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Recive Port connected at {}", stream.peer_addr().unwrap());
                thread::spawn(move || reacive_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("AsafOS control server Listen on port {}", LIS_PORT);
    drop(listener);
}
