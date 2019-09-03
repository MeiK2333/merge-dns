use crate::config::Config;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use dns_parser::Packet;

pub fn server(configs: Vec<Config>) {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    let socket = UdpSocket::bind(&addr).unwrap();
    let mut buf = [0; 4096];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("ip: {}, len: {}", src, amt);
        let n_addr = next_addr(&configs, &buf).unwrap();
        println!("next addr: {}", n_addr);
    }
}

pub fn next_addr<'a>(configs: &'a Vec<Config>, buf: &[u8]) -> Result<&'a str, &'a str> {
    let packet = Packet::parse(&buf).unwrap();

    for question in packet.questions.iter() {
        let config = Config::filter_rule(&configs, &question.qname.to_string())?;
        return Ok(&config.dns)
    }
    Err("dns query must include question!")
}