use std::net::UdpSocket;
use std::str;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3400")?;
    socket.connect("127.0.0.1:8080")?;

    let mut buffer = [0; 1024];

    let start = Instant::now();
    socket.send("echo echo echo".as_bytes())?;
    socket.recv(&mut buffer)?;
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("return: {}", str::from_utf8(&buffer).unwrap());

    Ok(())
}
