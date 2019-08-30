use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

use serde_json::Value;

use regex::Regex;

use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;

#[derive(Debug)]
struct Config {
    rule: String,
    dns: String,
    re: Regex,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = fs::read_to_string(&args[1]).expect("请选择正确的配置文件");
    let config: Value = serde_json::from_str(&config).expect("配置文件解析失败");
    let mut configs = Vec::new();

    if let Value::Array(conf) = config {
        for i in conf {
            if let Value::Array(rules) = &i["rules"] {
                for j in rules {
                    match (&i["dns"], &j) {
                        (Value::String(dns), Value::String(rule)) => {
                            configs.push(Config {
                                rule: rule.to_string(),
                                dns: dns.to_string(),
                                re: Regex::new(&rule.to_string()).unwrap(),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

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
