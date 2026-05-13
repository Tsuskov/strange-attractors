# Lorenz Attractor

Interactive 3D visualization of the Lorenz attractor in Rust.

## Features
- **Real-time simulation** using RK4 integration
- **3D rendering** with rotatable view (X/Y axes)
- **Interactive parameters**: σ (sigma), ρ (rho), β (beta)
- **Green trajectory** showing the chaotic path

## Run
```bash
source $HOME/.cargo/env && cargo run
```

## Controls
- **Pause/Resume**: Toggle simulation
- **Reset**: Clear trajectory
- **Sliders**: Adjust Lorenz parameters in real-time
- **Rotation**: Rotate 3D view with X/Y sliders

## About
The Lorenz system is a set of three coupled differential equations that produce chaotic behavior. The resulting trajectory forms the famous "butterfly" shape—a strange attractor.

Default parameters (σ=10, ρ=28, β=8/3) produce the classic butterfly pattern.