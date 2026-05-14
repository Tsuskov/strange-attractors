# Lorenz Attractor

Interactive 3D visualization of the Lorenz attractor and now Rössler attractor in Rust.

## Features
- **Real-time simulation** using RK4 integration
- **3D rendering** with rotatable view 
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
- **Rotation**: Click and drag to rotate the view
- **Zoom**: Scroll to zoom in/out
- **Rössler Attractor**: Press 'R' to switch to Rössler attractor visualization

## About
The Lorenz system is a set of three coupled differential equations that produce chaotic behavior. The resulting trajectory forms the famous "butterfly" shape—a strange attractor.

Default parameters (σ=10, ρ=28, β=8/3) produce the classic butterfly pattern.

The Rössler attractor is another chaotic system defined by three equations, producing a different type of strange attractor. Press 'R' to switch between the two visualizations.
