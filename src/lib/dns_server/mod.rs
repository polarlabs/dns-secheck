use hickory_server::ServerFuture;
use hickory_server::authority::{Catalog, ZoneType};
use hickory_server::proto::rr::{Name, RData, Record};
use hickory_server::store::in_memory::InMemoryAuthority;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub async fn run() -> ServerFuture<Catalog> {
    let mut catalog = Catalog::new();

    // Zone origin
    let origin = Name::from_ascii("example.com.").unwrap();

    let authority = InMemoryAuthority::empty(origin.clone(), ZoneType::Primary, false);

    let record = Record::from_rdata(
        Name::from_ascii("www.example.com.").unwrap(),
        60,
        RData::A("93.177.100.100".parse().unwrap()),
    );

    authority.upsert(record, 0).await;

    let authority = Arc::new(authority);
    catalog.upsert(origin.into(), vec![authority]);

    let mut server = ServerFuture::new(catalog);

    let udp_socket = UdpSocket::bind("93.177.64.153:53").await.unwrap();
    server.register_socket(udp_socket);

    //server.block_until_done().await;

    server
}
