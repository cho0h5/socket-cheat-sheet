use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::str;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("../uds-echo.sock")?;

    let mut buffer = [0; 1024];

    let start = Instant::now();
    stream.write("echo echo echo".as_bytes())?;
    stream.read(&mut buffer)?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("return: {}", str::from_utf8(&buffer).unwrap());

    Ok(())
}
