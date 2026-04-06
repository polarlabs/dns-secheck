use crate::util;

use hickory_client::client::{Client, ClientHandle};
use hickory_proto::rr::{DNSClass, Name, Record, RecordType};
use hickory_proto::runtime::TokioRuntimeProvider;
use hickory_proto::tcp::TcpClientStream;
use hickory_proto::udp::UdpClientStream;
use hickory_proto::xfer::DnsResponse;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use crate::dns::util::A;

pub struct DNSClient {
    server: IpAddr,
    udp_client: Client,
    tcp_client: Client,
}

impl DNSClient {
    pub async fn new(server: &IpAddr) -> Result<DNSClient, std::io::Error> {
        let address = SocketAddr::from((*server, 53));

        let udp_client = {
            let conn = UdpClientStream::builder(address, TokioRuntimeProvider::default()).build();
            let (mut client, bg) = Client::connect(conn).await?;
            tokio::spawn(bg);

            client
        };

        let tcp_client = {
            let (stream, sender) = TcpClientStream::new(address, None, None, TokioRuntimeProvider::new());
            let (mut client, bg) = Client::new(stream, sender, None).await?;
            tokio::spawn(bg);

            client
        };

        let dns_client = DNSClient {
            server: server.clone(),
            udp_client,
            tcp_client,
        };

        Ok(dns_client)
    }

    pub async fn send_udp_dns_request(&mut self, key: &str) -> std::io::Result<()> {
        let answers = send_dns_request(&mut self.tcp_client, &key).await;

        if answers.is_empty() {
            Ok(())
        } else {
            // Records are generic objects that can contain any data.
            //  To access it, we need to first check what type of record it is
            //  In this case we are interested in A, IPv4 address
            let a_data = answers
                .iter()
                .flat_map(|record| record.data().as_a())
                .collect::<Vec<_>>();

            if **a_data.first().unwrap() == A::new(self.server).0 {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "wrong resolution",
                ))
            }

        }
    }

    pub async fn send_tcp_dns_request(&mut self, key: &str) -> std::io::Result<()> {
        let answers = send_dns_request(&mut self.tcp_client, &key).await;

        if answers.is_empty() {
            Ok(())
        } else {
            // Records are generic objects that can contain any data.
            //  To access it, we need to first check what type of record it is
            //  In this case we are interested in A, IPv4 address
            let a_data = answers
                .iter()
                .flat_map(|record| record.data().as_a())
                .collect::<Vec<_>>();

            if **a_data.first().unwrap() == A::new(self.server).0 {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "wrong resolution",
                ))
            }
        }
    }
}

async fn send_dns_request(client: &mut Client, key: &str) -> Vec<Record> {
    let hostname = format!("{}:host", key);
    let hostname = util::base32::encode(&hostname);

    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(format!("{}.domain.de.", &hostname).as_str()).unwrap();

    // Send the query and get a message response
    if let Ok(response) = client
        .query(name, DNSClass::IN, RecordType::A)
        .await {
        let answers: Vec<Record> = response.answers().to_vec();
        answers
    } else {
        Vec::new()
    }
}
