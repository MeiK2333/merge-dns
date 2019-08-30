use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

fn recv(addr: SocketAddr) -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind(&addr)?;
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("{} {}", amt, src);
        for i in buf.iter() {
            println!("{}", i);
        }
    }
    Ok(())
}

fn main() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 53));
    loop {
        let _ = recv(addr).unwrap();
    }
}
