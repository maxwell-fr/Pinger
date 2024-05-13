use std::net::IpAddr;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use ping_rs::*;


const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(15);
const SLEEPTIME: Duration = Duration::from_secs(1);

#[derive(Debug)]
enum Status {
    GotReply(u32),
    Errored(u32)
}
struct Reply {
    status: Status,
    address: IpAddr
}

fn multiping() {
    let ips = vec![IpAddr::from([1,1,1,1]),
            IpAddr::from([8,8,8,8]),
            IpAddr::from([192,168,1,1]),
            IpAddr::from([192,168,1,2])];
    let (sender, receiver) = mpsc::channel();

    let mut threads = vec![];

    for ip in ips {
        let data = [8; 8];
        let sender = sender.clone();
        let thr = thread::spawn(move || {
            let mut error_count = 0;
            loop {
                let res = send_ping(&ip, TIMEOUT, &data, Some(&PING_OPTS));
                let status = match res {
                    Ok(r) => {
                        error_count = 0;
                        Status::GotReply(r.rtt)
                    },
                    Err(_) => {
                        error_count += 1;
                        Status::Errored(error_count)
                    }
                };
                let reply = Reply {
                    status,
                    address: ip,
                };

                if sender.send(reply).is_err() {
                    break;
                }
                thread::sleep(SLEEPTIME);
            }
        });
        threads.push(thr);
    }

    for reply in receiver {
        println!("{}: {:?}", reply.address, reply.status);
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
