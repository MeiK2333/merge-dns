use std::env;
use std::fs;

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = fs::read_to_string(&args[1]).expect("请选择正确的配置文件");
    let config: Value = serde_json::from_str(&config).expect("配置文件解析失败");

    if let Value::Array(conf) = config {
        for i in conf {
            print!("{}, ", i["dns"]);
            if let Value::Array(rule) = &i["rule"] {
                for j in rule {
                    print!("{}, ", j);
                }
            }
            println!("");
        }
    } else {
        panic!("parse config failure!");
    }
}
