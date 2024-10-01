
use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};

fn handle_client(mut stream: TcpStream) {

    println!("Connection from {}", stream.local_addr().unwrap());
    
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer);
    
    let b64_decoded_content = general_purpose::STANDARD.decode(buffer).unwrap();
    let archive_name = format!("grabby_files_{}.zip", 
                                stream.local_addr()
                                .unwrap()
                                .to_string()
                                .split(":")
                                .nth(0)
                                .unwrap()
                                .replace(".", "-"));
    println!("{}", archive_name);
    fs::write(PathBuf::from(archive_name), b64_decoded_content).expect("err");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1338");

    // accept connections and process them serially
    for stream in listener?.incoming() {
        handle_client(stream?);
    }
    Ok(())
}