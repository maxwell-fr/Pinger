use std::net::IpAddr;
use std::time::Duration;

fn main() {
    let ip = IpAddr::from([1,1,1,1]);
    println!("Pinging {ip}...");

    let timeout = Some(Duration::new(5,0));
    let res = ping::ping(ip, timeout, None, None, None, None);

    println!("Success? {}", res.is_ok());
}
