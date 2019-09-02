use serde_json::Value;
use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug)]
pub struct Config {
    pub rule: String,
    pub dns: String,
    pub re: Regex,
}

impl Config {
    pub fn load() -> Vec<Config> {
        let args: Vec<String> = env::args().collect();
        let config = if args.len() <= 1 {
            String::from(
                r#"
[
    {
        "rules": [""],
        "dns": "8.8.8.8:53"
    }
]
"#,
            )
        } else {
            fs::read_to_string(&args[1]).expect("请选择正确的配置文件")
        };
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
        configs
    }
}
