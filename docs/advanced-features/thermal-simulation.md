# Physics-Based Thermal Simulation

## Overview

Hardware Tool includes a physics-based thermal simulation engine for evaluating PCB thermal performance. The system models Joule heating from electrical currents, heat conduction through board materials, and temperature distribution across components—displayed as interactive heat maps.

### Problem Statement

Simulate thermal effects from electrical currents on PCB components, traces, and vias using fundamental physics laws:

| Law | Equation | Description |
|-----|----------|-------------|
| **Joule Heating** | $P = I^2 R$ | Power dissipation from current through resistance |
| **Fourier's Conduction** | $q = -k \nabla T$ | Heat flux proportional to temperature gradient |
| **Heat Equation** | $\frac{\partial T}{\partial t} = \kappa \nabla^2 T + \frac{Q}{c_p \rho}$ | Transient temperature evolution |

### Solution Architecture

Pure Rust implementation using:

- **Symbolica** — Symbolic equation derivation for heat sources and diffusion
- **fenris** — Finite Element Method (FEM) library for accurate thermal field solving
- **ndarray** — Efficient multi-dimensional grid computations
- **nalgebra** — Linear algebra for matrix assembly and solving
- **plotters** — Heat map visualization (static PNG/SVG)
- **Bevy** — Interactive 3D heat map overlay in EDA canvas

## Rust Crate Dependencies

```toml
[dependencies]
fenris = "0.8"          # Advanced FEM library for thermal PDEs
symbolica = "0.9"       # Computer Algebra System for symbolic derivation
ndarray = "0.15"        # Multi-dimensional arrays for grid data
nalgebra = "0.33"       # Linear algebra for FEM assembly/solvers
plotters = "0.3"        # 2D heatmap visualization
rayon = "1.10"          # Optional parallel assembly
```

### Why This Stack?

| Crate | Purpose | Advantage |
|-------|---------|-----------|
| **fenris** | FEM solver | Handles complex PCB geometries, higher-order elements, hp-refinement |
| **Symbolica** | Symbolic math | Derive/optimize equations before numerical solve |
| **ndarray** | Grid data | Battle-tested scientific computing, parallelizable |
| **nalgebra** | Linear algebra | Fast matrix ops, sparse solver support |
| **plotters** | Visualization | Lightweight, no GUI overhead, Bevy-compatible |

## Physics Fundamentals

### Joule Heating (Heat Source)

Current flowing through resistive elements generates heat:

```
P = I² × R

Where:
  P = Power dissipation (W)
  I = Current (A)
  R = Resistance (Ω)
```

For volumetric heat generation:

```
Q = P / V = I² × R / V   (W/m³)
```

### Heat Conduction (Fourier's Law)

Heat flows from hot to cold regions:

```
q = -k × ∇T

Where:
  q = Heat flux (W/m²)
  k = Thermal conductivity (W/m·K)
  ∇T = Temperature gradient (K/m)
```

### Transient Heat Equation

Temperature evolution over time:

```
∂T/∂t = κ × ∇²T + Q/(ρ × cₚ)

Where:
  T = Temperature (K or °C)
  t = Time (s)
  κ = Thermal diffusivity = k/(ρ × cₚ) (m²/s)
  ∇²T = Laplacian (spatial second derivative)
  Q = Volumetric heat source (W/m³)
  ρ = Density (kg/m³)
  cₚ = Specific heat capacity (J/kg·K)
```

### Steady-State (Poisson Equation)

For long-term operation (∂T/∂t = 0):

```
∇²T = -Q / k
```

## Material Properties

### Common PCB Materials

| Material | k (W/m·K) | ρ (kg/m³) | cₚ (J/kg·K) | κ (m²/s) |
|----------|-----------|-----------|-------------|----------|
| **FR4** | 0.3 | 1,900 | 1,200 | 1.3×10⁻⁷ |
| **Copper** | 385 | 8,960 | 385 | 1.1×10⁻⁴ |
| **Aluminum** | 205 | 2,700 | 900 | 8.4×10⁻⁵ |
| **Solder (Sn63)** | 50 | 8,400 | 150 | 4.0×10⁻⁵ |
| **Air** | 0.026 | 1.2 | 1,005 | 2.2×10⁻⁵ |

### Configuration

```rust
MaterialProperties {
    // FR4 substrate
    fr4: ThermalMaterial {
        conductivity: 0.3,        // W/(m·K)
        density: 1900.0,          // kg/m³
        specific_heat: 1200.0,    // J/(kg·K)
    },
    
    // Copper traces/planes
    copper: ThermalMaterial {
        conductivity: 385.0,
        density: 8960.0,
        specific_heat: 385.0,
    },
    
    // Component packages (typical)
    component_package: ThermalMaterial {
        conductivity: 1.0,        // Epoxy/plastic
        density: 1800.0,
        specific_heat: 1000.0,
    },
}
```

## Simulation Methods

### Finite Difference Method (FDM)

Simple grid-based approach for regular geometries:

```rust
use ndarray::Array2;
use nalgebra::DMatrix;

// Constants
const KAPPA: f64 = 0.1;           // Thermal diffusivity (m²/s)
const TEMP_THRESHOLD: f64 = 85.0; // Overheat limit (°C)
const AMBIENT_TEMP: f64 = 25.0;   // Room temperature

fn fdm_thermal_solve(
    nx: usize,              // Grid size X
    ny: usize,              // Grid size Y
    dx: f64,                // Spatial step (m)
    dt: f64,                // Time step (s)
    heat_sources: &Array2<f64>,  // Q values (W/m³)
    time_steps: usize,
) -> Array2<f64> {
    let mut temp = Array2::<f64>::from_elem((nx, ny), AMBIENT_TEMP);
    
    // Stability criterion: dt < dx² / (4 × κ)
    let stability = dx * dx / (4.0 * KAPPA);
    assert!(dt < stability, "Time step too large for stability");
    
    for _ in 0..time_steps {
        let mut temp_new = temp.clone();
        
        for i in 1..nx-1 {
            for j in 1..ny-1 {
                // 5-point stencil Laplacian
                let laplacian = (
                    temp[[i+1, j]] + temp[[i-1, j]] +
                    temp[[i, j+1]] + temp[[i, j-1]] -
                    4.0 * temp[[i, j]]
                ) / (dx * dx);
                
                // Heat equation update
                temp_new[[i, j]] = temp[[i, j]] + 
                    dt * (KAPPA * laplacian + heat_sources[[i, j]]);
            }
        }
        temp = temp_new;
    }
    
    temp
}
```

### Finite Element Method (FEM) with fenris

Advanced approach for complex PCB geometries:

```rust
use fenris::assembly::*;
use fenris::element::*;
use fenris::mesh::*;
use fenris::quadrature::*;
use nalgebra::{DMatrix, DVector};

fn fem_thermal_solve(
    mesh: &QuadMesh2d<f64>,
    material: &ThermalMaterial,
    heat_sources: &[f64],
    dt: f64,
    time_steps: usize,
) -> Vec<f64> {
    let element = Quad4Element::default();
    let quadrature = GaussQuadrature::default();
    
    // Assemble system matrices
    let mut assembler = FenrisAssembler::new(mesh, &element, &quadrature);
    
    // Stiffness matrix: K (conduction)
    let k_matrix = assembler.assemble_stiffness(|_| material.conductivity);
    
    // Mass matrix: M (transient term)
    let rho_cp = material.density * material.specific_heat;
    let m_matrix = assembler.assemble_mass(|_| rho_cp);
    
    // Load vector: f (heat sources)
    let load_vector = assembler.assemble_source(|_| heat_sources.iter().cloned());
    
    // Initial temperature
    let mut temp = vec![AMBIENT_TEMP; mesh.num_vertices()];
    
    // Implicit Euler time integration
    // (M + dt·K) × T_new = M × T_old + dt × f
    for _ in 0..time_steps {
        let a = &m_matrix + dt * &k_matrix;
        let b = &m_matrix * DVector::from_vec(temp.clone()) + dt * &load_vector;
        
        let t_new = a.lu().solve(&b).expect("Solve failed");
        temp = t_new.as_slice().to_vec();
    }
    
    temp
}
```

### Steady-State Solution

For long-term operation analysis (24/7 for years):

```rust
fn steady_state_solve(
    mesh: &QuadMesh2d<f64>,
    material: &ThermalMaterial,
    heat_sources: &[f64],
) -> Vec<f64> {
    let element = Quad4Element::default();
    let quadrature = GaussQuadrature::default();
    
    let mut assembler = FenrisAssembler::new(mesh, &element, &quadrature);
    let k_matrix = assembler.assemble_stiffness(|_| material.conductivity);
    let load_vector = assembler.assemble_source(|_| heat_sources.iter().cloned());
    
    // Solve Poisson equation: K × T = f
    let temp = k_matrix.lu().solve(&load_vector).expect("Solve failed");
    
    temp.as_slice().to_vec()
}
```

## Symbolic Equation Derivation

Use Symbolica for deriving and optimizing thermal equations:

```rust
use symbolica::prelude::*;

fn derive_joule_heating() {
    let mut state = State::new();
    
    // Define variables
    let i = state.get_or_insert_var("I");  // Current
    let r = state.get_or_insert_var("R");  // Resistance
    let v = state.get_or_insert_var("V");  // Volume
    
    // Joule heating: P = I² × R
    let power = Atom::new_var(i).pow(2) * Atom::new_var(r);
    
    // Volumetric heat: Q = P / V
    let q_volumetric = power / Atom::new_var(v);
    
    println!("Power: {}", power.printer(&state).print());
    println!("Volumetric heat: {}", q_volumetric.printer(&state).print());
}

fn derive_heat_equation() {
    let mut state = State::new();
    
    // Thermal properties
    let k = state.get_or_insert_var("k");      // Conductivity
    let rho = state.get_or_insert_var("rho");  // Density
    let cp = state.get_or_insert_var("cp");    // Specific heat
    
    // Thermal diffusivity: κ = k / (ρ × cₚ)
    let kappa = Atom::new_var(k) / (Atom::new_var(rho) * Atom::new_var(cp));
    
    println!("Thermal diffusivity: {}", kappa.printer(&state).print());
}
```

## Heat Map Visualization

### Static PNG Generation

```rust
use plotters::prelude::*;
use ndarray::Array2;

fn generate_heatmap(
    temp_grid: &Array2<f64>,
    filename: &str,
    min_temp: f64,
    max_temp: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ny, nx) = temp_grid.dim();
    
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("PCB Thermal Distribution", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..nx as f32, 0f32..ny as f32)?;
    
    chart.configure_mesh().draw()?;
    
    // Draw temperature cells
    chart.draw_series(
        temp_grid.indexed_iter().map(|((y, x), &temp)| {
            // Normalize temperature to 0-1 range
            let normalized = (temp - min_temp) / (max_temp - min_temp);
            let normalized = normalized.clamp(0.0, 1.0);
            
            // Color: Blue (cold) → Green → Yellow → Red (hot)
            let hue = 240.0 * (1.0 - normalized);  // 240° = blue, 0° = red
            
            Rectangle::new(
                [(x as f32, y as f32), ((x + 1) as f32, (y + 1) as f32)],
                HSLColor(hue / 360.0, 1.0, 0.5).filled(),
            )
        }),
    )?;
    
    // Add color bar legend
    let legend_area = root.margin(10, 10, 50, 10);
    // ... draw gradient legend with temperature scale
    
    root.present()?;
    Ok(())
}
```

### Bevy Integration (Interactive 3D)

```rust
use bevy::prelude::*;

#[derive(Component)]
struct ThermalOverlay {
    temperatures: Vec<f64>,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
}

fn update_thermal_overlay(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&ThermalOverlay>,
    thermal_sim: Res<ThermalSimulation>,
) {
    for overlay in query.iter() {
        if let Some(material) = materials.get_mut(&overlay.material_handle) {
            // Update vertex colors based on temperature
            let max_temp = thermal_sim.temperatures.iter()
                .cloned().fold(f64::MIN, f64::max);
            
            for (i, &temp) in thermal_sim.temperatures.iter().enumerate() {
                let normalized = (temp - 25.0) / (max_temp - 25.0);
                let color = temperature_to_color(normalized);
                // Apply to mesh vertex colors
            }
        }
    }
}

fn temperature_to_color(normalized: f64) -> Color {
    // Thermal colormap: blue → cyan → green → yellow → red
    let hue = 240.0 * (1.0 - normalized.clamp(0.0, 1.0));
    Color::hsl(hue as f32, 1.0, 0.5)
}
```

## Simulation Configuration

### Tick Rate Adjustment

Balance accuracy vs. computation speed:

```rust
SimulationConfig {
    // Time stepping
    tick_rate: TickRate {
        dt: 0.01,                    // Time step (s)
        mode: TickMode::Adaptive,    // Fixed, Adaptive, or Steady
        
        // Stability: dt < dx² / (4 × κ)
        auto_stability: true,
    },
    
    // Grid resolution
    resolution: Resolution {
        min_element_size: 0.1,       // mm
        max_element_size: 2.0,       // mm
        refinement_near_sources: true,
    },
    
    // Solver
    solver: SolverConfig {
        method: SolverMethod::ImplicitEuler,  // Stable for large dt
        tolerance: 1e-6,
        max_iterations: 1000,
    },
}
```

### Tick Rate Guidelines

| Scenario | Recommended dt | Notes |
|----------|----------------|-------|
| **Fine transient** | 1e-4 to 1e-3 s | Accurate switching behavior |
| **Normal transient** | 1e-2 to 1e-1 s | General thermal response |
| **Fast preview** | 1.0 to 10.0 s | Quick what-if analysis |
| **Steady-state** | ∞ (direct solve) | Long-term operation |

### Stability Criterion

For explicit methods:

```
dt < dx² / (4 × κ)

Example (FR4, dx = 1mm):
  κ = 1.3×10⁻⁷ m²/s
  dt < (0.001)² / (4 × 1.3×10⁻⁷)
  dt < 1.9 s
```

Implicit methods (recommended) are unconditionally stable.

## Long-Run Simulation Presets

### Preset Configurations

```rust
LongRunPresets {
    // Quick check (minutes)
    quick: Preset {
        duration: Duration::minutes(30),
        method: Method::Transient,
        dt: 1.0,
    },
    
    // Daily operation
    daily: Preset {
        duration: Duration::hours(24),
        method: Method::Transient,
        dt: 60.0,  // 1-minute steps
    },
    
    // Weekly stress test
    weekly: Preset {
        duration: Duration::days(7),
        method: Method::ScaledTransient,
        dt: 3600.0,  // 1-hour steps
    },
    
    // Continuous operation (years)
    continuous: Preset {
        duration: Duration::years(5),
        method: Method::SteadyState,  // Direct Poisson solve
        dt: f64::INFINITY,
    },
}
```

### Long-Run API

```rust
// Simulate 24/7 operation for 5 years
fn simulate_long_run(
    pcb: &PcbThermalModel,
    years: f64,
    ambient_temp: f64,
) -> LongRunResult {
    // Use steady-state for efficiency
    let steady_temps = steady_state_solve(
        &pcb.mesh,
        &pcb.materials,
        &pcb.heat_sources,
    );
    
    // Find maximum temperature
    let max_temp = steady_temps.iter().cloned().fold(f64::MIN, f64::max);
    
    // Check against threshold
    let safe = max_temp <= TEMP_THRESHOLD;
    
    LongRunResult {
        max_temperature: max_temp,
        safe_for_duration: safe,
        hotspot_locations: find_hotspots(&steady_temps, &pcb.mesh),
        recommendations: generate_recommendations(max_temp, &pcb),
    }
}

// Preset function
fn simulate_24_7(years: u32, ambient: f64) -> (bool, f64) {
    let result = simulate_long_run(&current_pcb(), years as f64, ambient);
    (result.safe_for_duration, result.max_temperature)
}
```

## Overheat Detection

### Threshold Configuration

```rust
OverheatConfig {
    // Temperature thresholds
    thresholds: Thresholds {
        warning: 70.0,           // °C - yellow alert
        critical: 85.0,          // °C - red alert (typical max)
        absolute_max: 125.0,     // °C - component damage
    },
    
    // Component-specific limits
    component_limits: vec![
        ComponentLimit { ref_des: "U1", max_temp: 85.0 },   // MCU
        ComponentLimit { ref_des: "Q1", max_temp: 150.0 },  // MOSFET
        ComponentLimit { ref_des: "C*", max_temp: 105.0 },  // Capacitors
    ],
    
    // Actions
    on_warning: Action::Highlight,
    on_critical: Action::Alert,
    on_exceed: Action::StopSimulation,
}
```

### Overheat Check API

```rust
fn check_overheating(
    temperatures: &[f64],
    mesh: &Mesh,
    config: &OverheatConfig,
) -> OverheatReport {
    let mut report = OverheatReport::default();
    
    for (node_idx, &temp) in temperatures.iter().enumerate() {
        let position = mesh.node_position(node_idx);
        
        if temp > config.thresholds.critical {
            report.critical_zones.push(HotSpot {
                position,
                temperature: temp,
                exceeds_by: temp - config.thresholds.critical,
            });
        } else if temp > config.thresholds.warning {
            report.warning_zones.push(HotSpot {
                position,
                temperature: temp,
                exceeds_by: temp - config.thresholds.warning,
            });
        }
    }
    
    report.max_temperature = temperatures.iter().cloned().fold(f64::MIN, f64::max);
    report.safe = report.critical_zones.is_empty();
    
    report
}
```

### Overheat Report UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Thermal Simulation Report                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Simulation: 24/7 operation, 5 years (steady-state)             │
│ Ambient: 25°C                                                   │
│                                                                 │
│ Results:                                                        │
│   Maximum temperature: 78.3°C                                   │
│   Location: U1 (MCU) center                                     │
│   Status: ✓ SAFE (below 85°C threshold)                        │
│                                                                 │
│ Temperature Distribution:                                       │
│   < 40°C:  45% of board area                                   │
│   40-60°C: 35% of board area                                   │
│   60-80°C: 18% of board area                                   │
│   > 80°C:   2% of board area                                   │
│                                                                 │
│ Hot Spots:                                                      │
│   1. U1 (MCU): 78.3°C - within limit (85°C)                    │
│   2. Q1 (MOSFET): 72.1°C - within limit (150°C)                │
│   3. VRM area: 65.4°C - OK                                     │
│                                                                 │
│ Recommendations:                                                │
│   ⚠ Consider thermal vias under U1 for 10°C reduction         │
│   ✓ Current design safe for continuous operation               │
│                                                                 │
│ [View Heatmap] [Export Report] [Add Thermal Vias] [Close]       │
└─────────────────────────────────────────────────────────────────┘
```

## Complete Implementation Example

### Full Thermal Simulation Module

```rust
use fenris::assembly::*;
use fenris::element::*;
use fenris::mesh::*;
use fenris::quadrature::*;
use symbolica::prelude::*;
use ndarray::Array2;
use nalgebra::{DMatrix, DVector};
use plotters::prelude::*;
use std::error::Error;

// ═══════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════

const TEMP_THRESHOLD: f64 = 85.0;  // °C
const AMBIENT_TEMP: f64 = 25.0;    // °C

// ═══════════════════════════════════════════════════════════════
// Material Properties
// ═══════════════════════════════════════════════════════════════

struct ThermalMaterial {
    conductivity: f64,    // W/(m·K)
    density: f64,         // kg/m³
    specific_heat: f64,   // J/(kg·K)
}

impl ThermalMaterial {
    fn diffusivity(&self) -> f64 {
        self.conductivity / (self.density * self.specific_heat)
    }
    
    fn fr4() -> Self {
        Self { conductivity: 0.3, density: 1900.0, specific_heat: 1200.0 }
    }
    
    fn copper() -> Self {
        Self { conductivity: 385.0, density: 8960.0, specific_heat: 385.0 }
    }
}

// ═══════════════════════════════════════════════════════════════
// Symbolic Derivation
// ═══════════════════════════════════════════════════════════════

fn symbolic_joule_heat() {
    let mut state = State::new();
    let i = state.get_or_insert_var("I");
    let r = state.get_or_insert_var("R");
    
    // P = I² × R
    let power = Atom::new_var(i).pow(2) * Atom::new_var(r);
    println!("Symbolic Joule heat: {}", power.printer(&state).print());
}

// ═══════════════════════════════════════════════════════════════
// FDM Solver (Simple Grid)
// ═══════════════════════════════════════════════════════════════

fn fdm_solve(
    nx: usize,
    ny: usize,
    dx: f64,
    dt: f64,
    heat_sources: &Array2<f64>,
    material: &ThermalMaterial,
    time_steps: usize,
) -> (Array2<f64>, bool) {
    let kappa = material.diffusivity();
    let mut temp = Array2::<f64>::from_elem((ny, nx), AMBIENT_TEMP);
    let mut overheated = false;
    
    for step in 0..time_steps {
        let mut temp_new = temp.clone();
        
        for i in 1..ny-1 {
            for j in 1..nx-1 {
                let laplacian = (
                    temp[[i+1, j]] + temp[[i-1, j]] +
                    temp[[i, j+1]] + temp[[i, j-1]] -
                    4.0 * temp[[i, j]]
                ) / (dx * dx);
                
                temp_new[[i, j]] = temp[[i, j]] + 
                    dt * (kappa * laplacian + heat_sources[[i, j]]);
            }
        }
        
        temp = temp_new;
        
        // Check overheat
        if let Some(&max_t) = temp.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            if max_t > TEMP_THRESHOLD {
                overheated = true;
                println!("Overheats at step {} ({:.1}s): {:.1}°C", 
                    step, step as f64 * dt, max_t);
                break;
            }
        }
    }
    
    (temp, overheated)
}

// ═══════════════════════════════════════════════════════════════
// Steady-State Solver
// ═══════════════════════════════════════════════════════════════

fn steady_state_fdm(
    nx: usize,
    ny: usize,
    dx: f64,
    heat_sources: &Array2<f64>,
    material: &ThermalMaterial,
) -> Array2<f64> {
    let n = nx * ny;
    let k = material.conductivity;
    
    // Build Laplacian matrix (sparse in production)
    let mut lap = DMatrix::<f64>::zeros(n, n);
    let mut rhs = DVector::<f64>::zeros(n);
    
    for i in 0..ny {
        for j in 0..nx {
            let idx = i * nx + j;
            
            if i == 0 || i == ny-1 || j == 0 || j == nx-1 {
                // Boundary: fixed at ambient
                lap[(idx, idx)] = 1.0;
                rhs[idx] = AMBIENT_TEMP;
            } else {
                // Interior: Laplacian stencil
                lap[(idx, idx)] = 4.0;
                lap[(idx, idx - 1)] = -1.0;
                lap[(idx, idx + 1)] = -1.0;
                lap[(idx, idx - nx)] = -1.0;
                lap[(idx, idx + nx)] = -1.0;
                
                // Source term: -Q × dx² / k
                rhs[idx] = heat_sources[[i, j]] * dx * dx / k;
            }
        }
    }
    
    // Solve: Lap × T = rhs
    let temp_flat = lap.lu().solve(&rhs).expect("Solve failed");
    
    Array2::from_shape_vec((ny, nx), temp_flat.as_slice().to_vec()).unwrap()
}

// ═══════════════════════════════════════════════════════════════
// Heat Map Generation
// ═══════════════════════════════════════════════════════════════

fn generate_heatmap(
    grid: &Array2<f64>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let (ny, nx) = grid.dim();
    let min_temp = grid.iter().cloned().fold(f64::MAX, f64::min);
    let max_temp = grid.iter().cloned().fold(f64::MIN, f64::max);
    
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Thermal Map ({:.1}°C - {:.1}°C)", min_temp, max_temp),
            ("sans-serif", 24),
        )
        .margin(20)
        .build_cartesian_2d(0f32..nx as f32, 0f32..ny as f32)?;
    
    chart.draw_series(
        grid.indexed_iter().map(|((y, x), &temp)| {
            let normalized = (temp - min_temp) / (max_temp - min_temp + 1e-6);
            let hue = 240.0 * (1.0 - normalized);  // Blue → Red
            
            Rectangle::new(
                [(x as f32, y as f32), ((x + 1) as f32, (y + 1) as f32)],
                HSLColor(hue / 360.0, 1.0, 0.5).filled(),
            )
        }),
    )?;
    
    root.present()?;
    println!("Heatmap saved to {}", filename);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// Main Entry Point
// ═══════════════════════════════════════════════════════════════

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Hardware Tool Thermal Simulation ===\n");
    
    // 1. Symbolic derivation
    symbolic_joule_heat();
    
    // 2. Setup grid (20×20 board, 1mm resolution)
    let nx = 20;
    let ny = 20;
    let dx = 0.001;  // 1mm in meters
    let dt = 0.1;    // 100ms tick
    
    // 3. Define heat sources (component at center)
    let mut heat_sources = Array2::<f64>::zeros((ny, nx));
    heat_sources[[ny/2, nx/2]] = 50.0;      // MCU: 50 W/m³
    heat_sources[[ny/2 + 2, nx/2]] = 30.0;  // MOSFET: 30 W/m³
    
    let material = ThermalMaterial::fr4();
    
    // 4. Transient simulation (1000 steps = 100s)
    println!("\n--- Transient Simulation ---");
    let (temp_transient, overheated) = fdm_solve(
        nx, ny, dx, dt, &heat_sources, &material, 1000
    );
    
    if !overheated {
        println!("Transient: Safe (max {:.1}°C)", 
            temp_transient.iter().cloned().fold(f64::MIN, f64::max));
    }
    generate_heatmap(&temp_transient, "transient_heatmap.png")?;
    
    // 5. Steady-state (24/7 for years)
    println!("\n--- Steady-State (Long-Run) ---");
    let temp_steady = steady_state_fdm(nx, ny, dx, &heat_sources, &material);
    let max_steady = temp_steady.iter().cloned().fold(f64::MIN, f64::max);
    
    if max_steady > TEMP_THRESHOLD {
        println!("Steady-state: OVERHEATS at {:.1}°C!", max_steady);
    } else {
        println!("Steady-state: Safe for 24/7 operation ({:.1}°C)", max_steady);
    }
    generate_heatmap(&temp_steady, "steady_heatmap.png")?;
    
    // 6. Preset: 5-year continuous
    println!("\n--- 5-Year Continuous Operation ---");
    let (safe, max_temp) = simulate_24_7(5, AMBIENT_TEMP);
    println!("Result: {} (max {:.1}°C)", 
        if safe { "SAFE" } else { "FAILS" }, max_temp);
    
    Ok(())
}

fn simulate_24_7(years: u32, ambient: f64) -> (bool, f64) {
    // Use steady-state for long-run
    let nx = 20;
    let ny = 20;
    let dx = 0.001;
    
    let mut heat_sources = Array2::<f64>::zeros((ny, nx));
    heat_sources[[ny/2, nx/2]] = 50.0;
    
    let material = ThermalMaterial::fr4();
    let temp = steady_state_fdm(nx, ny, dx, &heat_sources, &material);
    let max_temp = temp.iter().cloned().fold(f64::MIN, f64::max);
    
    (max_temp <= TEMP_THRESHOLD, max_temp)
}
```

## Radiation Heat Transfer

Thermal radiation is the third mode of heat transfer (alongside conduction and convection), critical for accurate PCB thermal analysis—especially for high-power components, enclosed systems, and space/vacuum applications.

### Stefan-Boltzmann Law

Radiative heat flux from a surface:

```
q_rad = ε × σ × (T⁴ - T_surr⁴)

Where:
  q_rad = Radiative heat flux (W/m²)
  ε = Surface emissivity (0-1, dimensionless)
  σ = Stefan-Boltzmann constant = 5.670374419 × 10⁻⁸ W/(m²·K⁴)
  T = Surface temperature (K)
  T_surr = Surrounding temperature (K)
```

### Surface Emissivity Database

| Material | Emissivity (ε) | Notes |
|----------|----------------|-------|
| **Polished copper** | 0.03 | Bare traces (rare) |
| **Oxidized copper** | 0.65 | Typical aged traces |
| **HASL solder** | 0.05 | Shiny finish |
| **ENIG gold** | 0.02 | Very low emissivity |
| **FR4 (green solder mask)** | 0.85-0.92 | High emissivity |
| **Black solder mask** | 0.95 | Near-blackbody |
| **White solder mask** | 0.88 | Slightly lower |
| **IC package (plastic)** | 0.80-0.95 | Depends on color |
| **IC package (ceramic)** | 0.90 | High emissivity |
| **Aluminum heatsink (anodized)** | 0.80-0.90 | Good radiator |
| **Aluminum heatsink (polished)** | 0.05 | Poor radiator |

### Radiation Configuration

```rust
RadiationConfig {
    enabled: true,
    
    // Stefan-Boltzmann constant
    sigma: 5.670374419e-8,  // W/(m²·K⁴)
    
    // Ambient/surrounding temperature
    t_surrounding: 298.15,  // K (25°C)
    
    // Surface properties
    surfaces: vec![
        RadiativeSurface {
            region: Region::SolderMask,
            emissivity: 0.90,
            area_factor: 1.0,  // Projected area multiplier
        },
        RadiativeSurface {
            region: Region::Component("U1"),
            emissivity: 0.85,
            area_factor: 1.2,  // Account for package fins
        },
        RadiativeSurface {
            region: Region::Heatsink("HS1"),
            emissivity: 0.85,  // Anodized aluminum
            area_factor: 5.0,  // Fin area multiplier
        },
    ],
    
    // View factor calculation
    view_factors: ViewFactorMethod::MonteCarlo {
        rays: 100_000,
        seed: 42,
    },
}
```

### View Factor Calculation

View factors (F_ij) determine how much radiation leaving surface i reaches surface j:

```
Reciprocity: A_i × F_ij = A_j × F_ji
Summation:   Σ F_ij = 1 (for enclosure)
```

#### View Factor Methods

| Method | Accuracy | Speed | Use Case |
|--------|----------|-------|----------|
| **Analytical** | Exact | Fast | Simple geometries (parallel plates) |
| **Contour Integration** | High | Medium | 2D/extruded shapes |
| **Monte Carlo Ray Tracing** | High | Slow | Complex 3D geometries |
| **Hemicube** | Medium | Fast | Real-time approximation |

```rust
ViewFactorCalculator {
    method: ViewFactorMethod::MonteCarlo,
    
    // Monte Carlo settings
    monte_carlo: MonteCarloConfig {
        rays_per_surface: 100_000,
        stratified_sampling: true,
        variance_threshold: 0.001,
    },
    
    // Hemicube settings (for real-time)
    hemicube: HemicubeConfig {
        resolution: 512,  // pixels per face
        gpu_accelerated: true,
    },
    
    // Caching
    cache_view_factors: true,
    cache_file: "view_factors.bin",
}
```

### Enclosure Radiation (Radiosity Method)

For multi-surface enclosures, solve the radiosity equation:

```
J_i = ε_i × σ × T_i⁴ + (1 - ε_i) × Σ(F_ij × J_j)

Where:
  J_i = Radiosity of surface i (W/m²)
  ε_i = Emissivity of surface i
  F_ij = View factor from i to j
```

Matrix form: **[I - (1-ε)F] × J = ε × σ × T⁴**

```rust
fn solve_radiosity(
    surfaces: &[Surface],
    view_factors: &DMatrix<f64>,
    temperatures: &[f64],  // Kelvin
) -> Vec<f64> {
    let n = surfaces.len();
    let sigma = 5.670374419e-8;
    
    // Build coefficient matrix
    let mut a = DMatrix::<f64>::identity(n, n);
    for i in 0..n {
        let rho_i = 1.0 - surfaces[i].emissivity;  // Reflectivity
        for j in 0..n {
            a[(i, j)] -= rho_i * view_factors[(i, j)];
        }
    }
    
    // Build RHS: ε × σ × T⁴
    let b: DVector<f64> = DVector::from_iterator(n, 
        surfaces.iter().zip(temperatures.iter()).map(|(s, &t)| {
            s.emissivity * sigma * t.powi(4)
        })
    );
    
    // Solve for radiosity
    let j = a.lu().solve(&b).expect("Radiosity solve failed");
    
    // Net heat flux: q_i = (J_i - Σ F_ij × J_j) / (1 - ε_i) × ε_i
    // Or simplified: q_i = ε_i / (1 - ε_i) × (σ T_i⁴ - J_i)
    j.as_slice().to_vec()
}
```

### Coupled Conduction-Radiation Solver

Iteratively solve conduction and radiation until convergence:

```rust
fn coupled_thermal_solve(
    mesh: &Mesh,
    materials: &Materials,
    heat_sources: &[f64],
    radiation_config: &RadiationConfig,
    max_iterations: usize,
    tolerance: f64,
) -> ThermalResult {
    let mut temperatures = vec![radiation_config.t_surrounding; mesh.num_nodes()];
    
    for iteration in 0..max_iterations {
        let temp_old = temperatures.clone();
        
        // 1. Solve conduction with current radiation heat flux as BC
        let radiation_flux = calculate_radiation_flux(&temperatures, radiation_config);
        temperatures = solve_conduction(mesh, materials, heat_sources, &radiation_flux);
        
        // 2. Check convergence
        let max_change = temperatures.iter()
            .zip(temp_old.iter())
            .map(|(t_new, t_old)| (t_new - t_old).abs())
            .fold(0.0, f64::max);
        
        if max_change < tolerance {
            return ThermalResult {
                temperatures,
                converged: true,
                iterations: iteration + 1,
            };
        }
    }
    
    ThermalResult {
        temperatures,
        converged: false,
        iterations: max_iterations,
    }
}
```

### Radiation in Vacuum/Space Applications

For space electronics (no convection):

```rust
SpaceEnvironmentConfig {
    // No convection
    convection_enabled: false,
    
    // Radiation dominant
    radiation: RadiationConfig {
        enabled: true,
        
        // Deep space background
        t_surrounding: 2.7,  // K (cosmic microwave background)
        
        // Solar flux (if sun-facing)
        solar_flux: Some(SolarFlux {
            intensity: 1361.0,  // W/m² at 1 AU
            absorptivity: 0.3,  // Surface solar absorptance
            direction: Vector3::new(1.0, 0.0, 0.0),
        }),
        
        // Earth IR (if in orbit)
        earth_ir: Some(EarthIR {
            intensity: 237.0,  // W/m² average
            view_factor_to_earth: 0.5,
        }),
        
        // Albedo
        albedo: Some(Albedo {
            factor: 0.3,
            solar_intensity: 1361.0,
        }),
    },
}
```

## Scientific Accuracy Enhancements

To achieve scientific-grade accuracy (<5% error vs. experimental data):

### Accuracy Levels

| Level | Error | Mesh | Materials | Validation | Use Case |
|-------|-------|------|-----------|------------|----------|
| **Preview** | 30-50% | Coarse | Generic | None | Quick what-if |
| **Design** | 10-30% | Medium | Datasheet | Spot-check | Design iteration |
| **Engineering** | 5-10% | Fine | Measured | Partial | Detailed analysis |
| **Scientific** | <5% | Adaptive | Characterized | Full | Publication/certification |

### Mesh Refinement Strategy

```rust
MeshRefinement {
    // Adaptive refinement
    adaptive: AdaptiveRefinement {
        enabled: true,
        
        // Error estimator
        estimator: ErrorEstimator::ZienkiewiczZhu,
        
        // Refinement criteria
        max_error: 0.01,           // 1% local error threshold
        max_gradient: 10.0,        // K/mm temperature gradient
        
        // Refinement near features
        refine_near_sources: true,
        refine_near_boundaries: true,
        refine_near_material_interfaces: true,
        
        // Limits
        min_element_size: 0.05,    // mm
        max_element_size: 2.0,     // mm
        max_refinement_levels: 5,
    },
    
    // hp-refinement (higher-order elements)
    hp_refinement: HpRefinement {
        enabled: true,
        max_polynomial_order: 4,
        smoothness_indicator: true,
    },
}
```

### Material Property Database

Scientific-grade material characterization:

```rust
MaterialDatabase {
    // Temperature-dependent properties
    temperature_dependent: true,
    
    // FR4 with full characterization
    fr4: TemperatureDependentMaterial {
        // Conductivity: k(T) polynomial fit
        conductivity: Polynomial {
            coefficients: [0.29, 0.0002, -1e-7],  // W/(m·K)
            valid_range: (233.0, 423.0),          // K
        },
        
        // Specific heat: cp(T)
        specific_heat: Polynomial {
            coefficients: [1100.0, 0.5, 0.001],   // J/(kg·K)
            valid_range: (233.0, 423.0),
        },
        
        // Density (constant for solids)
        density: 1900.0,  // kg/m³
        
        // Anisotropic conductivity (in-plane vs through-plane)
        anisotropic: Some(AnisotropicConductivity {
            k_xy: 0.8,   // W/(m·K) in-plane (with copper)
            k_z: 0.3,    // W/(m·K) through-plane
        }),
        
        // Uncertainty
        uncertainty: MaterialUncertainty {
            conductivity_std: 0.05,  // ±5% standard deviation
            specific_heat_std: 0.03,
        },
        
        // Source/reference
        source: "IPC-TM-650 2.4.24.6",
        measured_date: "2025-06-15",
    },
    
    // Copper with temperature dependence
    copper: TemperatureDependentMaterial {
        conductivity: Polynomial {
            // k decreases with temperature
            coefficients: [401.0, -0.0685, 0.0],  // W/(m·K)
            valid_range: (200.0, 500.0),
        },
        specific_heat: Polynomial {
            coefficients: [385.0, 0.0, 0.0],
            valid_range: (200.0, 500.0),
        },
        density: 8960.0,
        anisotropic: None,
        uncertainty: MaterialUncertainty {
            conductivity_std: 0.02,
            specific_heat_std: 0.02,
        },
        source: "NIST SRD 8",
        measured_date: "Reference",
    },
}
```

### Boundary Condition Accuracy

```rust
BoundaryConditions {
    // Convection (Newton's law of cooling)
    convection: ConvectionBC {
        enabled: true,
        
        // Heat transfer coefficient
        h: ConvectionCoefficient::Calculated {
            // Nusselt correlation for natural convection
            correlation: NusseltCorrelation::ChurchillChu,
            
            // Fluid properties (air at 25°C)
            fluid: FluidProperties {
                conductivity: 0.026,      // W/(m·K)
                viscosity: 1.85e-5,       // Pa·s
                density: 1.184,           // kg/m³
                specific_heat: 1005.0,    // J/(kg·K)
                prandtl: 0.71,
                beta: 0.00341,            // 1/K thermal expansion
            },
            
            // Geometry
            characteristic_length: 0.05,  // m (board dimension)
            orientation: Orientation::HorizontalUp,
        },
        
        // Or measured/specified
        // h: ConvectionCoefficient::Specified(10.0),  // W/(m²·K)
    },
    
    // Radiation (as detailed above)
    radiation: RadiationBC { /* ... */ },
    
    // Contact resistance
    contact_resistance: ContactResistanceBC {
        enabled: true,
        interfaces: vec![
            ContactInterface {
                surface_a: "component_bottom",
                surface_b: "pcb_top",
                resistance: 0.5,  // K·cm²/W (thermal grease)
            },
            ContactInterface {
                surface_a: "heatsink_base",
                surface_b: "component_top",
                resistance: 0.1,  // K·cm²/W (thermal pad)
            },
        ],
    },
}
```

### Validation Framework

```rust
ValidationFramework {
    // Experimental comparison
    experimental: ExperimentalValidation {
        // Thermocouple measurements
        thermocouples: vec![
            Measurement { location: (10.0, 15.0, 0.0), measured: 72.3, uncertainty: 0.5 },
            Measurement { location: (25.0, 20.0, 0.0), measured: 65.1, uncertainty: 0.5 },
            Measurement { location: (40.0, 30.0, 0.0), measured: 58.7, uncertainty: 0.5 },
        ],
        
        // IR camera data
        ir_camera: Some(IRCameraData {
            image_path: "thermal_ir.png",
            calibration: IRCalibration {
                emissivity_assumed: 0.95,
                reflected_temp: 25.0,
                distance: 0.3,
            },
        }),
    },
    
    // Analytical benchmarks
    analytical: AnalyticalBenchmarks {
        // Compare against known solutions
        benchmarks: vec![
            Benchmark::PointSourceInfinitePlate,
            Benchmark::FinEfficiency,
            Benchmark::SteadyState1D,
        ],
    },
    
    // Mesh convergence study
    mesh_convergence: MeshConvergenceStudy {
        enabled: true,
        refinement_levels: vec![1.0, 0.5, 0.25, 0.125],  // Element size multipliers
        convergence_metric: ConvergenceMetric::MaxTemperature,
        target_change: 0.01,  // 1% change threshold
    },
    
    // Grid Independence Index (GCI)
    gci: GridConvergenceIndex {
        enabled: true,
        safety_factor: 1.25,  // For 3+ grids
        target_gci: 0.05,     // 5% GCI
    },
}
```

### Uncertainty Quantification

```rust
UncertaintyQuantification {
    enabled: true,
    
    // Monte Carlo uncertainty propagation
    monte_carlo: MonteCarloUQ {
        samples: 1000,
        
        // Input uncertainties
        inputs: vec![
            UncertainInput::Material("fr4_conductivity", Distribution::Normal { mean: 0.3, std: 0.015 }),
            UncertainInput::Material("copper_conductivity", Distribution::Normal { mean: 385.0, std: 7.7 }),
            UncertainInput::BoundaryCondition("h_convection", Distribution::Uniform { min: 8.0, max: 12.0 }),
            UncertainInput::HeatSource("U1_power", Distribution::Normal { mean: 2.5, std: 0.1 }),
        ],
        
        // Output statistics
        outputs: vec![
            UncertainOutput::MaxTemperature,
            UncertainOutput::TemperatureAt(Point::new(25.0, 20.0, 0.0)),
        ],
    },
    
    // Sensitivity analysis
    sensitivity: SensitivityAnalysis {
        method: SensitivityMethod::Sobol,
        first_order: true,
        total_order: true,
    },
}
```

### Validation Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Thermal Simulation Validation Report                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Model: my_board.hwt_pcb                                         │
│ Accuracy Level: Scientific                                      │
│                                                                 │
│ ═══════════════════════════════════════════════════════════════ │
│ MESH CONVERGENCE                                                │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Level │ Elements │ Max Temp │ Change  │ GCI                     │
│ ──────┼──────────┼──────────┼─────────┼────────                 │
│ 1     │ 1,024    │ 82.3°C   │ -       │ -                       │
│ 2     │ 4,096    │ 79.8°C   │ 3.0%    │ 4.2%                    │
│ 3     │ 16,384   │ 78.9°C   │ 1.1%    │ 1.5%                    │
│ 4     │ 65,536   │ 78.6°C   │ 0.4%    │ 0.5%  ✓                 │
│                                                                 │
│ Grid-converged solution: 78.6°C ± 0.4°C                        │
│                                                                 │
│ ═══════════════════════════════════════════════════════════════ │
│ EXPERIMENTAL VALIDATION                                         │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Location      │ Measured │ Simulated │ Error  │ Status          │
│ ──────────────┼──────────┼───────────┼────────┼────────         │
│ TC1 (U1)      │ 72.3°C   │ 73.1°C    │ 1.1%   │ ✓ Pass          │
│ TC2 (center)  │ 65.1°C   │ 64.2°C    │ 1.4%   │ ✓ Pass          │
│ TC3 (edge)    │ 58.7°C   │ 57.9°C    │ 1.4%   │ ✓ Pass          │
│                                                                 │
│ RMS Error: 1.3%  |  Max Error: 1.4%  |  Target: <5%  ✓         │
│                                                                 │
│ ═══════════════════════════════════════════════════════════════ │
│ UNCERTAINTY QUANTIFICATION                                      │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Max Temperature: 78.6°C ± 2.1°C (95% CI)                       │
│                                                                 │
│ Sensitivity (Sobol indices):                                    │
│   FR4 conductivity:     0.42 (dominant)                        │
│   Convection coeff:     0.31                                   │
│   Component power:      0.18                                   │
│   Copper conductivity:  0.09                                   │
│                                                                 │
│ ═══════════════════════════════════════════════════════════════ │
│ CERTIFICATION STATUS                                            │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ ✓ Mesh converged (GCI < 5%)                                    │
│ ✓ Experimental validation passed (error < 5%)                  │
│ ✓ Uncertainty quantified                                       │
│ ✓ Material properties traceable                                │
│                                                                 │
│ RESULT: VALIDATED FOR SCIENTIFIC USE                           │
│                                                                 │
│ [Export PDF] [Export Data] [Archive] [Close]                    │
└─────────────────────────────────────────────────────────────────┘
```

### Scientific Accuracy Configuration

```rust
ScientificAccuracyConfig {
    // Target accuracy
    target_error: 0.05,  // 5% vs experimental
    
    // Mesh
    mesh: MeshRefinement {
        adaptive: true,
        hp_refinement: true,
        gci_target: 0.05,
    },
    
    // Materials
    materials: MaterialDatabase {
        temperature_dependent: true,
        anisotropic: true,
        uncertainty_included: true,
    },
    
    // Physics
    physics: PhysicsConfig {
        conduction: true,
        convection: ConvectionModel::NusseltCorrelation,
        radiation: RadiationModel::Radiosity,
        contact_resistance: true,
    },
    
    // Validation
    validation: ValidationFramework {
        mesh_convergence: true,
        experimental_comparison: true,
        uncertainty_quantification: true,
    },
    
    // Solver
    solver: SolverConfig {
        method: SolverMethod::DirectLU,
        tolerance: 1e-10,
        coupled_iterations: 100,
        convergence_criterion: 1e-6,
    },
}
```

## Updated Comparison Matrix

### Feature Matrix (with Radiation & Scientific Accuracy)

| Capability | Hardware Tool | SPICE | Ansys |
|------------|---------------|-------|-------|
| **Thermal field (2D/3D)** | ✓ Full FEM | ✗ Lumped only | ✓ Full CFD |
| **Joule heating** | ✓ | ✓ | ✓ |
| **Convection (natural)** | ✓ Nusselt | ✗ | ✓ Full CFD |
| **Convection (forced)** | ✓ Correlation | ✗ | ✓ Full CFD |
| **Radiation** | ✓ Radiosity | ✗ | ✓ |
| **View factors** | ✓ Monte Carlo | ✗ | ✓ |
| **Temperature-dependent** | ✓ | ○ | ✓ |
| **Anisotropic materials** | ✓ | ✗ | ✓ |
| **Contact resistance** | ✓ | ✗ | ✓ |
| **Mesh adaptation** | ✓ hp-FEM | ✗ | ✓ |
| **Uncertainty quantification** | ✓ Monte Carlo | ✗ | ✓ |
| **Validation framework** | ✓ GCI + Exp | ✗ | ✓ |
| **Cost** | Free | Free | $$$$ |
| **Rust integration** | ✓ Native | ✗ | ✗ |
| **Accuracy** | **<5%** | 50%+ error | <5% |
| **Speed** | Seconds-Minutes | Seconds | Hours |

### When to Use Each

| Scenario | Recommended Tool |
|----------|------------------|
| **Design-phase what-if** | Hardware Tool |
| **Quick thermal check** | Hardware Tool |
| **Electrical transients** | SPICE → Hardware Tool |
| **Scientific publication** | Hardware Tool (with validation) |
| **Certification/validation** | Ansys (or Hardware Tool with full validation) |
| **CFD with airflow** | Ansys |
| **Cost-sensitive projects** | Hardware Tool |
| **Space/vacuum thermal** | Hardware Tool (radiation-dominant) |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `T` | Toggle thermal overlay |
| `Shift+T` | Run thermal simulation |
| `Ctrl+T` | Open thermal settings |
| `Alt+T` | Show thermal report |

## Related Topics

- [Calculator Tools](./calculator-tools.md) - Thermal via calculator, power dissipation
- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md) - 3D thermal visualization
- [DFM Checks](./dfm-checks.md) - Thermal design rules
- [Component Placement](../pcb-layout/component-placement.md) - Thermal placement strategies
