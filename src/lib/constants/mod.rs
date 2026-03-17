use std::time::Duration;

// Globals, used internally only, not exposed to user or interface.
pub const COPYRIGHT: &str = "Copyright © 2026 Polarlabs";
pub const LICENSE: &str = "AGPL-3.0-only";

// Default values, eventually overwritten by CLI Arguments or environment variables.
pub const DNS_SECHECK_SERVER: &str = "93.177.64.153";
pub const DNS_SECHECK_SERVER_DEFAULT_INTERFACE: &str = "0.0.0.0";
pub const DNS_SECHECK_DNS_SERVER_PORT: u16 = 53;
pub const DNS_SECHECK_HTTP_SERVER_PORT: u16 = 80;
pub const DNS_SECHECK_HTTP_CACHE_SIZE: u64 = 1_000;
pub const DNS_SECHECK_HTTP_CACHE_TIME_TO_IDLE_S: Duration = Duration::from_secs(3600);
pub const DNS_SECHECK_KEY_LENGTH_USIZE: usize = 6;

pub const DNS_SECHECK_MAX_TCP_MESSAGE_SIZE: usize = 64 * 1024;
pub const DNS_SECHECK_MAX_UDP_MESSAGE_SIZE: usize = 64 * 1024;

// Names of environment variables.
pub const ENV_DNS_SECHECK_SERVER: &str = "DNS_SECHECK_SERVER";
pub const ENV_DNS_SECHECK_SERVER_DEFAULT_INTERFACE: &str = "DNS_SECHECK_SERVER_DEFAULT_INTERFACE";
pub const ENV_DNS_SECHECK_DNS_SERVER_PORT: &str = "DNS_SECHECK_DNS_SERVER_PORT";
pub const ENV_DNS_SECHECK_HTTP_SERVER_PORT: &str = "DNS_SECHECK_HTTP_SERVER_PORT";
pub const ENV_DNS_SECHECK_HTTP_CACHE_SIZE: &str = "DNS_SECHECK_HTTP_CACHE_SIZE";
pub const ENV_DNS_SECHECK_HTTP_CACHE_TIME_TO_IDLE_S: &str = "DNS_SECHECK_HTTP_CACHE_TIME_TO_IDLE_S";
