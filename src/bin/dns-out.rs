use mylib::{error_out, key_out, ok_out, resolve_via_system, send_tcp_dns_request, send_udp_dns_request, warn_out};

use std::io::{Read, Write};
use std::time::Duration;

const SERVER_PORT: &str = "53";
const READ_TIMEOUT: Duration = Duration::from_secs(5);
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);

use clap::{Parser, crate_version, value_parser};
use mylib::util::Status;

// Clap creates help text from doc comments (introduced with ///)
#[derive(Parser, Clone, Debug)]
#[command(about, before_help = mylib::legal_note(), version = mylib::version())]
pub struct Cli {
    /// Hostname or IP of DNS Secheck Server
    #[arg(short = 's', long = "server", env = mylib::constants::ENV_DNS_SECHECK_SERVER, default_value = mylib::constants::DNS_SECHECK_SERVER)]
    server: String,

    /// Port to connect to DNS Secheck DNS Server
    #[arg(long = "dns-port", value_parser = value_parser!(u16).range(1..=65535), env = mylib::constants::ENV_DNS_SECHECK_DNS_SERVER_PORT, default_value_t = mylib::constants::DNS_SECHECK_DNS_SERVER_PORT)]
    dns_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let server = args.server;
    let mut key = String::new();

    let result = mylib::check_status(&server).await;
    match result {
        Ok(response) => {
            ok_out("server is up");

            let status = response.text().await.unwrap();
            let status: Status = serde_json::from_str(&status).unwrap();
            if status.version() == crate_version!().to_string() {
                ok_out("version match: client / server");
            } else {
                warn_out(
                    format!(
                        "version mismatch client / server ({} vs {})",
                        crate_version!(),
                        status.version()
                    )
                    .as_str(),
                );
            }
        }
        Err(e) => {
            error_out("server is down");
            return Err(e);
        }
    }

    match mylib::new_test(&server).await {
        Ok(response) => {
            let tmp = response.text().await.unwrap();
            key = serde_json::from_str(&tmp).unwrap();
            key_out(
                format!(
                    "key: {}. use http://{}/test/{} for test result",
                    key, server, key
                )
                    .as_str(),
            );
        }
        Err(e) => {
            error_out("failed to retrieve a test key");
            return Err(e);
        }
    }

    let result = send_tcp_packet(&server, &key);
    match result {
        Ok(_) => warn_out("sent arbitrary data to the server on port 53 via TCP"),
        Err(_) => ok_out("could not send arbitrary data to the server on port 53 via TCP"),
    }

    let result = send_udp_packet(&server, &key);
    match result {
        Ok(_) => warn_out("sent arbitrary data to the server on port 53 via UDP"),
        Err(_) => ok_out("could not send arbitrary data to the server on port 53 via UDP"),
    }

    let result = send_udp_dns_request(&server, &key).await;
    match result {
        Ok(_) => warn_out("sent and received DNS data to the server on port 53 via UDP"),
        Err(_) => ok_out("could not send DNS data to the server on port 53 via UDP"),
    }

    let result = send_tcp_dns_request(&server, &key).await;
    match result {
        Ok(_) => warn_out("sent and received DNS data to the server on port 53 via TCP"),
        Err(_) => ok_out("could not send DNS data to the server on port 53 via TCP"),
    }

    let domains = mylib::malicious_domains(&server).await;
    for domain in domains {
        match resolve_via_system(&domain) {
            Ok(_) => {
                warn_out(format!("could resolve known malicious domain {}", domain).as_str());
            }
            Err(_) => {
                ok_out(format!("could not resolve known malicious domain {}", domain).as_str());
            }
        }
    }


}

fn send_tcp_packet(server: &str, key: &str) -> std::io::Result<()> {
    let mut stream = mylib::tcp_connect(format!("{server}:{SERVER_PORT}"))?;
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    stream.set_write_timeout(Some(WRITE_TIMEOUT))?;

    stream.write_all(format!("{key}:ping\n").as_bytes())?;
    //stream.write_all(format!("ping\n").as_bytes())?;
    stream.flush()?;

    let mut response = [0; 5];
    stream.read_exact(&mut response)?;

    if &response == b"pong\n" {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "no pong received",
        ))
    }
}

fn send_udp_packet(server: &str, key: &str) -> std::io::Result<()> {
    let socket = mylib::udp::udp_connect(format!("{server}:{SERVER_PORT}"))?;

    socket.send(format!("{key}:ping\n").as_bytes())?;

    let mut buf = [0u8; 1024];
    let len = socket.recv(&mut buf)?;

    if &buf[..len] == b"pong\n" {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "no pong received",
        ))
    }
}
