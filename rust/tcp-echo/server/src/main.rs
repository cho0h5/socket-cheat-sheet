use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (mut stream, _) = listener.accept()?;

    let mut buffer = [0; 1024];
    for stream in listener.incoming() {
        let mut stream = stream?;

        stream.read(&mut buffer)?;
        stream.write(&buffer)?;
    }

    Ok(())
}
