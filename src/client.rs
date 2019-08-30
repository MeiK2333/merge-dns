use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn send(addr: SocketAddr, src: SocketAddr) -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind(&addr)?;
        let mut buf = [0; 10];
        buf[0] = 1;
        buf[1] = 2;
        buf[2] = 3;
        buf[3] = 4;
        buf[4] = 5;
        buf[9] = 9;
        socket.send_to(&buf, &src)?;
    }
    Ok(())
}

fn main() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
    let src = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 53));
    let _ = send(addr, src);
}
