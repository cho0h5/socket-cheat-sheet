use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;

    let mut buffer = [0; 1024];

    loop {
        let (_, src_addr) = socket.recv_from(&mut buffer)?;
        socket.send_to(&buffer, src_addr)?;
    }
}
