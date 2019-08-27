use std::net::Ipv4Addr;
use std::str::FromStr;

use serde_json::{Result, Value};
use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};

fn main() {
    let config = r#"
    [
        {
            "rule": "*.youlocal.com",
            "dns": "10.2.1.10"
        },
        {
            "rule": "*",
            "dns": "8.8.8.8"
        }
    ]
    "#;
    let config: Value = serde_json::from_str(config).unwrap();
    if let Value::Array(conf) = config {
        for i in conf {
            println!("{}", i["rule"]);
        }
    } else {
        panic!("parse config failure!");
    }

/*
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    let name = Name::from_str("www.example.com.").unwrap();

    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();

    let answers: &[Record] = response.answers();
    if let &RData::A(ref ip) = answers[0].rdata() {
        assert_eq!(*ip, Ipv4Addr::new(93, 184, 216, 34))
    } else {
        assert!(false, "unexpected result")
    }
*/
}
