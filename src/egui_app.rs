use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use eframe::emath::Numeric;
use eframe::{egui, Frame, epaint};
use eframe::egui::{Context, Pos2};
use pinger::{Engine, Target};
use crate::{SLEEPTIMEP, TIMEOUT};

pub struct EguiApplication {
    engine: Engine,
    channel_tx: Sender<Box<Target>>,
    channel_rx: Receiver<Box<Target>>,

    add_dialog_open: bool,
    add_dialog_input: String
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
                self.add_dialog_open = true;
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

            let fill_color = match t.error_count() > 0 {
                true => egui::Color32::RED,
                false => egui::Color32::GREEN,
            };
            
            let s = format!("rtt: {:5} min/max/avg: {:^5}/{:^5}/{:^5}  errors: {:^6}    ",
                            n_o_d(t.last_rtt()), n_o_d(t.min_rtt()), n_o_d(t.max_rtt()),
                            n_o_d(t.avg_rtt()), t.error_count());
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
                        ui.monospace(egui::RichText::new(t.name()).color(egui::Color32::BLACK).size(24.0));
                        ui.monospace(egui::RichText::new(i).color(egui::Color32::BLACK).size(10.0));
                        ui.monospace(egui::RichText::new(s).color(egui::Color32::BLACK).size(10.0));
                        let target = self.engine.get(addr).unwrap().clone();
                        let history: Vec<Option<u32>> = target.hist_iter().copied().collect();

                        let graph_height = 20.0;
                        let graph_width = 200.0;
                        let scaled_height = target.max_rtt().unwrap_or(1) as f32 / graph_height;
                        let (response, painter) = ui.allocate_painter([graph_width, graph_height].into(), egui::Sense::focusable_noninteractive());
                        let rect = response.rect;

                        let mut last_p = Pos2::new(rect.min.x, rect.max.y);
                        let skip = if history.len() <= graph_width as usize {
                            0
                        }
                        else {
                            history.len() - graph_width as usize
                        };

                        for (x, h) in target.hist_iter().skip(skip).enumerate() {
                            let y = h.unwrap_or(0) as f32;
                            let p = Pos2::new((x as f32) + rect.min.x, rect.max.y - (y)/scaled_height);
                            let color = match h {
                                Some(_) => egui::Color32::BLUE,
                                None => egui::Color32::TRANSPARENT,
                            };
                            painter.line_segment([last_p, p], egui::Stroke::new(1.0, color));
                            last_p = p;
                        }

                });
            });
    }

    fn add_target_dialog(&mut self, ctx: &Context) {
        if self.add_dialog_open {
            egui::Modal::new("Add Target".into())
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
    }

    pub fn new() -> Self {
        let (channel_tx, channel_rx) = mpsc::channel();
        Self {
            engine: Engine::new(),
            channel_rx,
            channel_tx,
            add_dialog_open: false,
            add_dialog_input: String::new()
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.engine.add(target);
    }
}
