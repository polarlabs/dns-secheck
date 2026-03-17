mod cache;
pub use cache::Cache;

pub mod constants;

mod dns_client;
pub use dns_client::resolve_via_system;
pub use dns_client::send_tcp_dns_request;
pub use dns_client::send_udp_dns_request;

mod dns_server;
pub use dns_protocol::is_dns_packet;
pub use dns_server::run;

mod http_client;
pub use http_client::check_status;
pub use http_client::malicious_domains;
pub use http_client::new_test;

pub mod http_server;

pub mod net;

mod tcp;
pub use tcp::tcp_connect;

pub mod udp;

mod dns_protocol;
pub mod util;

use clap::{crate_authors, crate_name, crate_version};

const BROKEN_HEART: char = char::from_u32(0x1f494).unwrap();
const KEY: char = char::from_u32(0x1f511).unwrap();

const OK: char = char::from_u32(0x2705).unwrap();
const WARN: char = char::from_u32(0x1F480).unwrap();

pub fn ok_out(out: &str) {
    println!(" {} {}", OK, out);
}

pub fn key_out(out: &str) {
    println!("{}  {}", KEY, out);
}

pub fn error_out(out: &str) {
    println!(" {} {}", BROKEN_HEART, out);
}

pub fn warn_out(out: &str) {
    println!(" {} {}", WARN, out);
}

pub fn legal_note() -> String {
    format!(
        "{} ({}). Licensed under {}. {} represented by <{}>.",
        crate_name!(),
        crate_version!(),
        constants::LICENSE,
        constants::COPYRIGHT,
        crate_authors!(", ")
    )
}

pub fn version() -> &'static str {
    concat!("(", crate_version!(), ").")
}
