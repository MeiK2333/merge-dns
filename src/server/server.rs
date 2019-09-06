use crate::config::Config;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::proto::op::message::Message;

use log::{warn, info};

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
        let resp_buf = match dns_search(&configs, &buf) {
            Ok(b) => b,
            Err(e) => {
                warn!("dns_search error: {}", e);
                continue;
            }
        };
        // 将第三方的请求结果返回给请求者
        let _ = match socket.send_to(&resp_buf, src) {
            Ok(_) => info!("dns search success: src: {}", src),
            Err(e) => warn!("socket send_to error: {}", e.to_string())
        };
    }
}

fn dns_search(configs: &Vec<Config>, buf: &[u8]) -> Result<Vec<u8>, String> {
    let message = match Message::from_vec(&buf) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string())
    };
    // 根据规则获取查询 DNS 服务器地址
    let dns_addr = next_addr(&configs, &message)?;

    let mut resp_message = do_search(dns_addr, &message)?;
    resp_message.set_id(message.id());
    let resp = match resp_message.to_vec() {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    Ok(resp)
}

fn do_search(addr: &str, message: &Message) -> Result<Message, String> {
    let address = match addr.parse() {
        Ok(a) => a,
        Err(_) => return Err("addr parse error".to_string())
    };
    let conn = match UdpClientConnection::new(address) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };
    let client = SyncClient::new(conn);

    // TODO: 支持 DoT 和 DoH
    // 虽然格式允许一个 DNS 查询包含多个 question ，但在语义上来说，这样是有问题的
    // 因此，对于一个 DNS 查询来说，其第一个 question 就是唯一的一个 question
    // https://stackoverflow.com/questions/4082081/requesting-a-and-aaaa-records-in-single-dns-query/4083071#4083071
    let question = &message.queries()[0];
    let name = question.name();
    info!("dns search: name {}, dns server: {}", name, address);
    let response = match client.query(name, question.query_class(), question.query_type()) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };
    for message in response.messages() {
        return Ok(message.clone());
    }
    Err("Response no message".to_string())
}

fn next_addr<'a>(configs: &'a Vec<Config>, message: &Message) -> Result<&'a str, &'a str> {
    for question in message.queries().iter() {
        let config = Config::filter_rule(&configs, &question.name().to_string())?;
        return Ok(&config.dns);
    }
    Err("dns query must include question!")
}