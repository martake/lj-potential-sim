use eframe::egui;
use egui::{Color32, Pos2, Stroke, Vec2};

use crate::params::SimParams;
use crate::simulation::SimState;

pub struct SimulatorApp {
    sim: SimState,
    params: SimParams,
}

impl SimulatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let params = SimParams::default();
        let sim = SimState::new(params.atom_count, Vec2::new(800.0, 600.0));
        Self { sim, params }
    }
}

impl eframe::App for SimulatorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("params_panel")
            .default_size(250.0)
            .show_inside(ui, |ui| {
                ui.heading("シミュレーション パラメータ");
                ui.add_space(4.0);

                let prev_count = self.params.atom_count;
                ui.add(
                    egui::Slider::new(&mut self.params.atom_count, 2..=30)
                        .text("Atom Count"),
                );
                if self.params.atom_count != prev_count {
                    self.sim.reset(self.params.atom_count);
                }

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.params.r_zone, 10.0..=80.0)
                        .text("R_zone"),
                );
                ui.add(
                    egui::Slider::new(&mut self.params.r_shield, 20.0..=150.0)
                        .text("R_shield"),
                );

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.params.repulsion_strength, 100.0..=20000.0)
                        .logarithmic(true)
                        .text("Repulsion Strength"),
                );
                ui.add(
                    egui::Slider::new(&mut self.params.attraction_strength, 1.0..=500.0)
                        .logarithmic(true)
                        .text("Attraction Strength"),
                );

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut self.params.electron_noise, 0.0..=10.0)
                        .text("Electron Noise"),
                );
                ui.add(
                    egui::Slider::new(&mut self.params.damping, 0.9..=1.0)
                        .text("Damping"),
                );
                ui.add(
                    egui::Slider::new(&mut self.params.dt, 0.001..=0.05)
                        .text("Timestep (dt)"),
                );

                ui.separator();

                if ui
                    .button(if self.params.paused {
                        "Resume"
                    } else {
                        "Pause"
                    })
                    .clicked()
                {
                    self.params.paused = !self.params.paused;
                }

                if ui.button("Reset").clicked() {
                    self.sim.reset(self.params.atom_count);
                }
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available = ui.available_size();
            self.sim.bounds = available;

            if !self.params.paused {
                self.sim.update(&self.params);
            }

            let painter = ui.painter();
            let origin = ui.min_rect().min;

            for atom in self.sim.atoms() {
                let center = Pos2::new(origin.x + atom.pos.x, origin.y + atom.pos.y);

                // Positive shield
                painter.circle_filled(
                    center,
                    self.params.r_shield,
                    Color32::from_rgba_unmultiplied(0, 100, 255, 20),
                );

                // Electron zone ring
                painter.circle_stroke(
                    center,
                    self.params.r_zone,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(100, 200, 255, 60)),
                );

                // Nucleus
                painter.circle_filled(center, 3.0, Color32::from_rgb(255, 60, 60));

                // Electron position
                let electron_pos =
                    Pos2::new(center.x + atom.electron_offset.x, center.y + atom.electron_offset.y);

                // Electron glow
                painter.circle_filled(
                    electron_pos,
                    6.0,
                    Color32::from_rgba_unmultiplied(0, 255, 255, 40),
                );

                // Electron core
                painter.circle_filled(
                    electron_pos,
                    2.5,
                    Color32::from_rgba_unmultiplied(0, 255, 255, 200),
                );
            }
        });

        ui.ctx().request_repaint();
    }
}
