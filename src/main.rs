use eframe::egui;
use lorenz_attractors::{Simulator, State, AttractorType};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Strange Attractors",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    simulator: Simulator,
    running: bool,
    rotation_x: f32,
    rotation_y: f32,
    zoom: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            simulator: Simulator::new(State {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            running: true,
            rotation_x: 0.3,
            rotation_y: 0.5,
            zoom: 1.0,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Strange Attractors");
        
        ui.horizontal(|ui| {
            if ui.button(if self.running { "⏸ Pause" } else { "▶ Resume" }).clicked() {
                self.running = !self.running;
            }
            if ui.button("🔄 Reset").clicked() {
                self.simulator.clear_trajectory();
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Attractor:");
            if ui.selectable_label(matches!(self.simulator.attractor, AttractorType::Lorenz), "Lorenz").clicked() {
                self.simulator.attractor = AttractorType::Lorenz;
                self.simulator.clear_trajectory();
            }
            if ui.selectable_label(matches!(self.simulator.attractor, AttractorType::Rossler), "Rössler").clicked() {
                self.simulator.attractor = AttractorType::Rossler;
                self.simulator.clear_trajectory();
            }
        });

        ui.label(format!(
            "Position: x={:.2}, y={:.2}, z={:.2}",
            self.simulator.state.x, self.simulator.state.y, self.simulator.state.z
        ));

        ui.label(format!("Trajectory points: {}", self.simulator.trajectory.len()));

        ui.separator();

        ui.group(|ui| {
            match self.simulator.attractor {
                AttractorType::Lorenz => {
                    ui.label("Lorenz Parameters:");
                    ui.horizontal(|ui| {
                        ui.label("σ:");
                        ui.add(egui::Slider::new(&mut self.simulator.lorenz_params.sigma, 0.0..=50.0).step_by(0.1));
                        ui.label(format!("{:.1}", self.simulator.lorenz_params.sigma));
                    });
                    ui.horizontal(|ui| {
                        ui.label("ρ:");
                        ui.add(egui::Slider::new(&mut self.simulator.lorenz_params.rho, 0.0..=50.0).step_by(0.1));
                        ui.label(format!("{:.1}", self.simulator.lorenz_params.rho));
                    });
                    ui.horizontal(|ui| {
                        ui.label("β:");
                        ui.add(egui::Slider::new(&mut self.simulator.lorenz_params.beta, 0.0..=10.0).step_by(0.01));
                        ui.label(format!("{:.3}", self.simulator.lorenz_params.beta));
                    });
                }
                AttractorType::Rossler => {
                    ui.label("Rössler Parameters:");
                    ui.horizontal(|ui| {
                        ui.label("a:");
                        ui.add(egui::Slider::new(&mut self.simulator.rossler_params.a, 0.0..=1.0).step_by(0.01));
                        ui.label(format!("{:.2}", self.simulator.rossler_params.a));
                    });
                    ui.horizontal(|ui| {
                        ui.label("b:");
                        ui.add(egui::Slider::new(&mut self.simulator.rossler_params.b, 0.0..=1.0).step_by(0.01));
                        ui.label(format!("{:.2}", self.simulator.rossler_params.b));
                    });
                    ui.horizontal(|ui| {
                        ui.label("c:");
                        ui.add(egui::Slider::new(&mut self.simulator.rossler_params.c, 0.0..=50.0).step_by(0.1));
                        ui.label(format!("{:.1}", self.simulator.rossler_params.c));
                    });
                }
            }
        });

        ui.separator();

        let canvas_size = egui::vec2(600.0, 600.0);
        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());
        let canvas_center = response.rect.center();

        let drag_delta = response.drag_delta();
        if drag_delta != egui::Vec2::ZERO {
            self.rotation_y += drag_delta.x * 0.01;
            self.rotation_x += drag_delta.y * 0.01;
        }

        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta != 0.0 {
            self.zoom *= 1.0 + (scroll_delta * 0.001);
            self.zoom = self.zoom.clamp(0.1, 10.0);
        }

        painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);

        if self.simulator.trajectory.len() > 1 {
            for i in 1..self.simulator.trajectory.len() {
                let p1 = &self.simulator.trajectory[i-1];
                let p2 = &self.simulator.trajectory[i];

                let (x1, y1) = Simulator::project_3d_to_2d(p1, self.rotation_x, self.rotation_y);
                let (x2, y2) = Simulator::project_3d_to_2d(p2, self.rotation_x, self.rotation_y);

                let scale = 50.0 * self.zoom;
                let screen_p1 = canvas_center + egui::vec2(x1 * scale, y1 * scale);
                let screen_p2 = canvas_center + egui::vec2(x2 * scale, y2 * scale);

                painter.line_segment([screen_p1, screen_p2], egui::Stroke::new(1.5, egui::Color32::GREEN));
            }
        }

        if self.running {
            for _ in 0..10 {
                self.simulator.step();
            }
            ui.ctx().request_repaint();
        }
    }
}
