use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    stream.write("hello server".as_bytes())?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("from server: {}", str::from_utf8(&buffer).unwrap());

    Ok(())
}
