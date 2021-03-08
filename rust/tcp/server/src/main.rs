use std::net::TcpListener;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (mut stream, _) = listener.accept()?;
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("from client: {}", str::from_utf8(&buffer).unwrap());
    stream.write("hello client".as_bytes())?;

    Ok(())
}
