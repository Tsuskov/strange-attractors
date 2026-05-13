use eframe::egui;
use lorenz_attractors::{LorenzSimulator, State};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lorenz Attractor",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    simulator: LorenzSimulator,
    running: bool,
    rotation_x: f32,
    rotation_y: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            simulator: LorenzSimulator::new(State {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            running: true,
            rotation_x: 0.3,
            rotation_y: 0.5,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Lorenz Attractor");
        
        ui.horizontal(|ui| {
            if ui.button(if self.running { "⏸ Pause" } else { "▶ Resume" }).clicked() {
                self.running = !self.running;
            }
            if ui.button("🔄 Reset").clicked() {
                self.simulator.clear_trajectory();
            }
        });

        ui.separator();

        ui.label(format!(
            "Position: x={:.2}, y={:.2}, z={:.2}",
            self.simulator.state.x, self.simulator.state.y, self.simulator.state.z
        ));

        ui.label(format!("Trajectory points: {}", self.simulator.trajectory.len()));

        ui.separator();

        ui.group(|ui| {
            ui.label("Parameters:");
            
            ui.horizontal(|ui| {
                ui.label("σ (sigma):");
                ui.add(egui::Slider::new(&mut self.simulator.params.sigma, 0.0..=50.0).step_by(0.1));
                ui.label(format!("{:.1}", self.simulator.params.sigma));
            });

            ui.horizontal(|ui| {
                ui.label("ρ (rho):");
                ui.add(egui::Slider::new(&mut self.simulator.params.rho, 0.0..=50.0).step_by(0.1));
                ui.label(format!("{:.1}", self.simulator.params.rho));
            });

            ui.horizontal(|ui| {
                ui.label("β (beta):");
                ui.add(egui::Slider::new(&mut self.simulator.params.beta, 0.0..=10.0).step_by(0.01));
                ui.label(format!("{:.3}", self.simulator.params.beta));
            });
        });

        ui.separator();

        let canvas_size = egui::vec2(600.0, 600.0);
        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());
        let canvas_center = response.rect.center();

        painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);

        if self.simulator.trajectory.len() > 1 {
            for i in 1..self.simulator.trajectory.len() {
                let p1 = &self.simulator.trajectory[i-1];
                let p2 = &self.simulator.trajectory[i];

                let (x1, y1) = lorenz_attractors::LorenzSimulator::project_3d_to_2d(p1, self.rotation_x, self.rotation_y);
                let (x2, y2) = lorenz_attractors::LorenzSimulator::project_3d_to_2d(p2, self.rotation_x, self.rotation_y);

                let screen_p1 = canvas_center + egui::vec2(x1 * 50.0, y1 * 50.0);
                let screen_p2 = canvas_center + egui::vec2(x2 * 50.0, y2 * 50.0);

                painter.line_segment([screen_p1, screen_p2], egui::Stroke::new(1.5, egui::Color32::GREEN));
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Rotation X:");
            ui.add(egui::Slider::new(&mut self.rotation_x, -std::f32::consts::PI..=std::f32::consts::PI));
            ui.label("Y:");
            ui.add(egui::Slider::new(&mut self.rotation_y, -std::f32::consts::PI..=std::f32::consts::PI));
        });

        if self.running {
            for _ in 0..10 {
                self.simulator.step();
            }
            ui.ctx().request_repaint();
        }
    }
}
