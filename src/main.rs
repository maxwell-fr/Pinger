use std::net::IpAddr;
use std::time::Duration;
use ping_rs::*;

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    let ip = IpAddr::from([1, 1, 1, 1]);
    println!("Pinging {ip}...");
    let data = [8; 8];

    let res = send_ping(&ip, TIMEOUT, &data, Some(&PING_OPTS));

    match res {
        Ok(r) => {
                println!("Success! rtt {:?}", r.rtt)
            }
        Err(_) => {
            println!("Failed!")
        }
    }
}
