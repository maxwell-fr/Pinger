use std::net::IpAddr;
use std::time::Duration;
use eframe::egui;

use ping_rs::*;

use pinger::Target;
use crate::egui_app::EguiApplication;

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(5);
const SLEEPTIMEP: Duration = Duration::from_millis(4000);

mod egui_app;


fn main() -> std::result::Result<(), eframe::Error> {
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

    let ips = vec![IpAddr::from([1,1,1,1]),
                   IpAddr::from([8,8,8,8]),
                   IpAddr::from([192,168,1,1]),
                   IpAddr::from([20,50,166,83])];

    env_logger::init();
    let mut pinger_app = EguiApplication::new();
    for ip in ips {
        let friendly = format!("Tester {}", ip.to_string());
        pinger_app.add_target(Target::new(friendly, ip.clone(),SLEEPTIMEP,TIMEOUT,50,10));
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Pinger Test",
        options,
        Box::new(|_| Box::new(pinger_app)),
    )
}
