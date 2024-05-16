use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use eframe::{egui, Frame};
use eframe::egui::Context;
use pinger::{Pinger, Target};
use crate::{SLEEPTIMEP, TIMEOUT};

pub struct EguiApplication {
    pingers: Vec<Pinger>,
    targets: HashMap<IpAddr, Box<Target>>,
    channel_tx: Sender<Box<Target>>,
    channel_rx: Receiver<Box<Target>>,

    add_dialog_open: bool,
    add_dialog_input: String
}

impl eframe::App for EguiApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint_after(Duration::from_millis(250)); // force periodic UI redraws
        ctx.set_pixels_per_point(2.0); // todo: this should be configurable

        //fetch our target updates, if any
        for t in self.channel_rx.try_iter() {
            if self.targets.contains_key(&t.addr()) {
                self.targets.insert(t.addr(), t);
            }
        }

        if self.add_dialog_open {
            egui::Window::new("Add Target")
                .show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.add_dialog_input);

                    if ui.button("Add").clicked() {
                        if let Ok(new_ip) = IpAddr::from_str(&self.add_dialog_input) {
                            self.add_target(Target::new(self.add_dialog_input.clone(), new_ip,SLEEPTIMEP,TIMEOUT,50,10));
                            self.add_dialog_open = false;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.add_dialog_open = false;
                    }
                });
        }


        fn num_or_dashes(n: Option<u32>) -> String {
            match n {
                Some(r) => r.to_string(),
                None => "---".to_string()
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Add Target").clicked() {
                self.add_dialog_open = true;
            }
            let h = format!("{:30} {:4}  {:10}  {:10}  {:4}  {:4}  {:4}    {:16}","name", "rtt", "min", "max", "avg", "hist", "errs", "addr");
            ui.monospace(h);
            for (_, t) in &self.targets {
                let s = format!("{:30} {:4}  {:10}  {:10}  {:4}  {:4}  {:4}    {:16}",
                                t.name(), num_or_dashes(t.last_rtt()), num_or_dashes(t.min_rtt()), num_or_dashes(t.max_rtt()),
                                num_or_dashes(t.avg_rtt()),
                                t.hist_iter().count(), t.error_count(), t.addr().to_string());
                ui.monospace(s);
            }
        });
    }
}

impl EguiApplication {
    pub fn new() -> Self {
        let (channel_tx, channel_rx) = mpsc::channel();
        Self {
            pingers: vec![],
            targets: HashMap::new(),
            channel_rx,
            channel_tx,
            add_dialog_open: false,
            add_dialog_input: String::new()
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.targets.insert(target.addr(), Box::new(target.clone()));
        self.pingers.push(Pinger::new(target, self.channel_tx.clone()));
    }
}
