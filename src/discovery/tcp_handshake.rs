use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::discovery::discover::Peer;
const TCP_PORT: u16 = 34255;
pub fn tcp_handshake(peer: &Peer, local_id: &str, local_name: &str) -> io::Result<TcpStream> {
    let addr = format!("{}:{}", peer.address.ip(), TCP_PORT);
    let mut stream = TcpStream::connect(addr)?;
    let hello = format!("HELLO|{}|{}|SEND", local_id, local_name);
    stream.write_all(hello.as_bytes());

    let mut buf = [0u8; 256];
    let len = stream.read(&mut buf)?;
    let reply = String::from_utf8_lossy(&buf[..len]);
    println!("TCP reply: {}", reply);
    if reply.starts_with("OK|") {
        println!("Handshake successful with {}", peer.name);
        Ok(stream)
    } else {
        Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "Peer rejected",
        ))
    }
}
