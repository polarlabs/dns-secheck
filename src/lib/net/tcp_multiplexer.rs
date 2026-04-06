use crate::constants::{
    DNS_SECHECK_KEY_LENGTH_USIZE, DNS_SECHECK_MAX_TCP_MESSAGE_SIZE,
    DNS_SECHECK_MAX_UDP_MESSAGE_SIZE,
};
use crate::dns_protocol::looks_like_dns_tcp;
use crate::net::TokioTcpListener;
use crate::util::{TestKey, TestResult};
use crate::{Cache, dns_protocol};
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::util::string::split_on_first;

pub struct TcpMultiplexer {
    interface: IpAddr,
}

impl TcpMultiplexer {
    pub async fn run(listener: TokioTcpListener, cache: Cache) -> std::io::Result<()> {
        let listener = listener.0;
        
        let server = TcpMultiplexer {
            interface: listener.local_addr()?.ip(),
        };
        
        loop {
            let (stream, addr) = listener.accept().await?;
            println!("Client connected (TCP): {}", addr);

            server.handle_client(stream, addr, cache.clone()).await?;
        }
    }

    async fn handle_client(
        &self,
        mut stream: tokio::net::TcpStream,
        addr: SocketAddr,
        cache: Cache,
    ) -> std::io::Result<()> {
        let mut buf = Vec::new();
        let mut tmp = [0u8; DNS_SECHECK_MAX_TCP_MESSAGE_SIZE];

        loop {
            let len = stream.read(&mut tmp).await?;
            if len == 0 {
                break;
            } // connection closed

            buf.extend_from_slice(&tmp[..len]);

            if looks_like_dns_tcp(&buf) {
                println!("DNS packet from {} via TCP", addr);

                let cache = cache.clone();
                self.handle_dns(&buf[2..], addr, &mut stream, cache).await?;
                break;
            } else {
                println!("Arbitrary packet from {} (TCP)", addr);
                let cache = cache.clone();

                if let Err(e) = self.handle_generic(&buf, &mut stream, cache).await {
                    println!("Connection error: {:?}", e);
                }
            }
        }

        Ok(())
    }

    async fn handle_dns(
        &self,
        message: &[u8],
        addr: SocketAddr,
        socket: &mut tokio::net::TcpStream,
        cache: Cache,
    ) -> std::io::Result<()> {
        let message = dns_protocol::parse_dns(message)?;
        let key = dns_protocol::parse_dns_request(&message);

        match key {
            None => {}
            Some(key) => {
                let a = crate::dns::util::A::new(self.interface);
                let message = crate::dns_protocol::build_dns_response(&message, &a);

                // serialize DNS message
                let bytes = message.to_vec().expect("failed to encode message");

                // DNS over TCP length prefix
                let len = (bytes.len() as u16).to_be_bytes();

                socket.write_all(&len).await?;
                socket.write_all(&bytes).await?;

                cache
                    .upsert(
                        key.into(),
                        TestResult::from(
                            format!("TCP: received DNS request from {addr}, sent DNS response").as_str(),
                        ),
                    )
                    .await;
            }
        }

        Ok(())
    }

    async fn handle_generic(
        &self,
        message: &[u8],
        socket: &mut tokio::net::TcpStream,
        cache: Cache,
    ) -> Result<(), Box<dyn Error>> {
        let message = String::from_utf8_lossy(message).trim().to_string();
        match split_on_first(&message, ':') {
            (None, _) => {}
            (Some(_), None) => {}
            (Some(key), Some(message)) => {
                let key = TestKey::from(key);

                if message == "ping" {
                    let addr = socket.peer_addr()?;
                    println!("Received ping, sending pong (TCP)");
                    socket.write_all(b"pong\n").await?;
                    cache
                        .upsert(
                            key,
                            TestResult::from(format!("TCP: received ping from {addr}, sent pong").as_str()),
                        )
                        .await;
                } else {
                    socket.write_all(b"unknown command\n").await?;
                }
            }
        }

        Ok(())
    }
}
