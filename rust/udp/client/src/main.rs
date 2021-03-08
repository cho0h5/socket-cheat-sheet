use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3400")?;
    socket.connect("127.0.0.1:8080")?;
    socket.send("hello server".as_bytes())?;
    let mut buffer = [0; 1024];
    let number_of_bytes = socket.recv(&mut buffer)?;
    println!(
        "from server: {}",
        str::from_utf8(&buffer[..number_of_bytes]).unwrap()
    );

    Ok(())
}
