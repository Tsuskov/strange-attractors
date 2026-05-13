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

fn lorenz_derivatives(state: State, params: &LorenzParams) -> State {
    let dx = params.sigma * (state.y - state.x);
    let dy = state.x * (params.rho - state.z) - state.y;
    let dz = state.x * state.y - params.beta * state.z;
    State { x: dx, y: dy, z: dz }
}

pub struct LorenzSimulator {
    pub state: State,
    pub params: LorenzParams,
    pub dt: f64,
    pub trajectory: Vec<State>,
}

impl LorenzSimulator {
    pub fn new(initial_state: State) -> Self {
        let mut sim = Self {
            state: initial_state,
            params: LorenzParams::default(),
            dt: 0.001,
            trajectory: Vec::with_capacity(10000),
        };
        sim.trajectory.push(initial_state);
        sim
    }

    pub fn step(&mut self) {
        let k1 = lorenz_derivatives(self.state, &self.params);
        
        let state_2 = State {
            x: self.state.x + k1.x * self.dt / 2.0,
            y: self.state.y + k1.y * self.dt / 2.0,
            z: self.state.z + k1.z * self.dt / 2.0,
        };
        let k2 = lorenz_derivatives(state_2, &self.params);
        
        let state_3 = State {
            x: self.state.x + k2.x * self.dt / 2.0,
            y: self.state.y + k2.y * self.dt / 2.0,
            z: self.state.z + k2.z * self.dt / 2.0,
        };
        let k3 = lorenz_derivatives(state_3, &self.params);
        
        let state_4 = State {
            x: self.state.x + k3.x * self.dt,
            y: self.state.y + k3.y * self.dt,
            z: self.state.z + k3.z * self.dt,
        };
        let k4 = lorenz_derivatives(state_4, &self.params);

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
