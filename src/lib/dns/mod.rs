mod client;
pub use client::DNSClient;

mod resolver;
pub use resolver::Resolver;

pub mod util;

use hickory_client::proto::rr::RecordType;
use crate::{ok_out, warn_out};

pub async fn resolve_via_system(host: &str) {
    let resolver = Resolver::new();
    let lookup = resolver.lookup_ip(host).await;

    // A lookup might return multiple records, e.g. A and CNAME
    // Check if any of the records are blocked and count the blocked ones
    let mut blocked_n = 0;
    for record in lookup.as_lookup().records() {
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

    // If all records have been blocked
    if blocked_n == lookup.as_lookup().records().len() {
        ok_out(&format!("DNS resolution of {} has been blocked", host));
    } else {
        warn_out(&format!("DNS resolution of {} has not been blocked", host));
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
