use eframe::egui;
use strange_attractors::{AttractorType, Simulator, State};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 900.0]),
        ..Default::default()
    };
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

fn attractor_color(index: usize, total: usize) -> egui::Color32 {
    let t = index as f32 / total as f32;
    let r = (0.5 + 0.5 * (t * std::f32::consts::TAU).cos()) * 255.0;
    let g = (0.5 + 0.5 * (t * std::f32::consts::TAU + 2.0).cos()) * 255.0;
    let b = (0.5 + 0.5 * (t * std::f32::consts::TAU + 4.0).cos()) * 255.0;
    egui::Color32::from_rgb(r as u8, g as u8, b as u8)
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Strange Attractors");

            ui.horizontal(|ui| {
                if ui
                    .button(if self.running {
                        "⏸ Pause"
                    } else {
                        "▶ Resume"
                    })
                    .clicked()
                {
                    self.running = !self.running;
                }
                if ui.button("🔄 Reset").clicked() {
                    self.simulator.clear_trajectory();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Attractor:");
                if ui
                    .selectable_label(
                        matches!(self.simulator.attractor, AttractorType::Lorenz),
                        "Lorenz",
                    )
                    .clicked()
                {
                    self.simulator.attractor = AttractorType::Lorenz;
                    self.simulator.reset_for_attractor();
                    self.zoom = 1.0;
                }
                if ui
                    .selectable_label(
                        matches!(self.simulator.attractor, AttractorType::Rossler),
                        "Rössler",
                    )
                    .clicked()
                {
                    self.simulator.attractor = AttractorType::Rossler;
                    self.simulator.reset_for_attractor();
                    self.zoom = 1.0;
                }
                if ui
                    .selectable_label(
                        matches!(self.simulator.attractor, AttractorType::Thomas),
                        "Thomas",
                    )
                    .clicked()
                {
                    self.simulator.attractor = AttractorType::Thomas;
                    self.simulator.reset_for_attractor();
                    self.zoom = 3.0;
                }
            });

            ui.label(format!(
                "Position: x={:.3}, y={:.3}, z={:.3}",
                self.simulator.state.x, self.simulator.state.y, self.simulator.state.z
            ));
            ui.label(format!(
                "Trajectory points: {}",
                self.simulator.trajectory.len()
            ));

            ui.separator();

            ui.group(|ui| match self.simulator.attractor {
                AttractorType::Lorenz => {
                    ui.label("Lorenz Parameters:");
                    ui.horizontal(|ui| {
                        ui.label("σ:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.lorenz_params.sigma, 0.0..=50.0)
                                .step_by(0.1),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("ρ:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.lorenz_params.rho, 0.0..=50.0)
                                .step_by(0.1),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("β:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.lorenz_params.beta, 0.0..=10.0)
                                .step_by(0.01),
                        );
                    });
                }
                AttractorType::Rossler => {
                    ui.label("Rössler Parameters:");
                    ui.horizontal(|ui| {
                        ui.label("a:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.rossler_params.a, 0.0..=1.0)
                                .step_by(0.01),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("b:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.rossler_params.b, 0.0..=1.0)
                                .step_by(0.01),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("c:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.rossler_params.c, 0.0..=50.0)
                                .step_by(0.1),
                        );
                    });
                }
                AttractorType::Thomas => {
                    ui.label("Thomas Parameters:");
                    ui.label("Classic chaos: b ≈ 0.208  |  Less chaos: b → 0.3");
                    ui.horizontal(|ui| {
                        ui.label("b:");
                        ui.add(
                            egui::Slider::new(&mut self.simulator.thomas_params.b, 0.1..=0.4)
                                .step_by(0.001),
                        );
                    });
                }
            });

            ui.separator();

            let available = ui.available_size();
            let canvas_size = egui::vec2(available.x, available.y.min(550.0));
            let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::drag());
            let canvas_center = response.rect.center();

            let drag_delta = response.drag_delta();
            if drag_delta != egui::Vec2::ZERO {
                self.rotation_y += drag_delta.x * 0.01;
                self.rotation_x += drag_delta.y * 0.01;
            }

            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                self.zoom *= 1.0 + scroll_delta * 0.001;
                self.zoom = self.zoom.clamp(0.1, 10.0);
            }

            painter.rect_filled(response.rect, 4.0, egui::Color32::from_rgb(10, 10, 20));

            let traj = &self.simulator.trajectory;
            if traj.len() > 1 {
                let cos_x = self.rotation_x.cos();
                let sin_x = self.rotation_x.sin();
                let cos_y = self.rotation_y.cos();
                let sin_y = self.rotation_y.sin();
                let scale = 50.0 * self.zoom;
                let total = traj.len();

                let projected: Vec<egui::Pos2> = traj
                    .iter()
                    .map(|p| {
                        let (px, py) =
                            Simulator::project_3d_to_2d(p, cos_x, sin_x, cos_y, sin_y);
                        canvas_center + egui::vec2(px * scale, py * scale)
                    })
                    .collect();

                for i in 1..total {
                    let color = attractor_color(i, total);
                    painter.line_segment(
                        [projected[i - 1], projected[i]],
                        egui::Stroke::new(1.2, color),
                    );
                }
            }

            if self.running {
                let steps = match self.simulator.attractor {
                    AttractorType::Thomas => 3,
                    _ => 10,
                };
                for _ in 0..steps {
                    self.simulator.step();
                }
                ui.ctx().request_repaint();
            }
        });
    }
}
