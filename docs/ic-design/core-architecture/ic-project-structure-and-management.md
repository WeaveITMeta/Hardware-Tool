# IC Project Structure & Management

## Overview

Hardware Tool organizes IC design projects with a unified structure supporting digital ASIC, analog, and mixed-signal workflows. Projects integrate seamlessly with Process Design Kits (PDKs) and maintain compatibility with industry-standard tools.

## Project Structure

```
my_ic_project/
├── my_ic_project.hwt_ic           # Project manifest
├── pdk/                            # PDK reference
│   └── sky130.pdk_link            # Symlink to installed PDK
├── rtl/                            # RTL source files
│   ├── top.v                      # Top-level Verilog
│   ├── cpu_core.v                 # CPU module
│   └── memory.v                   # Memory module
├── analog/                         # Analog schematics
│   ├── ldo.hwt_sch                # LDO schematic
│   └── pll.hwt_sch                # PLL schematic
├── layout/                         # Physical layout
│   ├── top.hwt_lay                # Top-level layout
│   ├── cpu_core.hwt_lay           # CPU layout
│   └── analog/
│       ├── ldo.hwt_lay            # LDO layout
│       └── pll.hwt_lay            # PLL layout
├── lib/                            # Custom libraries
│   ├── cells/                     # Standard cells
│   └── ip/                        # IP blocks
├── constraints/                    # Design constraints
│   ├── timing.sdc                 # Timing constraints
│   ├── floorplan.tcl              # Floorplan
│   └── power.upf                  # Power intent
├── sim/                            # Simulation
│   ├── testbench/                 # Testbenches
│   ├── spice/                     # SPICE netlists
│   └── results/                   # Simulation results
├── signoff/                        # Signoff data
│   ├── drc/                       # DRC reports
│   ├── lvs/                       # LVS reports
│   ├── timing/                    # STA reports
│   └── power/                     # Power reports
└── output/                         # Manufacturing output
    ├── gdsii/                     # GDSII files
    ├── lef/                       # LEF files
    └── spef/                      # Parasitic data
```

## Project Manifest (.hwt_ic)

```toml
[project]
name = "my_soc"
version = "1.0.0"
type = "mixed-signal"
description = "System-on-Chip with analog peripherals"

[pdk]
name = "sky130"
version = "1.0.0"
variant = "sky130A"
path = "./pdk/sky130.pdk_link"

[technology]
node = "130nm"
metal_layers = 5
poly_layers = 1

[design]
top_module = "top"
clock_period = 10.0  # ns

[digital]
rtl_path = "./rtl"
synthesis_library = "sky130_fd_sc_hd"
optimization = "area"

[analog]
schematic_path = "./analog"
simulation_corner = "tt"

[constraints]
timing = "./constraints/timing.sdc"
floorplan = "./constraints/floorplan.tcl"
power = "./constraints/power.upf"

[output]
gdsii = "./output/gdsii"
lef = "./output/lef"
```

## PDK Integration

```rust
PDKIntegration {
    // PDK loading
    loading: PDKLoading {
        auto_detect: true,
        search_paths: vec![
            "$HWT_PDK_PATH",
            "~/.hwt/pdks",
            "/opt/pdks",
        ],
    },
    
    // PDK components
    components: PDKComponents {
        device_models: true,      // SPICE models
        standard_cells: true,     // Digital cells
        io_cells: true,           // I/O pads
        memory_compilers: true,   // SRAM, ROM
        analog_devices: true,     // Transistors, R, C
        pcells: true,             // Parameterized cells
    },
    
    // Technology files
    technology: TechnologyFiles {
        layer_map: true,
        design_rules: true,
        antenna_rules: true,
        lef_tech: true,
    },
}
```

## Rust API

```rust
// Create new IC project
let project = ICProject::new("my_soc")?;

// Set PDK
project.set_pdk(PDK::SkyWater130)?;

// Import RTL
project.import_rtl("rtl/top.v")?;

// Add analog block
let ldo = project.add_analog_block("ldo")?;
ldo.import_schematic("analog/ldo.hwt_sch")?;

// Set constraints
project.set_timing_constraints("constraints/timing.sdc")?;
project.set_floorplan("constraints/floorplan.tcl")?;

// Run flow
project.synthesize()?;
project.place_and_route()?;
project.signoff()?;

// Export
project.export_gdsii("output/gdsii/my_soc.gds")?;
```

## Related Topics

- [Unified IC Design File Format](./unified-ic-design-file-format.md)
- [RTL Capture Workflow](./rtl-capture-workflow.md)
- [Analog Schematic Workflow](./analog-schematic-workflow.md)
