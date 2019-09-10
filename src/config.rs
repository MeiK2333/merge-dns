use serde_json::Value;
use std::env;
use std::fs;

use regex::Regex;
use log::info;

#[derive(Debug)]
pub struct Config {
    pub rule: String,
    pub dns: String,
    pub re: Regex,
}

#[derive(Debug)]
pub struct Configs {
    configs: Vec<Config>,
}

impl Configs {
    pub fn load() -> Configs {
        let args: Vec<String> = env::args().collect();
        let config = if args.len() <= 1 {
            info!("Profile not provided, default configuration");
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
            info!("load CONFIG on `{}`", &args[1]);
            fs::read_to_string(&args[1]).expect("请选择正确的配置文件")
        };
        let config: Value = serde_json::from_str(&config).expect("配置文件解析失败");
        let mut configs = Configs {
            configs: Vec::new()
        };

        if let Value::Array(conf) = config {
            for i in conf {
                if let Value::Array(rules) = &i["rules"] {
                    for j in rules {
                        match (&i["dns"], &j) {
                            (Value::String(dns), Value::String(rule)) => {
                                configs.configs.push(Config {
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

    pub fn filter_rule(self, name: &str) -> Result<Config, &str> {
        for config in self.configs {
            if config.re.is_match(name) {
                return Ok(config);
            }
        }
        Err("rule not found!")
    }
}
