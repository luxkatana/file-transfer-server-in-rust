extern crate serde;
extern crate serde_json;
use std::io::Write;
use serde::Serialize;
use std::net::TcpListener;
use std::fs;
use std::process::exit;
#[derive(Serialize)]
struct Metadata {
    filename: String,
}


fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let file_to_stream = match args.get(1) {
        Some(e) => e,
        None => {
            eprintln!("SYNTAX: {} <FILE>", args.get(0).unwrap());
            exit(1);
        }
    };
    let metadata = Metadata {
        filename: file_to_stream.to_owned()
    };
    let mut metadata: Vec<u8> = match serde_json::to_string(&metadata) {
        Ok(e) => e.as_bytes().to_vec(),
        Err(err) => {
            eprintln!("Could not serialize data: {err}");
            std::process::exit(1);
        }
    };
    metadata.extend(b"\0");
    let server = TcpListener::bind("127.0.0.1:8000")?;
    let file_data: Vec<u8> = match fs::read(file_to_stream) {
        Ok(e) => e,
        Err(t) => {
            eprintln!("Could not read file data: {t}");
            exit(1);
        }
    };
    metadata.extend(file_data);
    println!("Server up and running at {}", server.local_addr()?);
    loop {
        let (mut client, addr) = server.accept()?;
        if let Err(e) = client.write_all(&mut metadata) {
            eprintln!("Could not send data to {addr}: {e}");
        }
        else {
            println!("Successfully sent data to {addr}");
        }
    }
}
