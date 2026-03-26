use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use crate::util;
use hickory_client::client::{Client, ClientHandle};
use hickory_client::proto::rr::{DNSClass, Name, Record, RecordType};
use hickory_client::proto::runtime::TokioRuntimeProvider;
use hickory_client::proto::udp::UdpClientStream;
use hickory_client::proto::xfer::DnsResponse;
use hickory_proto::rr::rdata::A;
use hickory_proto::tcp::TcpClientStream;

use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::Resolver;
use hickory_resolver::system_conf::read_system_conf;


pub fn resolve_via_system(host: &str) -> Result<Vec<std::net::IpAddr>, Box<dyn std::error::Error>> {

    let addrs = (host, 0).to_socket_addrs()?;

    Ok(addrs.map(|a| a.ip()).collect())
}

pub async fn resolve_via_system2(host: &str) {
    let (config, opts) = read_system_conf().unwrap();

    // Build resolver that uses the system configuration
    let resolver = Resolver::builder_with_config(
        config,
        TokioConnectionProvider::default()
    ).build();

    let lookup = resolver.lookup_ip(format!("{}.", host)).await.unwrap();
    let lookup = lookup.as_lookup();

    let mut blocked_n = 0;
    for record in lookup.records() {
        match record.record_type() {
            RecordType::A => {
                let a = record.data().as_a().unwrap().to_string();
                if blocked_a(&a) {
                    blocked_n += 1;
                }
            }
            RecordType::CNAME => {
                let cname = record.data().as_cname().unwrap().to_string();

                if blocked_cname(&cname) {
                    blocked_n += 1;
                }
            }
            _ => {}
        }
    }

    if blocked_n == lookup.records().len() {
        println!("DNS resolution of {} has been blocked", host);
    } else {
        println!("DNS resolution of {} has not blocked", host);
    }
}

fn blocked_a(a: &str) -> bool {
    match a {
        "185.242.177.5" => true,
        _ => false,
    }

}

fn blocked_cname(cname: &str) -> bool {
    match cname {
        "block.blue-shield.at." => true,
        _ => false,
    }
}

/*
todo: check for registered domain
use publicsuffix::List; // or another PSL crate

fn registered_domain_from_cname(cname_rdata: &RData, psl: &List) -> Option<String> {
    let target = match cname_rdata {
        RData::CNAME(name) => name,
        _ => return None,
    };

    let domain = target.to_utf8(); // e.g. "a.b.example.co.uk."
    let domain = domain.trim_end_matches('.'); // remove trailing dot

    psl.parse_domain(domain)
        .ok()?
        .root()
        .map(|d| d.to_string()) // "example.co.uk"
}
 */
pub async fn send_udp_dns_request(server: &str, key: &str) -> std::io::Result<()> {
    let address = SocketAddr::from(([93, 177, 64, 153], 53));

    let conn = UdpClientStream::builder(address, TokioRuntimeProvider::default()).build();
    let (mut client, bg) = Client::connect(conn).await.unwrap();

    tokio::spawn(bg);

    let hostname = format!("{}:host", key);
    let hostname = util::base32::encode(&hostname);

    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(format!("{}.domain.de.", &hostname).as_str()).unwrap();

    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    let response: DnsResponse = client
        .query(name, DNSClass::IN, RecordType::A)
        .await
        .unwrap();

    let answers: &[Record] = response.answers();

    // Records are generic objects that can contain any data.
    //  To access it, we need to first check what type of record it is
    //  In this case we are interested in A, IPv4 address
    let a_data = answers
        .iter()
        .flat_map(|record| record.data().as_a())
        .collect::<Vec<_>>();

    if **a_data.first().unwrap() == A::new(1, 2, 3, 4) {
        Ok(())

    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "wrong resolution",
        ))
    }
}

pub async fn send_tcp_dns_request(server: &str, key: &str) -> std::io::Result<()> {
    let address = SocketAddr::from(([93, 177, 64, 153], 53));

    let (stream, sender) = TcpClientStream::new(address, None, None, TokioRuntimeProvider::new());

    let (mut client, bg) = Client::new(stream, sender, None).await.unwrap();

    tokio::spawn(bg);

    let hostname = format!("{}:host", key);
    let hostname = util::base32::encode(&hostname);

    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(format!("{}.domain.de.", &hostname).as_str()).unwrap();

    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    let response: DnsResponse = client
        .query(name, DNSClass::IN, RecordType::A)
        .await
        .unwrap();

    let answers: &[Record] = response.answers();

    // Records are generic objects that can contain any data.
    //  To access it, we need to first check what type of record it is
    //  In this case we are interested in A, IPv4 address
    let a_data = answers
        .iter()
        .flat_map(|record| record.data().as_a())
        .collect::<Vec<_>>();

    if **a_data.first().unwrap() == A::new(1, 2, 3, 4) {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "wrong resolution",
        ))
    }
}
