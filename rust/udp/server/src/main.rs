use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    let mut buffer = [0; 1024];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer)?;
    println!(
        "from client: {}",
        str::from_utf8(&buffer[..number_of_bytes]).unwrap()
    );
    socket.send_to("hello client".as_bytes(), src_addr)?;

    Ok(())
}
