use std::fs;
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
    let conf_str = fs::read_to_string("config").unwrap();
    let mut conf: Config = toml::from_str(&conf_str).unwrap();
    println!("{:?}", conf);
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

    env_logger::init();
    let mut pinger_app = EguiApplication::new();
    for targ in conf.targets() {
        pinger_app.add_target(targ.clone());
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
