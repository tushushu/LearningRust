use std::string::String;

fn main() {
    construct_ip("V4", String::from("127.0.0.1"));
    construct_ip("V6", String::from("::1"));
    construct_ip("V7", String::from("foo"));
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn construct_ip(kind: &str, ip_address: String) -> IpAddrKind {
    let result = {
        if kind == "V4" {
            IpAddrKind::V4(ip_address)
        } else if kind == "V6" {
            IpAddrKind::V6(ip_address)
        } else {
            panic!("Invalid Value! Parameter kind cannot be {}!", kind);
        }
    };
    println!("Ip address {:?} constructed!", result);
    return result;
}
