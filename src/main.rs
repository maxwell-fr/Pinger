use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use std::net::IpAddr;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use ping_rs::*;
use pinger::Target;


const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(15);
const SLEEPTIME: Duration = Duration::from_secs(1);
const SLEEPTIME10: Duration = Duration::from_secs(10);



fn multiping() {
    let ips = vec![IpAddr::from([1,1,1,1]),
            IpAddr::from([8,8,8,8]),
            IpAddr::from([192,168,1,1]),
            IpAddr::from([192,168,1,2])];
    let (sender, receiver) = mpsc::channel();

    let mut threads = vec![];
    let mut targets: HashMap<IpAddr, Arc<Mutex<Target>>> = HashMap::new();

    for ip in ips {
        let data = [8; 8];
        let sender = sender.clone();
        let friendly = format!("Tester {}", ip.to_string());
        let target = Arc::new(Mutex::new(Target::new(friendly, ip.clone(),10,5,50,10)));
        targets.insert(ip.clone(), target.clone());
        let thr = thread::spawn(move || {
            loop {
                let res = send_ping(&ip, TIMEOUT, &data, Some(&PING_OPTS));

                target.lock().unwrap().update_from_reply(res.into());

                if sender.send(ip.clone()).is_err() {
                    break;
                }
                thread::sleep(SLEEPTIME10);
            }
        });
        threads.push(thr);
    }

    loop {
        for ip in receiver.try_iter() {
            let t = targets[&ip].lock().unwrap();
            println!("\n{}: {:?}", t.addr(), t);
        }
        print!("."); std::io::stdout().flush();
        thread::sleep(SLEEPTIME);
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
