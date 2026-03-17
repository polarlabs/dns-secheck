use std::net::{TcpStream, ToSocketAddrs};

pub fn tcp_connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(addr)?;

    Ok(stream)
}
