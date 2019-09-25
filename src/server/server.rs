use crate::config::{Configs, Config};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::proto::op::message::Message;
use trust_dns::proto::op::query::Query;
use trust_dns::proto::rr::RecordType;
use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::codec::BytesCodec;
use futures::sink::Sink;

use log::{warn, info, debug};
use tokio::prelude::*;
use tokio;

pub fn server() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 54));
    let socket = UdpSocket::bind(&addr).unwrap();
    let (sink, stream) = UdpFramed::new(socket, BytesCodec::new()).split();

    let stream = stream.map(move |(msg, addr)| {
        let resp_buf = match dns_search(&addr, &msg) {
            Ok(b) => b,
            Err(e) => {
                warn!("dns_search error: {}", e);
                return ("PONG".into(), addr);
            }
        };
        ("PONG".into(), addr)
    });
    tokio::run({
        sink.send_all(stream)
            .map(|_| ())
            .map_err(|e| println!("error = {:?}", e))
    });
//
//    loop {
//        let (_amt, src) = match socket.recv_from(&mut buf) {
//            Ok(a) => a,
//            Err(e) => {
//                warn!("socket recv_from error: {}", e.to_string());
//                continue;
//            }
//        };
//
//        // 向第三方 DNS 服务器发送查询请求
//        let resp_buf = match dns_search(&src, &configs, &buf) {
//            Ok(b) => b,
//            Err(e) => {
//                warn!("dns_search error: {}", e);
//                continue;
//            }
//        };
//        // 将第三方的请求结果返回给请求者
//        let _ = match socket.send_to(&resp_buf, src) {
//            Ok(_) => debug!("dns message send success: src: {}", src),
//            Err(e) => warn!("socket send_to error: {}", e.to_string())
//        };
//    }
}

fn dns_search(source_address: &SocketAddr, buf: &[u8]) -> Result<Vec<u8>, String> {
    let message = match Message::from_vec(&buf) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string())
    };

    // https://stackoverflow.com/questions/4082081/requesting-a-and-aaaa-records-in-single-dns-query/4083071
    if message.queries().len() != 1 {
        return Err("dns query format error".to_string());
    }
    let query = &message.queries()[0];
    // 根据规则获取查询 DNS 服务器地址
    let dns_server_address = remote_dns_server_address(&query)?;

    let mut resp_message = match query.query_type() {
        RecordType::A => do_lookup(query)?,
        _ => do_search(query, dns_server_address.as_str())?
    };
    resp_message.set_id(message.id());
    let resp = match resp_message.to_vec() {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    info!("source: {}\t\tdns server: {}\t\tname: {}", source_address, dns_server_address, query.name());
    Ok(resp)
}

fn do_lookup(query: &Query) -> Result<Message, String> {
    let resolver = match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };
    let response = match resolver.lookup(&query.name().to_ascii(), RecordType::A) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };
    let mut message = Message::new();

    message.add_query(query.clone());

    for record in response.record_iter() {
        message.add_answer(record.clone());
    }
    Ok(message)
}

fn do_search(query: &Query, dns_server_address: &str) -> Result<Message, String> {
    let dns_server_address = match dns_server_address.parse() {
        Ok(a) => a,
        Err(_) => return Err("addr parse error".to_string())
    };
    let conn = match UdpClientConnection::new(dns_server_address) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };
    let client = SyncClient::new(conn);

    let response = match client.query(query.name(), query.query_class(), query.query_type()) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };
    for message in response.messages() {
        return Ok(message.clone());
    }
    Err("Response no message".to_string())
}

fn remote_dns_server_address(query: &Query) -> Result<String, String> {
    let configs = Configs::load();
    let config = configs.filter_rule(&query.name().to_string())?;
    Ok(config)
}
