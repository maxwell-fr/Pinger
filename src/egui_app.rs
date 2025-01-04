use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use eframe::{egui, Frame, epaint};
use eframe::egui::{Context, Pos2};
use pinger::{Engine, Target};
use crate::{SLEEPTIMEP, TIMEOUT};

#[derive(Default)]
struct AddTargetDialog {
    is_open: bool,
    name: String,
    ip: String,
    interval: String
}


pub struct EguiApplication {
    engine: Engine,
    channel_tx: Sender<Box<Target>>,
    channel_rx: Receiver<Box<Target>>,

    add_dialog: AddTargetDialog
}

/// helper function to simplify displaying option numbers
fn n_o_d(n: Option<u32>) -> String {
    match n {
        Some(r) => r.to_string(),
        None => "---".to_string()
    }
}

impl eframe::App for EguiApplication {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint_after(Duration::from_millis(250)); // force periodic UI redraws
        ctx.set_pixels_per_point(2.0); //TODO: this should be configurable

        self.add_target_dialog(ctx);

        egui::TopBottomPanel::top("TopBar")
        .show(ctx, |ui| {
            if ui.button("Add Target").clicked() {
                self.add_dialog.is_open = true;
            }
        });

        egui::CentralPanel::default()
        .show(ctx, |ui| {
            for a in &self.engine.get_addresses() {
                self.pinger_data(a, ctx);
            }
        });
    }
}
impl EguiApplication {
    fn pinger_data(&mut self, addr: &IpAddr, ctx: &Context) {
            let t = self.engine.get(addr);
            if t.is_none() {
                return;
            }
            let t = t.unwrap();
            let error_val = t.error_count_threshold();
            let warn_val = error_val / 2;

            let fill_color = match t.error_count_recent(t.error_count_threshold() as usize) {
                c if (0 .. warn_val).contains(&c) => egui::Color32::GREEN,
                c if (warn_val .. error_val).contains(&c) => egui::Color32::YELLOW,
                _ => egui::Color32::RED,
            };
            
            let s = format!("rtt: {:5} min/max/avg: {:^5}/{:^5}/{:^5}",
                            n_o_d(t.last_rtt()), n_o_d(t.min_rtt()), n_o_d(t.max_rtt()),
                            n_o_d(t.avg_rtt()));
            let e = format!("err_t: {:^5} err_r: {:^5}",
                            t.error_count(), t.error_count_recent(t.error_count_threshold() as usize));
            let i = format!("ip: {}", t.addr());


            let area = egui::Area::new(addr.to_string().into());
            area
            .show(ctx, |ui| {
                egui::Frame {
                        inner_margin: 5.0.into(),
                        outer_margin: 0.0.into(),
                        rounding: 0.0.into(),
                        fill: fill_color,
                        stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                        shadow: epaint::Shadow::NONE
                }.show(ui, |ui| {
                        ui.monospace(egui::RichText::new(t.name()).color(egui::Color32::BLACK).size(18.0));
                        ui.monospace(egui::RichText::new(i).color(egui::Color32::BLACK).size(10.0));
                        ui.monospace(egui::RichText::new(s).color(egui::Color32::BLACK).size(10.0));
                        ui.monospace(egui::RichText::new(e).color(egui::Color32::BLACK).size(10.0));
                        let target = self.engine.get(addr).unwrap().clone();
                        let history: Vec<Option<u32>> = target.hist_iter().copied().collect();

                        let graph_height = 30.0;
                        let graph_width = 256.0;
                        let scaled_height = target.max_rtt().unwrap_or(1) as f32 / graph_height;
                        let (response, painter) = ui.allocate_painter([graph_width, graph_height].into(), egui::Sense::focusable_noninteractive());
                        let rect = response.rect;

                        let skip = match history.len() <= graph_width as usize {
                            true => 0,
                            false => history.len() - graph_width as usize,
                        };

                        let mut last_p = Pos2::new(rect.min.x, rect.max.y);

                        for (x, h) in target.hist_iter().skip(skip).enumerate() {
                            let y = h.unwrap_or(0) as f32;
                            let p = Pos2::new((x as f32) + rect.min.x, rect.max.y - (y)/scaled_height);
                            let stroke = match h {
                                Some(_) => egui::Stroke::new(1.0, egui::Color32::BLUE),
                                None => egui::Stroke::new(1.0, egui::Color32::TRANSPARENT),
                            };
                            painter.line_segment([last_p, p], stroke);
                            last_p = p;
                        }

                });
            });
    }

    fn add_target_dialog(&mut self, ctx: &Context) {
        if self.add_dialog.is_open {
            egui::Modal::new("Add Target".into())
                .show(ctx, |ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.add_dialog.name);
                    ui.label("IP Address");
                    ui.text_edit_singleline(&mut self.add_dialog.ip);

                    if ui.button("Add").clicked() {
                        if let Ok(new_ip) = IpAddr::from_str(&self.add_dialog.ip) {
                            self.add_target(Target::new(self.add_dialog.name.clone(), new_ip,SLEEPTIMEP,TIMEOUT,50,10));
                            self.add_dialog.is_open = false;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.add_dialog.is_open = false;
                    }
                });
        }
    }

    pub fn new() -> Self {
        let (channel_tx, channel_rx) = mpsc::channel();
        Self {
            engine: Engine::new(),
            channel_rx,
            channel_tx,
            add_dialog: AddTargetDialog::default()
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.engine.add(target);
    }
}
