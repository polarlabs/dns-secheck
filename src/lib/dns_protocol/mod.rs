use crate::util;
use hickory_proto::op::{Message, MessageType, OpCode, ResponseCode};
use hickory_proto::rr::rdata::A;
use hickory_proto::rr::{RData, Record};
use std::net::Ipv4Addr;

pub fn parse_dns(packet: &[u8]) -> Result<Message, hickory_proto::ProtoError> {
    Message::from_vec(packet)
}

pub fn parse_dns_request(msg: &Message) -> Option<String> {
    let query = msg.queries().first()?;

    let name = query.name().to_string();
    let qtype = query.query_type();

    if let (Some(hostname), Some(domain)) = util::string::split_on_first(&name, '.') {
        if let Ok(decoded_hostname) = util::base32::decode(&hostname) {
            let key =
                if let (Some(key), Some(name)) = util::string::split_on_first(&decoded_hostname, ':') {
                    println!("Key: {} {} {:?}", key, name, qtype);
                    Some(key.to_string())
                } else {
                    println!("Invalid name: {}", decoded_hostname);
                    None
                };

            key
        } else {
            println!("Invalid name: {}", hostname);

            None
        }
    } else {
        println!("Invalid name: {}", name);

        None
    }
}

// Check if data is a DNS packet
pub fn is_dns_packet(data: &[u8]) -> bool {
    Message::from_vec(data).is_ok()
}

pub fn looks_like_dns_tcp(buf: &[u8]) -> bool {
    if buf.len() < 14 {
        return false;
    }

    let length = u16::from_be_bytes([buf[0], buf[1]]) as usize;

    if length < 12 {
        return false;
    }

    if buf.len() < length + 2 {
        return false;
    }

    let qdcount = u16::from_be_bytes([buf[6], buf[7]]);
    let ancount = u16::from_be_bytes([buf[8], buf[9]]);

    // DNS queries typically have at least one question
    if qdcount == 0 {
        return false;
    }

    // queries usually have no answers
    if ancount > 0 {
        return false;
    }

    true
}

pub fn build_dns_response(request: &Message) -> Message {
    let mut response = Message::new();

    response.set_id(request.id());
    response.set_message_type(MessageType::Response);
    response.set_op_code(OpCode::Query);
    response.set_authoritative(true);
    response.set_response_code(ResponseCode::NoError);

    response.add_queries(request.queries().to_vec());

    let query = &request.queries()[0];

    let record = Record::from_rdata(
        query.name().clone(),
        60,
        RData::A(A::from(Ipv4Addr::new(1, 2, 3, 4))),
    );

    response.add_answer(record);

    response
}
