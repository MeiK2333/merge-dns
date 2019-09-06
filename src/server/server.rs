use crate::config::Config;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::proto::op::message::Message;
use trust_dns::proto::op::query::Query;

use log::{warn, info, debug};

pub fn server(configs: Vec<Config>) {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    let socket = UdpSocket::bind(&addr).unwrap();
    let mut buf = [0; 512];
    loop {
        let (_amt, src) = match socket.recv_from(&mut buf) {
            Ok(a) => a,
            Err(e) => {
                warn!("socket recv_from error: {}", e.to_string());
                continue;
            }
        };

        // 向第三方 DNS 服务器发送查询请求
        let resp_buf = match dns_search(&src, &configs, &buf) {
            Ok(b) => b,
            Err(e) => {
                warn!("dns_search error: {}", e);
                continue;
            }
        };
        // 将第三方的请求结果返回给请求者
        let _ = match socket.send_to(&resp_buf, src) {
            Ok(_) => debug!("dns message send success: src: {}", src),
            Err(e) => warn!("socket send_to error: {}", e.to_string())
        };
    }
}

fn dns_search(source_address: &SocketAddr, configs: &Vec<Config>, buf: &[u8]) -> Result<Vec<u8>, String> {
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
    let dns_server_address = remote_dns_server_address(&configs, &query)?;

    let mut resp_message = do_search(query, dns_server_address)?;
    resp_message.set_id(message.id());
    let resp = match resp_message.to_vec() {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    info!("source: {}\tdns server: {}\tname: {}", source_address, dns_server_address, query.name());
    Ok(resp)
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

fn remote_dns_server_address<'a>(configs: &'a Vec<Config>, query: &Query) -> Result<&'a str, &'a str> {
    let config = Config::filter_rule(&configs, &query.name().to_string())?;
    return Ok(&config.dns);
}