use crate::config::Config;

use std;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

pub fn server(configs: Vec<Config>) -> std::io::Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    let socket = UdpSocket::bind(&addr)?;
    loop {
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("ip: {}, len: {}", src, amt);
        let mut message = vec![];
        message.extend_from_slice(&buf);
        let message = String::from_utf8(message).unwrap();
        println!("{}", message);
    }
}
