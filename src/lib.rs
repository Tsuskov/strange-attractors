#[derive(Clone, Copy, Debug)]
pub struct State {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct LorenzParams {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

pub struct RosslerParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Default for RosslerParams {
    fn default() -> Self {
        Self {
            a: 0.1,
            b: 0.1,
            c: 14.0,
        }
    }
}

pub struct ThomasParams {
    pub b: f64,
}

impl Default for ThomasParams {
    fn default() -> Self {
        Self { b: 0.208186 }
    }
}

fn lorenz_derivatives(state: State, params: &LorenzParams) -> State {
    let dx = params.sigma * (state.y - state.x);
    let dy = state.x * (params.rho - state.z) - state.y;
    let dz = state.x * state.y - params.beta * state.z;
    State {
        x: dx,
        y: dy,
        z: dz,
    }
}

fn rossler_derivatives(state: State, params: &RosslerParams) -> State {
    let dx = -state.y - state.z;
    let dy = state.x + params.a * state.y;
    let dz = params.b + state.z * (state.x - params.c);
    State {
        x: dx,
        y: dy,
        z: dz,
    }
}

fn thomas_derivatives(state: State, params: &ThomasParams) -> State {
    let dx = state.y.sin() - params.b * state.x;
    let dy = state.z.sin() - params.b * state.y;
    let dz = state.x.sin() - params.b * state.z;
    State {
        x: dx,
        y: dy,
        z: dz,
    }
}

#[derive(PartialEq)]
pub enum AttractorType {
    Lorenz,
    Rossler,
    Thomas,
}

pub struct Simulator {
    pub state: State,
    pub lorenz_params: LorenzParams,
    pub rossler_params: RosslerParams,
    pub thomas_params: ThomasParams,
    pub attractor: AttractorType,
    pub dt: f64,
    pub trajectory: Vec<State>,
}

impl Simulator {
    pub fn new(initial_state: State) -> Self {
        let mut sim = Self {
            state: initial_state,
            lorenz_params: LorenzParams::default(),
            rossler_params: RosslerParams::default(),
            thomas_params: ThomasParams::default(),
            attractor: AttractorType::Lorenz,
            dt: 0.001,
            trajectory: Vec::with_capacity(10000),
        };
        sim.trajectory.push(initial_state);
        sim
    }

    pub fn reset_for_attractor(&mut self) {
        self.state = match self.attractor {
            AttractorType::Lorenz => State {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            AttractorType::Rossler => State {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            AttractorType::Thomas => State {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        self.dt = match self.attractor {
            AttractorType::Thomas => 0.05,
            _ => 0.001,
        };
        self.clear_trajectory();
    }

    pub fn step(&mut self) {
        let derivatives = |s: State| match self.attractor {
            AttractorType::Lorenz => lorenz_derivatives(s, &self.lorenz_params),
            AttractorType::Rossler => rossler_derivatives(s, &self.rossler_params),
            AttractorType::Thomas => thomas_derivatives(s, &self.thomas_params),
        };

        let k1 = derivatives(self.state);

        let state_2 = State {
            x: self.state.x + k1.x * self.dt / 2.0,
            y: self.state.y + k1.y * self.dt / 2.0,
            z: self.state.z + k1.z * self.dt / 2.0,
        };
        let k2 = derivatives(state_2);

        let state_3 = State {
            x: self.state.x + k2.x * self.dt / 2.0,
            y: self.state.y + k2.y * self.dt / 2.0,
            z: self.state.z + k2.z * self.dt / 2.0,
        };
        let k3 = derivatives(state_3);

        let state_4 = State {
            x: self.state.x + k3.x * self.dt,
            y: self.state.y + k3.y * self.dt,
            z: self.state.z + k3.z * self.dt,
        };
        let k4 = derivatives(state_4);

        self.state.x += (k1.x + 2.0 * k2.x + 2.0 * k3.x + k4.x) * self.dt / 6.0;
        self.state.y += (k1.y + 2.0 * k2.y + 2.0 * k3.y + k4.y) * self.dt / 6.0;
        self.state.z += (k1.z + 2.0 * k2.z + 2.0 * k3.z + k4.z) * self.dt / 6.0;

        if self.trajectory.len() < 100000 {
            self.trajectory.push(self.state);
        }
    }

    pub fn clear_trajectory(&mut self) {
        self.trajectory.clear();
        self.trajectory.push(self.state);
    }

    pub fn project_3d_to_2d(
        point: &State,
        cos_x: f32,
        sin_x: f32,
        cos_y: f32,
        sin_y: f32,
    ) -> (f32, f32) {
        let x = point.x as f32;
        let y = point.y as f32;
        let z = point.z as f32;

        let y_rot = y * cos_x - z * sin_x;
        let z_rot = y * sin_x + z * cos_x;

        let x_rot = x * cos_y + z_rot * sin_y;
        let z_rot2 = -x * sin_y + z_rot * cos_y;

        let scale = 1.0 / (5.0 + z_rot2 * 0.1).max(0.5);
        (x_rot * scale, y_rot * scale)
    }
}
