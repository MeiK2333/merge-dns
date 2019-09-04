use crate::config::Config;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::rr::{DNSClass, RecordType};
use trust_dns::proto::op::message::Message;

pub fn server(configs: Vec<Config>) {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    let socket = UdpSocket::bind(&addr).unwrap();
    let mut buf = [0; 4096];
    loop {
        let (_amt, src) = match socket.recv_from(&mut buf) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("socket recv_from error: {}", e.to_string());
                continue;
            }
        };

        // 向第三方 DNS 服务器发送查询请求
        let resp_buf = match dns_search(&configs, &buf) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("dns_search error: {}", e);
                continue;
            }
        };
        // 将第三方的请求结果返回给请求者
        let _ = match socket.send_to(&resp_buf, src) {
            Ok(_) => println!("dns search success: src: {}", src),
            Err(e) => eprintln!("socket send_to error: {}", e.to_string())
        };
    }
}

fn dns_search(configs: &Vec<Config>, buf: &[u8]) -> Result<[u8; 4096], String> {
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

    let mut v: [u8; 4096] = [0; 4096];
    let mut cnt = 0;
    for i in resp.iter() {
        v[cnt] = *i;
        cnt += 1;
    }
    Ok(v)
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
    let name = message.queries()[0].name();

    // TODO: 支持更多请求方式
    // TODO: 支持 DoT 和 DoH
    let response = match client.query(name, DNSClass::IN, RecordType::A) {
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