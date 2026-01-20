# RF Project Structure & Management

## Overview

Hardware Tool organizes RF, microwave, and photonic projects with a unified structure supporting amplifiers, filters, antennas, waveguides, and photonic integrated circuits. Projects integrate schematic capture with EM simulation and manufacturing output.

## Project Structure

```
my_rf_design/
├── my_rf_design.hwt_rf               # Project manifest
├── substrate/                         # Substrate definition
│   └── rogers_4003c.substrate        # Substrate properties
├── schematic/                         # RF schematics
│   ├── lna.hwt_sch                   # LNA schematic
│   ├── filter.hwt_sch                # Filter schematic
│   └── matching.hwt_sch              # Matching networks
├── layout/                            # Physical layout
│   ├── lna.hwt_rlay                  # LNA layout
│   ├── filter.hwt_rlay               # Filter layout
│   └── antenna.hwt_rlay              # Antenna layout
├── simulation/                        # Simulation data
│   ├── spice/                        # Circuit simulation
│   ├── em/                           # EM simulation
│   │   ├── 2d/                       # 2D EM (microstrip)
│   │   └── 3d/                       # 3D EM (full-wave)
│   └── system/                       # System-level
├── models/                            # Component models
│   ├── transistors/                  # Transistor models
│   ├── passives/                     # Passive models
│   └── s_parameters/                 # S-parameter files
├── measurements/                      # Measurement data
│   ├── vna/                          # VNA measurements
│   └── spectrum/                     # Spectrum analyzer
└── output/                            # Manufacturing output
    ├── gerber/                       # PCB Gerbers
    ├── gdsii/                        # IC masks
    └── touchstone/                   # S-parameter files
```

## Project Manifest (.hwt_rf)

```toml
[project]
name = "wifi_frontend"
version = "1.0.0"
type = "rf_module"
description = "2.4 GHz WiFi front-end module"

[frequency]
center = 2.45e9           # Hz
bandwidth = 100e6         # Hz
harmonics = [2, 3, 5]     # For harmonic analysis

[substrate]
material = "Rogers RO4003C"
thickness = 0.508e-3      # m (20 mil)
er = 3.55
loss_tangent = 0.0027
copper_thickness = 35e-6  # m (1 oz)

[design]
impedance = 50.0          # Ω
topology = "microstrip"

[lna]
gain = 15.0               # dB
noise_figure = 1.5        # dB
iip3 = -5.0               # dBm

[filter]
type = "bandpass"
order = 5
ripple = 0.1              # dB

[simulation]
em_solver = "fdtd"
mesh_density = 20         # cells per wavelength
```

## Rust API

```rust
// Create RF project
let project = RFProject::new("wifi_frontend")?;

// Set substrate
project.set_substrate(Substrate::Rogers_RO4003C {
    thickness: 0.508e-3,
    copper: 35e-6,
})?;

// Create LNA
let lna = project.create_lna("lna")?;
lna.set_frequency(2.4e9, 2.5e9)?;
lna.set_gain(15.0)?;
lna.set_noise_figure(1.5)?;

// Design matching networks
lna.design_input_match(MatchType::LNetwork)?;
lna.design_output_match(MatchType::PiNetwork)?;

// Create filter
let filter = project.create_filter("bpf")?;
filter.set_type(FilterType::Bandpass)?;
filter.set_center_frequency(2.45e9)?;
filter.set_bandwidth(100e6)?;
filter.synthesize(SynthesisMethod::Chebyshev { ripple: 0.1 })?;

// Run EM simulation
project.simulate_em(EMConfig::default())?;

// Export
project.export_gerber("output/gerber/")?;
project.export_touchstone("output/touchstone/")?;
```

## Related Topics

- [Unified RF Design File Format](./unified-rf-design-file-format.md)
- [RF Schematic Capture Workflow](./rf-schematic-capture-workflow.md)
- [Waveguide Layout Workflow](./waveguide-layout-workflow.md)
