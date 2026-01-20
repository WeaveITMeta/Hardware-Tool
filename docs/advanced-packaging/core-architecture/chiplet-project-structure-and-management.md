# Chiplet Project Structure & Management

## Overview

Hardware Tool organizes advanced packaging projects with a unified structure supporting 2.5D/3D integration, chiplets, interposers, and heterogeneous systems. Projects integrate die-level design with package-level assembly and thermal/mechanical analysis.

## Project Structure

```
my_chiplet_system/
├── my_chiplet_system.hwt_pkg         # Project manifest
├── dies/                              # Die definitions
│   ├── compute_die/                  # Compute chiplet
│   │   ├── compute.hwt_die           # Die specification
│   │   └── bumps.hwt_bump            # Bump map
│   ├── hbm_stack/                    # HBM memory
│   │   └── hbm3.hwt_die              # HBM specification
│   └── io_die/                       # I/O chiplet
│       └── io.hwt_die                # I/O specification
├── interposer/                        # Interposer design
│   ├── interposer.hwt_int            # Interposer layout
│   ├── rdl/                          # RDL layers
│   │   ├── rdl_m1.hwt_rdl            # RDL metal 1
│   │   └── rdl_m2.hwt_rdl            # RDL metal 2
│   └── tsv/                          # TSV definitions
│       └── tsv_array.hwt_tsv         # TSV arrays
├── substrate/                         # Package substrate
│   ├── substrate.hwt_sub             # Substrate layout
│   └── bga.hwt_bga                   # BGA pattern
├── assembly/                          # Assembly definition
│   ├── die_placement.hwt_asm         # Die positions
│   ├── bonding.hwt_bond              # Bonding spec
│   └── wire_bond.hwt_wb              # Wire bonds (if any)
├── simulation/                        # Simulation data
│   ├── thermal/                      # Thermal analysis
│   ├── mechanical/                   # Stress/warpage
│   ├── si/                           # Signal integrity
│   └── pi/                           # Power integrity
├── verification/                      # Verification
│   ├── drc/                          # Design rule check
│   └── connectivity/                 # Connectivity check
└── output/                            # Manufacturing output
    ├── gdsii/                        # Interposer masks
    ├── substrate/                    # Substrate Gerbers
    ├── assembly/                     # Assembly data
    └── documentation/                # Fab drawings
```

## Project Manifest (.hwt_pkg)

```toml
[project]
name = "hpc_accelerator"
version = "1.0.0"
type = "2.5d_interposer"
description = "HPC accelerator with HBM3 memory"

[package]
size = [55e-3, 55e-3]     # m
type = "FCBGA"
ball_pitch = 1.0e-3       # m
ball_count = 2500

[interposer]
material = "silicon"
thickness = 100e-6        # m
rdl_layers = 4
tsv_pitch = 50e-6         # m

[dies]
[[dies.die]]
name = "compute"
technology = "5nm"
size = [10e-3, 10e-3]     # m
power = 150.0             # W
position = [0, 0]         # center

[[dies.die]]
name = "hbm3_1"
technology = "hbm3"
size = [8e-3, 10e-3]
power = 20.0
position = [-12e-3, 0]    # left of compute

[[dies.die]]
name = "hbm3_2"
technology = "hbm3"
size = [8e-3, 10e-3]
power = 20.0
position = [12e-3, 0]     # right of compute

[interconnect]
standard = "UCIe"
data_rate = 32e9          # bps
lanes = 64

[thermal]
max_junction = 105.0      # °C
heat_sink = "forced_air"
thermal_resistance = 0.1  # °C/W
```

## Rust API

```rust
// Create chiplet system
let system = ChipletSystem::new("hpc_accelerator")?;

// Add dies
let compute = system.add_die(Die::new("compute", Technology::N5)?)?;
let hbm1 = system.add_die(Die::new("hbm3_1", Technology::HBM3)?)?;
let hbm2 = system.add_die(Die::new("hbm3_2", Technology::HBM3)?)?;

// Create interposer
let interposer = system.create_interposer(InterposerConfig {
    size: (55e-3, 55e-3),
    thickness: 100e-6,
    rdl_layers: 4,
    tsv_pitch: 50e-6,
})?;

// Place dies
interposer.place_die(&compute, Position::center())?;
interposer.place_die(&hbm1, Position::left_of(&compute, 2e-3))?;
interposer.place_die(&hbm2, Position::right_of(&compute, 2e-3))?;

// Connect with UCIe
system.connect(&compute, &hbm1, UCIe::new(64))?;
system.connect(&compute, &hbm2, UCIe::new(64))?;

// Run analyses
let thermal = system.analyze_thermal()?;
let si = system.analyze_si()?;

// Export
system.export_gdsii("output/gdsii/interposer.gds")?;
system.export_substrate("output/substrate/package.brd")?;
```

## Related Topics

- [Unified Packaging File Format](./unified-packaging-file-format.md)
- [Interposer Capture Workflow](./interposer-capture-workflow.md)
- [Heterogeneous Integration Workflow](./heterogeneous-integration-workflow.md)
