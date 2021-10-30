use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("../uds.sock")?;

    stream.write("hello server".as_bytes())?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("from server: {}", str::from_utf8(&buffer).unwrap());

    Ok(())
}
