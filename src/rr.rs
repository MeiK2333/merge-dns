pub struct Record {
    name: String,
    dns_type: String,
    class: String,
    ttl: u32,
    rdlength: u32,
    rdata: String,
}

impl Record {
    pub fn load(data: [u8; 512]) -> Record {
        Record {
            name: String::from(""),
            dns_type: String::from(""),
            class: String::from(""),
            ttl: 0,
            rdlength: 0,
            rdata: String::from(""),
        }
    }
}
