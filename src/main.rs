use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("gemini.conman.org:1965")?;
    stream.write_all(b"gemini://gemini.conman.org/\r
")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    Ok(())
}
