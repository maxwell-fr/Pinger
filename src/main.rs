use std::hash::Hash;
use std::io::Write;
use std::net::IpAddr;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use ping_rs::*;

use pinger::{Pinger, Target};

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(5);
const SLEEPTIME: Duration = Duration::from_secs(1);
const SLEEPTIMEP: Duration = Duration::from_millis(4000);



fn multiping() {
    let ips = vec![IpAddr::from([1,1,1,1]),
            IpAddr::from([8,8,8,8]),
            IpAddr::from([192,168,1,1]),
            IpAddr::from([20,50,166,83])];
    let (sender, receiver) = mpsc::channel();

    let mut pingers = vec![];

    for ip in ips {
        let sender = sender.clone();
        let friendly = format!("Tester {}", ip.to_string());
        pingers.push(Pinger::new(Target::new(friendly, ip.clone(),10,5,50,10), sender));
    }

    let mut ctr: u64 = 0;
    println!("{:12} {:4}  {:4}  {:4}  {:4}  {:4}  {:4}     {:16}","loops", "rtt", "min", "max", "avg", "hist", "errs", "addr");
    loop {
        let incoming = receiver.try_recv();
        match incoming {
            Ok(t) => {
                print!("{:012} ", ctr);
                print!("{:4}  {:4}  {:4}  {:4}  {:4}  {:4}", t.last_rtt(), t.min_rtt(), t.max_rtt(), t.avg_rtt(),
                       t.hist_iter().count(), t.error_count()); std::io::stdout().flush();
                println!("    {:16}", t.addr().to_string());
            }
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {println!("Disconnected?");}
        }
        std::io::stdout().flush();
        ctr += 1;
        thread::sleep(Duration::from_millis(1));
    }
}

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

    multiping();
}
