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

fn lorenz_derivatives(state: State, params: &LorenzParams) -> State {
    let dx = params.sigma * (state.y - state.x);
    let dy = state.x * (params.rho - state.z) - state.y;
    let dz = state.x * state.y - params.beta * state.z;
    State { x: dx, y: dy, z: dz }
}

fn rossler_derivatives(state: State, params: &RosslerParams) -> State {
    let dx = -state.y - state.z;
    let dy = state.x + params.a * state.y;
    let dz = params.b + state.z * (state.x - params.c);
    State { x: dx, y: dy, z: dz }
}

pub enum AttractorType {
    Lorenz,
    Rossler,
}

pub struct Simulator {
    pub state: State,
    pub lorenz_params: LorenzParams,
    pub rossler_params: RosslerParams,
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
            attractor: AttractorType::Lorenz,
            dt: 0.001,
            trajectory: Vec::with_capacity(10000),
        };
        sim.trajectory.push(initial_state);
        sim
    }

    pub fn step(&mut self) {
        let derivative = match self.attractor {
            AttractorType::Lorenz => lorenz_derivatives(self.state, &self.lorenz_params),
            AttractorType::Rossler => rossler_derivatives(self.state, &self.rossler_params),
        };

        let k1 = derivative;
        
        let state_2 = State {
            x: self.state.x + k1.x * self.dt / 2.0,
            y: self.state.y + k1.y * self.dt / 2.0,
            z: self.state.z + k1.z * self.dt / 2.0,
        };
        let k2 = match self.attractor {
            AttractorType::Lorenz => lorenz_derivatives(state_2, &self.lorenz_params),
            AttractorType::Rossler => rossler_derivatives(state_2, &self.rossler_params),
        };
        
        let state_3 = State {
            x: self.state.x + k2.x * self.dt / 2.0,
            y: self.state.y + k2.y * self.dt / 2.0,
            z: self.state.z + k2.z * self.dt / 2.0,
        };
        let k3 = match self.attractor {
            AttractorType::Lorenz => lorenz_derivatives(state_3, &self.lorenz_params),
            AttractorType::Rossler => rossler_derivatives(state_3, &self.rossler_params),
        };
        
        let state_4 = State {
            x: self.state.x + k3.x * self.dt,
            y: self.state.y + k3.y * self.dt,
            z: self.state.z + k3.z * self.dt,
        };
        let k4 = match self.attractor {
            AttractorType::Lorenz => lorenz_derivatives(state_4, &self.lorenz_params),
            AttractorType::Rossler => rossler_derivatives(state_4, &self.rossler_params),
        };

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

    pub fn project_3d_to_2d(point: &State, angle_x: f32, angle_y: f32) -> (f32, f32) {
        let x = point.x as f32;
        let y = point.y as f32;
        let z = point.z as f32;

        let cos_x = angle_x.cos();
        let sin_x = angle_x.sin();
        let cos_y = angle_y.cos();
        let sin_y = angle_y.sin();

        let y_rot = y * cos_x - z * sin_x;
        let z_rot = y * sin_x + z * cos_x;

        let x_rot = x * cos_y + z_rot * sin_y;
        let z_rot2 = -x * sin_y + z_rot * cos_y;

        let scale = 1.0 / (5.0 + z_rot2 * 0.1).max(0.5);
        (x_rot * scale, y_rot * scale)
    }
}
