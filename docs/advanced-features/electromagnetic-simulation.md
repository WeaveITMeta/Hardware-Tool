# Electromagnetic Simulation (EMC/EMI)

## Overview

Hardware Tool includes integrated electromagnetic simulation for predicting and mitigating EMC/EMI issues before physical prototyping. The 3D full-wave field solver enables accurate prediction of radiated emissions, susceptibility, and near-field coupling—critical for regulatory compliance (FCC, CE, CISPR).

### Why EMC/EMI Simulation Matters

| Issue | Consequence | Cost to Fix |
|-------|-------------|-------------|
| **Radiated emissions fail** | Product recall, redesign | $50k-$500k |
| **Conducted emissions fail** | Filter redesign, re-test | $10k-$50k |
| **ESD susceptibility** | Field failures, RMA | $100k+ |
| **Crosstalk/coupling** | Intermittent failures | $5k-$20k |

### Simulation Capabilities

```
┌─────────────────────────────────────────────────────────────────┐
│                    EMC/EMI Simulation Suite                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Near-Field │  │  Far-Field  │  │  Conducted  │             │
│  │  Analysis   │  │  Radiation  │  │  Emissions  │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              3D Full-Wave Field Solver                   │   │
│  │         (FDTD / MoM / FEM Hybrid Engine)                │   │
│  └─────────────────────────────────────────────────────────┘   │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Coupling   │  │  Antenna    │  │  Filter     │             │
│  │  Prediction │  │  Patterns   │  │  Design     │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Near-Field Analysis

### Magnetic Field (H-Field) Visualization

Predict magnetic field emissions from current loops:

```rust
NearFieldAnalysis {
    // Analysis type
    field_type: FieldType::Magnetic,  // H-field
    
    // Frequency range
    frequencies: vec![30e6, 100e6, 300e6, 1e9],  // MHz to GHz
    
    // Observation plane
    observation: ObservationPlane {
        height: 5.0,                  // mm above board
        resolution: 0.5,              // mm grid
        extent: PlaneExtent::BoardOutline { margin: 10.0 },
    },
    
    // Sources
    sources: SourceConfig {
        method: SourceMethod::FromCurrents,  // Extract from SI/PI
        include_harmonics: true,
        harmonic_count: 10,
    },
    
    // Output
    output: NearFieldOutput {
        magnitude_plot: true,
        vector_plot: true,
        animation: true,              // Time-domain animation
        export_nf2ff: true,           // For far-field transform
    },
}
```

### Near-Field Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ Near-Field H-Field @ 100 MHz                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Magnitude (dBµA/m)                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░▓▓████████▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░▓██████████████▓░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░▓██████████████▓░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░▓▓████████▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░░░▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │                    ▲                                        │ │
│ │               U1 (MCU)                                      │ │
│ │            100 MHz clock                                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Legend: ░ <40  ▒ 40-60  ▓ 60-80  █ >80 dBµA/m                 │
│                                                                 │
│ Hotspots:                                                       │
│   1. U1 clock loop: 85 dBµA/m (reduce loop area)              │
│   2. Power input: 72 dBµA/m (add common-mode choke)           │
│                                                                 │
│ [3D View] [Vector Plot] [Animate] [Export] [Close]              │
└─────────────────────────────────────────────────────────────────┘
```

### Electric Field (E-Field) Analysis

```rust
EFieldAnalysis {
    field_type: FieldType::Electric,
    
    // High-impedance sources (voltage-driven)
    sources: vec![
        EFieldSource::Net { name: "CLK_100M", amplitude: 3.3 },
        EFieldSource::Net { name: "RESET_N", amplitude: 3.3 },
    ],
    
    // Coupling analysis
    coupling: CouplingAnalysis {
        victim_nets: vec!["ANALOG_IN", "ADC_REF"],
        coupling_threshold: -40.0,    // dB
    },
}
```

## Far-Field Radiation

### Radiated Emissions Prediction

Predict emissions at standard test distances:

```rust
RadiatedEmissions {
    // Test configuration (per CISPR 32)
    test_config: TestConfig {
        standard: Standard::CISPR32_ClassB,
        distance: 10.0,               // meters
        frequency_range: (30e6, 1e9), // 30 MHz to 1 GHz
        polarization: Polarization::Both,
    },
    
    // Antenna patterns
    antenna: AntennaConfig {
        type_: AntennaType::Biconical,  // 30-300 MHz
        height_scan: vec![1.0, 2.0, 3.0, 4.0],  // meters
    },
    
    // Limits
    limits: EmissionLimits::CISPR32_ClassB,
    
    // Margin
    margin: 6.0,                      // dB design margin
}
```

### Emissions Spectrum

```
┌─────────────────────────────────────────────────────────────────┐
│ Radiated Emissions @ 10m (CISPR 32 Class B)                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Level (dBµV/m)                                                  │
│    50├─────────────────────────────────────────────────────────│
│      │                              Limit ─────────────────────│
│    40├─────────────────────────────────────────────────────────│
│      │     ╱╲                                                   │
│    30├────╱──╲──────────────────────────────────────────────── │
│      │   ╱    ╲    ╱╲                                          │
│    20├──╱──────╲──╱──╲──────────────────────────────────────── │
│      │ ╱        ╲╱    ╲    ╱╲                                  │
│    10├╱──────────────────╲╱──╲──────────────────────────────── │
│      │                        ╲                                │
│     0├─────────────────────────╲───────────────────────────── │
│      └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬───────│
│        30   50   100  200  300  500  700  1000               │
│                     Frequency (MHz)                            │
│                                                                 │
│ ─── Measured    ─ ─ Limit (Class B)    ▬▬▬ Limit - 6dB margin │
│                                                                 │
│ Status: ✓ PASS (6.2 dB margin at worst case)                  │
│                                                                 │
│ Peaks:                                                          │
│   100 MHz: 28.5 dBµV/m (limit 40) ✓ 11.5 dB margin            │
│   300 MHz: 25.2 dBµV/m (limit 37) ✓ 11.8 dB margin            │
│                                                                 │
│ [Antenna Pattern] [3D Radiation] [Export] [Close]               │
└─────────────────────────────────────────────────────────────────┘
```

### 3D Radiation Pattern

```rust
RadiationPattern3D {
    // Frequency of interest
    frequency: 100e6,
    
    // Angular resolution
    theta_resolution: 5.0,            // degrees
    phi_resolution: 5.0,              // degrees
    
    // Visualization
    visualization: PatternVisualization {
        type_: PatternType::Gain,     // or EField, Power
        scale: Scale::Logarithmic,
        normalize: true,
        show_main_lobe: true,
        show_nulls: true,
    },
    
    // Export
    export: PatternExport {
        format: ExportFormat::CSV,
        include_phase: true,
    },
}
```

## Conducted Emissions

### Common-Mode / Differential-Mode Analysis

```rust
ConductedEmissions {
    // Test port (power input)
    port: "J1",  // Power connector
    
    // LISN configuration
    lisn: LISNConfig {
        impedance: 50.0,              // Ω
        type_: LISNType::V_Network,
    },
    
    // Frequency range (CISPR 32)
    frequency_range: (150e3, 30e6),   // 150 kHz to 30 MHz
    
    // Decomposition
    decomposition: CMDMDecomposition {
        enabled: true,
        show_cm: true,                // Common-mode
        show_dm: true,                // Differential-mode
    },
    
    // Limits
    limits: ConductedLimits::CISPR32_ClassB,
}
```

### Conducted Emissions Spectrum

```
┌─────────────────────────────────────────────────────────────────┐
│ Conducted Emissions (CISPR 32 Class B)                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Level (dBµV)                                                    │
│    80├─────────────────────────────────────────────────────────│
│      │                                                          │
│    70├─────────────────────────────────────────────────────────│
│      │ Limit (QP) ─────────────────────────────────────────────│
│    60├─────────────────────────────────────────────────────────│
│      │╱╲                                                        │
│    50├╱─╲───────────────────────────────────────────────────── │
│      │   ╲  ╱╲                                                  │
│    40├────╲╱──╲─────────────────────────────────────────────── │
│      │        ╲    ╱╲                                          │
│    30├─────────╲──╱──╲──────────────────────────────────────── │
│      │          ╲╱    ╲                                        │
│    20├───────────────────╲─────────────────────────────────── │
│      └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬───────│
│       0.15  0.5   1    2    5    10   20   30                 │
│                     Frequency (MHz)                            │
│                                                                 │
│ ─── Total    ─ ─ CM    ··· DM    ▬▬▬ Limit                    │
│                                                                 │
│ ⚠ FAIL at 500 kHz: 62 dBµV (limit 56 dBµV)                    │
│   Dominant: Common-mode (switching noise)                       │
│   Fix: Add CM choke (≥1mH @ 500kHz)                            │
│                                                                 │
│ [Filter Design] [CM/DM Analysis] [Export] [Close]               │
└─────────────────────────────────────────────────────────────────┘
```

## EMI Filter Design

### Automatic Filter Synthesis

```rust
EMIFilterDesign {
    // Target
    target: FilterTarget {
        emissions_spec: ConductedLimits::CISPR32_ClassB,
        margin: 6.0,                  // dB
    },
    
    // Constraints
    constraints: FilterConstraints {
        max_components: 10,
        max_cost: 5.0,                // $
        max_volume: 500.0,            // mm³
        max_insertion_loss_dc: 0.1,   // Ω
    },
    
    // Topology options
    topology: FilterTopology::Auto,   // or Pi, T, LC, etc.
    
    // Component library
    library: FilterComponentLibrary {
        inductors: vec!["Murata", "TDK", "Wurth"],
        capacitors: vec!["Murata", "Samsung"],
    },
}
```

### Filter Design Output

```
┌─────────────────────────────────────────────────────────────────┐
│ EMI Filter Design                                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Recommended Filter (Pi topology):                               │
│                                                                 │
│      ┌─────┐      ┌─────┐      ┌─────┐                         │
│  ────┤ C1  ├──────┤ L1  ├──────┤ C2  ├────                     │
│      │100nF│      │10µH │      │100nF│                         │
│      └──┬──┘      └─────┘      └──┬──┘                         │
│         │                         │                             │
│        ─┴─                       ─┴─                            │
│        GND                       GND                            │
│                                                                 │
│ Components:                                                     │
│   C1: Murata GRM155R71H104K (100nF, 0402) - $0.02              │
│   L1: TDK MLZ1005M100W (10µH, 0402) - $0.15                    │
│   C2: Murata GRM155R71H104K (100nF, 0402) - $0.02              │
│                                                                 │
│ Total cost: $0.19                                               │
│ Total volume: 2.4 mm³                                           │
│                                                                 │
│ Performance:                                                    │
│   Attenuation @ 500kHz: 18 dB (need 6 dB) ✓                   │
│   Attenuation @ 1MHz: 24 dB ✓                                  │
│   DC resistance: 0.05Ω ✓                                       │
│                                                                 │
│ [Add to Schematic] [Simulate] [Optimize] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## Shielding Effectiveness

### Enclosure Shielding Analysis

```rust
ShieldingAnalysis {
    // Enclosure model
    enclosure: EnclosureModel {
        material: ShieldMaterial::Aluminum,
        thickness: 1.5,               // mm
        conductivity: 3.5e7,          // S/m
        
        // Apertures
        apertures: vec![
            Aperture { type_: ApertureType::Slot, length: 50.0, width: 2.0 },
            Aperture { type_: ApertureType::Hole, diameter: 5.0, count: 10 },
            Aperture { type_: ApertureType::Seam, length: 100.0, gap: 0.1 },
        ],
    },
    
    // Frequency range
    frequency_range: (30e6, 1e9),
    
    // Analysis
    analysis: ShieldingAnalysisType {
        se_vs_frequency: true,
        aperture_contribution: true,
        worst_case_direction: true,
    },
}
```

### Shielding Effectiveness Plot

```
┌─────────────────────────────────────────────────────────────────┐
│ Shielding Effectiveness Analysis                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ SE (dB)                                                         │
│   100├─────────────────────────────────────────────────────────│
│      │─────────────────────────────────────────────────────────│
│    80├─────────────────────────────────────────────────────────│
│      │                                                          │
│    60├─────────────────────────────────────────────────────────│
│      │                    ╲                                     │
│    40├─────────────────────╲────────────────────────────────── │
│      │                      ╲        ╱╲                        │
│    20├───────────────────────╲──────╱──╲─────────────────────  │
│      │                        ╲    ╱    ╲                      │
│     0├─────────────────────────╲──╱──────╲───────────────────  │
│      └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬───────│
│        30   50   100  200  300  500  700  1000               │
│                     Frequency (MHz)                            │
│                                                                 │
│ ─── Total SE    ─ ─ Material only    ··· With apertures       │
│                                                                 │
│ Limiting factors:                                               │
│   @ 100 MHz: 50mm slot (SE = 25 dB)                            │
│   @ 500 MHz: Slot resonance (SE = 8 dB) ⚠                     │
│   @ 800 MHz: Hole array (SE = 15 dB)                           │
│                                                                 │
│ Recommendations:                                                │
│   1. Add EMI gasket to seam → +15 dB                          │
│   2. Reduce slot to 25mm → +6 dB @ resonance                  │
│   3. Use honeycomb vent → +20 dB vs holes                     │
│                                                                 │
│ [Apply Fixes] [3D View] [Export] [Close]                        │
└─────────────────────────────────────────────────────────────────┘
```

## ESD Analysis

### ESD Susceptibility Simulation

```rust
ESDAnalysis {
    // ESD event
    event: ESDEvent {
        model: ESDModel::HBM,         // Human Body Model
        voltage: 8000.0,              // V (IEC 61000-4-2 Level 4)
        waveform: ESDWaveform::IEC61000_4_2,
    },
    
    // Injection points
    injection_points: vec![
        ESDPoint::Connector("USB"),
        ESDPoint::Connector("HDMI"),
        ESDPoint::TouchPoint("Button1"),
        ESDPoint::Enclosure("Screw1"),
    ],
    
    // Victim circuits
    victims: vec![
        VictimCircuit { net: "USB_D+", threshold: 7.0 },   // V
        VictimCircuit { net: "USB_D-", threshold: 7.0 },
        VictimCircuit { net: "RESET_N", threshold: 5.5 },
    ],
    
    // Analysis
    analysis: ESDAnalysisType {
        voltage_waveforms: true,
        current_paths: true,
        protection_effectiveness: true,
    },
}
```

### ESD Current Path Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ ESD Analysis: USB Connector (8kV HBM)                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Current Path:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   ESD ──▶ USB Shell ──▶ Shield ──▶ GND Plane ──▶ Earth     │ │
│ │              │                                              │ │
│ │              ▼ (coupling)                                   │ │
│ │           USB_D+ ──▶ TVS ──▶ GND (protected path)          │ │
│ │              │                                              │ │
│ │              ▼ (residual)                                   │ │
│ │           PHY IC (U3)                                       │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Voltage at U3 pins:                                             │
│   USB_D+: 4.2V peak (limit 7V) ✓                              │
│   USB_D-: 3.8V peak (limit 7V) ✓                              │
│   VBUS:   5.1V peak (limit 6V) ✓                              │
│                                                                 │
│ Protection devices:                                             │
│   TVS1 (PESD5V0S2BT): Clamping @ 8.5V, 15A peak ✓             │
│                                                                 │
│ Status: ✓ PROTECTED (all voltages within limits)              │
│                                                                 │
│ [Current Animation] [Waveforms] [Export] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## 3D Full-Wave Solver

### FDTD Engine Configuration

```rust
FDTDSolver {
    // Mesh
    mesh: FDTDMesh {
        min_cell_size: 0.1,           // mm
        max_cell_size: 2.0,           // mm
        adaptive: true,
        cells_per_wavelength: 20,
    },
    
    // Boundary conditions
    boundaries: BoundaryConditions {
        type_: BoundaryType::PML,     // Perfectly Matched Layer
        pml_layers: 8,
    },
    
    // Excitation
    excitation: Excitation {
        type_: ExcitationType::GaussianPulse,
        bandwidth: (1e6, 10e9),       // 1 MHz to 10 GHz
    },
    
    // Solver settings
    solver: SolverSettings {
        time_steps: 50000,
        convergence: -40.0,           // dB
        gpu_acceleration: true,
    },
}
```

### GPU-Accelerated Simulation

```rust
GPUAcceleration {
    enabled: true,
    
    // Device selection
    device: GPUDevice::Auto,          // or Specific(0)
    
    // Memory management
    memory: GPUMemory {
        max_usage: 0.8,               // 80% of VRAM
        out_of_core: true,            // Spill to system RAM
    },
    
    // Performance
    performance: GPUPerformance {
        precision: Precision::Single, // or Double
        async_transfers: true,
    },
}
```

## Pre-Compliance Testing

### Automated Pre-Compliance Report

```rust
PreComplianceReport {
    // Standards
    standards: vec![
        Standard::FCC_Part15_ClassB,
        Standard::CISPR32_ClassB,
        Standard::IEC61000_4_2,       // ESD
        Standard::IEC61000_4_3,       // Radiated immunity
    ],
    
    // Tests
    tests: vec![
        Test::RadiatedEmissions,
        Test::ConductedEmissions,
        Test::ESDImmunity,
        Test::RadiatedImmunity,
    ],
    
    // Report format
    format: ReportFormat::PDF,
    
    // Include
    include: ReportContent {
        executive_summary: true,
        detailed_results: true,
        plots: true,
        recommendations: true,
        compliance_matrix: true,
    },
}
```

### Pre-Compliance Report

```
┌─────────────────────────────────────────────────────────────────┐
│ EMC Pre-Compliance Report                                       │
│ Project: my_board.hwt                                           │
│ Date: 2026-01-19                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ EXECUTIVE SUMMARY                                               │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Overall Status: ⚠ CONDITIONAL PASS                            │
│                                                                 │
│ Test                      │ Standard        │ Result │ Margin  │
│ ──────────────────────────┼─────────────────┼────────┼──────── │
│ Radiated Emissions        │ FCC Part 15B    │ ✓ Pass │ +6.2 dB │
│ Radiated Emissions        │ CISPR 32 B      │ ✓ Pass │ +5.8 dB │
│ Conducted Emissions       │ FCC Part 15B    │ ⚠ Marg │ +1.2 dB │
│ Conducted Emissions       │ CISPR 32 B      │ ⚠ Marg │ +0.8 dB │
│ ESD Immunity (contact)    │ IEC 61000-4-2   │ ✓ Pass │ Level 4 │
│ ESD Immunity (air)        │ IEC 61000-4-2   │ ✓ Pass │ Level 4 │
│                                                                 │
│ RECOMMENDATIONS                                                 │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ 1. Conducted Emissions (Priority: HIGH)                         │
│    Issue: Marginal pass at 500 kHz (1.2 dB margin)             │
│    Root cause: Switching noise from U2 (DC-DC)                 │
│    Fix: Add 10µH CM choke + 100nF Y-caps                       │
│    Estimated improvement: +8 dB                                 │
│    Cost: $0.35                                                  │
│                                                                 │
│ 2. Clock Harmonics (Priority: MEDIUM)                           │
│    Issue: 100 MHz harmonics visible at 300, 500 MHz            │
│    Root cause: CLK trace near board edge                       │
│    Fix: Route CLK on inner layer, add ground guard             │
│    Estimated improvement: +4 dB                                 │
│                                                                 │
│ CONFIDENCE LEVEL: 85%                                           │
│ (Based on simulation accuracy vs. typical lab correlation)      │
│                                                                 │
│ [Export PDF] [Apply Fixes] [Re-simulate] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## Rust Implementation

### Core EMC Engine

```rust
use nalgebra::{Vector3, Complex};
use ndarray::Array3;

// ═══════════════════════════════════════════════════════════════
// FDTD Field Solver
// ═══════════════════════════════════════════════════════════════

struct FDTDSolver {
    // Grid
    ex: Array3<f64>,
    ey: Array3<f64>,
    ez: Array3<f64>,
    hx: Array3<f64>,
    hy: Array3<f64>,
    hz: Array3<f64>,
    
    // Material properties
    epsilon: Array3<f64>,
    sigma: Array3<f64>,
    mu: Array3<f64>,
    
    // Grid spacing
    dx: f64,
    dy: f64,
    dz: f64,
    dt: f64,
}

impl FDTDSolver {
    fn new(nx: usize, ny: usize, nz: usize, cell_size: f64) -> Self {
        let dx = cell_size;
        let dy = cell_size;
        let dz = cell_size;
        
        // Courant stability condition
        let c = 3e8;  // Speed of light
        let dt = 0.99 / (c * (1.0/dx.powi(2) + 1.0/dy.powi(2) + 1.0/dz.powi(2)).sqrt());
        
        Self {
            ex: Array3::zeros((nx, ny, nz)),
            ey: Array3::zeros((nx, ny, nz)),
            ez: Array3::zeros((nx, ny, nz)),
            hx: Array3::zeros((nx, ny, nz)),
            hy: Array3::zeros((nx, ny, nz)),
            hz: Array3::zeros((nx, ny, nz)),
            epsilon: Array3::from_elem((nx, ny, nz), 8.854e-12),
            sigma: Array3::zeros((nx, ny, nz)),
            mu: Array3::from_elem((nx, ny, nz), 1.257e-6),
            dx, dy, dz, dt,
        }
    }
    
    fn step(&mut self) {
        // Update H-field (Faraday's law)
        self.update_h_field();
        
        // Update E-field (Ampere's law)
        self.update_e_field();
    }
    
    fn update_h_field(&mut self) {
        let dt_mu = self.dt / self.mu[[0, 0, 0]];
        
        for i in 0..self.hx.dim().0 - 1 {
            for j in 0..self.hx.dim().1 - 1 {
                for k in 0..self.hx.dim().2 - 1 {
                    // Hx update
                    self.hx[[i, j, k]] -= dt_mu * (
                        (self.ez[[i, j+1, k]] - self.ez[[i, j, k]]) / self.dy -
                        (self.ey[[i, j, k+1]] - self.ey[[i, j, k]]) / self.dz
                    );
                    
                    // Hy update
                    self.hy[[i, j, k]] -= dt_mu * (
                        (self.ex[[i, j, k+1]] - self.ex[[i, j, k]]) / self.dz -
                        (self.ez[[i+1, j, k]] - self.ez[[i, j, k]]) / self.dx
                    );
                    
                    // Hz update
                    self.hz[[i, j, k]] -= dt_mu * (
                        (self.ey[[i+1, j, k]] - self.ey[[i, j, k]]) / self.dx -
                        (self.ex[[i, j+1, k]] - self.ex[[i, j, k]]) / self.dy
                    );
                }
            }
        }
    }
    
    fn update_e_field(&mut self) {
        for i in 1..self.ex.dim().0 {
            for j in 1..self.ex.dim().1 {
                for k in 1..self.ex.dim().2 {
                    let eps = self.epsilon[[i, j, k]];
                    let sig = self.sigma[[i, j, k]];
                    
                    let ca = (1.0 - sig * self.dt / (2.0 * eps)) / 
                             (1.0 + sig * self.dt / (2.0 * eps));
                    let cb = (self.dt / eps) / 
                             (1.0 + sig * self.dt / (2.0 * eps));
                    
                    // Ex update
                    self.ex[[i, j, k]] = ca * self.ex[[i, j, k]] + cb * (
                        (self.hz[[i, j, k]] - self.hz[[i, j-1, k]]) / self.dy -
                        (self.hy[[i, j, k]] - self.hy[[i, j, k-1]]) / self.dz
                    );
                    
                    // Similar for Ey, Ez...
                }
            }
        }
    }
    
    fn near_to_far_field(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
        // Near-field to far-field transformation
        // Using equivalence principle
        let k = 2.0 * std::f64::consts::PI * frequency / 3e8;
        
        // Integrate over observation surface
        // ... implementation
        Complex::new(0.0, 0.0)
    }
}

// ═══════════════════════════════════════════════════════════════
// Emissions Calculator
// ═══════════════════════════════════════════════════════════════

struct EmissionsCalculator {
    solver: FDTDSolver,
    limits: EmissionLimits,
}

impl EmissionsCalculator {
    fn calculate_radiated_emissions(
        &mut self,
        frequencies: &[f64],
        distance: f64,
    ) -> Vec<EmissionResult> {
        let mut results = Vec::new();
        
        for &freq in frequencies {
            // Get far-field at test distance
            let e_field = self.solver.near_to_far_field(freq, 90.0, 0.0);
            let magnitude_dbuvm = 20.0 * (e_field.norm() * 1e6).log10();
            
            // Compare to limit
            let limit = self.limits.get_limit(freq);
            let margin = limit - magnitude_dbuvm;
            
            results.push(EmissionResult {
                frequency: freq,
                level: magnitude_dbuvm,
                limit,
                margin,
                pass: margin > 0.0,
            });
        }
        
        results
    }
}
```

## API Usage

```rust
// Run EMC analysis
let emc_result = pcb.analyze_emc(EMCConfig {
    analysis: vec![
        EMCAnalysis::NearField { height: 5.0 },
        EMCAnalysis::RadiatedEmissions { distance: 10.0 },
        EMCAnalysis::ConductedEmissions,
    ],
    standards: vec![Standard::CISPR32_ClassB],
    margin: 6.0,
})?;

// Check compliance
if emc_result.radiated.pass && emc_result.conducted.pass {
    println!("✓ Pre-compliance PASS");
} else {
    for issue in emc_result.issues {
        println!("⚠ {}: {} dB over limit at {} MHz", 
            issue.test, issue.excess, issue.frequency / 1e6);
    }
}

// Auto-fix EMI issues
let fixes = pcb.suggest_emc_fixes(&emc_result)?;
for fix in fixes {
    println!("Suggested: {} (est. improvement: {} dB)", fix.description, fix.improvement);
}

// Generate pre-compliance report
pcb.generate_emc_report(ReportConfig {
    format: ReportFormat::PDF,
    output: "emc_report.pdf",
})?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `M` | Toggle near-field overlay |
| `Shift+M` | Run EMC analysis |
| `Ctrl+M` | EMC dashboard |
| `Alt+M` | Generate EMC report |

## Related Topics

- [Signal Integrity](./signal-power-integrity.md) - SI/PI analysis
- [DRC](../pcb-layout/drc.md) - Design rule checks
- [Thermal Simulation](./thermal-simulation.md) - Current-induced heating
- [DFM Checks](./dfm-checks.md) - Manufacturing checks
- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md) - 3D visualization
