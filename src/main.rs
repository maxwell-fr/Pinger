use std::net::IpAddr;
use std::time::Duration;
use eframe::egui;
use toml;

use ping_rs::*;

use pinger::Target;
use crate::egui_app::EguiApplication;
use crate::config::Config;

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: u32 = 5;
const SLEEPTIMEP: u32 = 4000;

mod egui_app;
mod config;


fn main() -> std::result::Result<(), eframe::Error> {
    let mut conf = Config::default();
    let ip = IpAddr::from([1, 1, 1, 1]);
    println!("Pinging {ip}...");
    let data = [8; 8];

    let res = send_ping(&ip, Duration::from_millis(TIMEOUT.into()), &data, Some(&PING_OPTS));

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
        let friendly = format!("Tester {}", ip);
        let t = Target::new(friendly, ip,SLEEPTIMEP,TIMEOUT,50,10);
        conf.add_target(t.clone());
        pinger_app.add_target(t);
    }
    println!("{}", toml::to_string_pretty(&conf).unwrap());
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Pinger Test",
        options,
        Box::new(|_| Box::new(pinger_app)),
    )
}
