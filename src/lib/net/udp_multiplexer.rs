use std::net::IpAddr;
use crate::constants::DNS_SECHECK_MAX_UDP_MESSAGE_SIZE;
use crate::dns_protocol::is_dns_packet;
use crate::net::udp_socket::TokioUdpSocket;
use crate::util::{TestKey, TestResult};
use crate::{Cache, dns_protocol};
use crate::util::string::split_on_first;

pub struct UdpMultiplexer {
    interface: IpAddr,
}

impl UdpMultiplexer {
    pub async fn run(socket: TokioUdpSocket, cache: Cache) -> std::io::Result<()> {
        let socket = socket.0;
        let mut buf = [0u8; DNS_SECHECK_MAX_UDP_MESSAGE_SIZE];

        let server = UdpMultiplexer {
            interface: socket.local_addr()?.ip(),
        };

        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let packet = &buf[..len];

            if is_dns_packet(packet) {
                server.handle_dns(packet, addr, &socket, cache.clone()).await?;
            } else {
                println!("Arbitrary packet from {} (UDP)", addr);

                let message = String::from_utf8_lossy(packet).trim().to_string();
                match split_on_first(&message, ':') {
                    (None, _) => {}
                    (Some(_), None) => {}
                    (Some(key), Some(message)) => {
                        let key = TestKey::from(key);

                        if message == "ping" {
                            println!("Received ping, sending pong (UDP)");
                            socket.send_to(b"pong\n", addr).await?;
                            cache
                                .upsert(
                                    key,
                                    TestResult::from(format!("TCP: received ping from {addr}, sent pong").as_str()),
                                )
                                .await;
                        } else {
                            socket.send_to(b"unknown command\n", addr).await?;
                        }
                    }
                }
            }
        }
    }

    async fn handle_dns(
        &self,
        packet: &[u8],
        addr: std::net::SocketAddr,
        socket: &tokio::net::UdpSocket,
        cache: Cache,
    ) -> std::io::Result<()> {
        println!("DNS packet from {} via UDP", addr);

        let message = dns_protocol::parse_dns(packet)?;
        let key = dns_protocol::parse_dns_request(&message);

        match key {
            None => {}
            Some(key) => {
                let a = crate::dns::util::A::new(self.interface);
                let message = dns_protocol::build_dns_response(&message, &a);

                let bytes = message.to_vec()?;

                socket.send_to(&bytes, addr).await?;

                cache
                    .upsert(
                        key.into(),
                        TestResult::from(
                            format!("UDP: received DNS request from {addr}, sent DNS response")
                                .as_str(),
                        ),
                    )
                    .await;
            }
        }

        Ok(())
    }
}
