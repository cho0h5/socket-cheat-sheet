use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::str;


fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("../uds.sock")?;

    let (mut stream, _) = listener.accept()?;
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("from client: {}", str::from_utf8(&buffer).unwrap());
    stream.write("hello client".as_bytes())?;

    Ok(())
}
