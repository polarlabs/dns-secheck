use std::net::IpAddr;

pub struct StdTcpListener(pub std::net::TcpListener);

impl StdTcpListener {
    pub fn bind(interface: &IpAddr, port: u16) -> Self {
        let tcp_listener = std::net::TcpListener::bind(format!("{interface}:{port}"))
            .expect("Failed to bind to port {interface}:{port}.");

        StdTcpListener(tcp_listener)
    }
}

pub struct TokioTcpListener(pub tokio::net::TcpListener);

impl TokioTcpListener {
    pub async fn bind(interface: &IpAddr, port: u16) -> Self {
        let tcp_listener = tokio::net::TcpListener::bind(format!("{interface}:{port}"))
            .await
            .expect("Failed to bind to port {interface}:{port}.");

        TokioTcpListener(tcp_listener)
    }
}
