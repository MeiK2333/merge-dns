use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn recv(addr: SocketAddr) -> std::io::Result<()> {
    let socket = UdpSocket::bind(&addr)?;
    let mut buf = [0; 512];
    let (amt, src) = socket.recv_from(&mut buf)?;
    println!("ip: {}, len: {}", src, amt);
    let mut message = vec![];
    message.extend_from_slice(&buf);
    let message = String::from_utf8(message).unwrap();
    println!("{}", message);

    Ok(())
}

fn main() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    loop {
        let _ = recv(addr).unwrap();
    }
}
