use std::os::unix::net::UnixListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("../uds-echo.sock")?;

    let (mut stream, _) = listener.accept()?;

    let mut buffer = [0; 1024];
    for stream in listener.incoming() {
        let mut stream = stream?;

        stream.read(&mut buffer)?;
        stream.write(&buffer)?;
    }

    Ok(())
}
