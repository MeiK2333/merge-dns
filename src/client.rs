use crate::config;

use std::io::{self, BufRead};
use std::str::FromStr;

use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;

pub fn client_handle(configs: Vec<config::Config>) {
    let stdin = io::stdin();
    println!("请输入站点名：");
    for domain in stdin.lock().lines() {
        let domain = domain.unwrap();
        for config in configs.iter() {
            let result = config.re.is_match(&domain);
            if result {
                println!("使用 {} 进行查询", config.dns);
                let conn =
                    UdpClientConnection::new(config.dns.to_string().parse().unwrap()).unwrap();
                let client = SyncClient::new(conn);
                let name = Name::from_str(&domain.to_string()).unwrap();
                let response: DnsResponse =
                    client.query(&name, DNSClass::IN, RecordType::A).unwrap();
                let answers: &[Record] = response.answers();
                if answers.len() > 0 {
                    for ans in answers.iter() {
                        if let &RData::A(ref ip) = ans.rdata() {
                            println!("{}", ip);
                        }
                    }
                } else {
                    println!("无查询结果");
                }
                println!("查询完成\n");
                break;
            }
        }
    }
}
