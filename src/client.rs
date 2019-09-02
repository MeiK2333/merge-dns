use std::io::{self, BufRead};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn send(message: String, addr: SocketAddr, src: SocketAddr) -> std::io::Result<()> {
    let socket = UdpSocket::bind(&addr)?;
    let buf = message.as_bytes();
    socket.send_to(&buf, &src)?;

    Ok(())
}

fn main() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
    let src = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 53));
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let message = line.unwrap();
        let _ = send(message, addr, src).unwrap();
    }
}
