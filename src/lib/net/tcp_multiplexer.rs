use crate::constants::{
    DNS_SECHECK_KEY_LENGTH_USIZE, DNS_SECHECK_MAX_TCP_MESSAGE_SIZE,
    DNS_SECHECK_MAX_UDP_MESSAGE_SIZE,
};
use crate::dns_protocol::looks_like_dns_tcp;
use crate::net::TokioTcpListener;
use crate::util::{TestKey, TestResult};
use crate::{Cache, dns_protocol, is_dns_packet};
use std::error::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::util::string::split_on_first;

pub async fn run(listener: TokioTcpListener, cache: Cache) -> std::io::Result<()> {
    let listener = listener.0;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client connected (TCP): {}", addr);

        handle_client(stream, addr, cache.clone()).await?;
    }
}

pub async fn handle_client(
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

        /*
        let packet = &buf[..len];

        let mut length_buf = [0u8; 2];

        // Peek first 2 bytes to check DNS framing
        socket.peek(&mut length_buf).await?;

        let msg_len = u16::from_be_bytes(length_buf) as usize;

        // DNS messages must be at least 12 bytes
        if msg_len >= 12 && msg_len <= 4096 {

         */
        if looks_like_dns_tcp(&buf) {
            println!("DNS packet from {} via TCP", addr);

            let cache = cache.clone();
            handle_dns(&buf[2..], addr, &mut stream, cache).await?;
            break;
        } else {
            println!("Arbitrary packet from {} (TCP)", addr);
            let cache = cache.clone();

            if let Err(e) = handle_generic(&buf, &mut stream, cache).await {
                println!("Connection error: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn handle_dns(
    message: &[u8],
    addr: std::net::SocketAddr,
    socket: &mut tokio::net::TcpStream,
    cache: Cache,
) -> std::io::Result<()> {
    let message = dns_protocol::parse_dns(message)?;
    let key = dns_protocol::parse_dns_request(&message);

    match key {
        None => {}
        Some(key) => {
            let message = crate::dns_protocol::build_dns_response(&message);

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
