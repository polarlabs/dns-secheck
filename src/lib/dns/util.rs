use std::net::IpAddr;

pub struct A(pub hickory_proto::rr::rdata::A);

impl A {
    pub fn new(addr: IpAddr) -> A {
        let octets = match addr {
            IpAddr::V4(addr) => addr.octets(),
            IpAddr::V6(_) => unimplemented!(),
        };
        
        let a = hickory_proto::rr::rdata::A::new(octets[0], octets[1], octets[2], octets[3]); 
        
        A(a)
    }
}
