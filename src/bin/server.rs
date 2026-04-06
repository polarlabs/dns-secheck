use std::net::IpAddr;
use clap::{Parser, value_parser};

use mylib::{Cache, http_server, net};
use mylib::net::tcp_multiplexer::TcpMultiplexer;
use mylib::net::udp_multiplexer::UdpMultiplexer;

// Clap creates help text from doc comments (introduced with ///)
#[derive(Parser, Clone, Debug)]
#[command(about, before_help = mylib::legal_note(), version = mylib::version())]
struct Cli {
    /// Interface to bind the DNS Secheck Server
    #[arg(short = 'i', long = "interface",
    env = mylib::constants::ENV_DNS_SECHECK_SERVER_DEFAULT_INTERFACE,
    default_value = mylib::constants::DNS_SECHECK_SERVER_DEFAULT_INTERFACE)]
    interface: IpAddr,

    /// Port used by DNS Secheck HTTP Server
    #[arg(long = "dns-port", value_parser = value_parser!(u16).range(1..=65535),
    env = mylib::constants::ENV_DNS_SECHECK_DNS_SERVER_PORT,
    default_value_t = mylib::constants::DNS_SECHECK_DNS_SERVER_PORT)]
    dns_port: u16,

    /// Port used by DNS Secheck HTTP Server
    #[arg(long = "http-port", value_parser = value_parser!(u16).range(1..=65535),
    env = mylib::constants::ENV_DNS_SECHECK_HTTP_SERVER_PORT,
    default_value_t = mylib::constants::DNS_SECHECK_HTTP_SERVER_PORT)]
    http_port: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let interface = args.interface;
    let dns_port = args.dns_port;
    let http_port = args.http_port;

    let cache = Cache::new();

    let listener = net::StdTcpListener::bind(&interface, http_port);
    let http_server =
        http_server::run_http(listener, cache.clone()).expect("Failed to start http server.");
    let http_server = tokio::spawn(http_server);

    let listener = net::TokioTcpListener::bind(&interface, dns_port).await;
    let tcp_mux = TcpMultiplexer::run(listener, cache.clone());
    let tcp_mux = tokio::spawn(tcp_mux);

    let socket = net::TokioUdpSocket::bind(&interface, dns_port).await;
    let udp_mux = UdpMultiplexer::run(socket, cache.clone());
    let udp_mux = tokio::spawn(udp_mux);

    tokio::select! {
        _ = http_server => {}
        _ = tcp_mux => {}
        _ = udp_mux => {}
        _ = tokio::signal::ctrl_c() => {
            println!("Shutdown");
        }
    }

    Ok(())
}
