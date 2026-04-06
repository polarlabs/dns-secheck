use std::net::IpAddr;

pub struct TokioUdpSocket(pub tokio::net::UdpSocket);

impl TokioUdpSocket {
    pub async fn bind(interface: &IpAddr, port: u16) -> Self {
        let socket = tokio::net::UdpSocket::bind(format!("{interface}:{port}"))
            .await
            .expect("Failed to bind to port {interface}:{port}.");

        TokioUdpSocket(socket)
    }
}
