use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

const READ_TIMEOUT: Duration = Duration::from_secs(5);

pub fn udp_connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(addr)?;

    // Set receive timeout to 5 seconds
    socket.set_read_timeout(Some(READ_TIMEOUT))?;

    Ok(socket)
}
