# MEMS Project Structure & Management

## Overview

Hardware Tool organizes MEMS and sensor projects with a unified structure supporting accelerometers, gyroscopes, pressure sensors, microphones, and custom MEMS devices. Projects integrate mechanical design with electrical readout and fabrication process flows.

## Project Structure

```
my_mems_sensor/
├── my_mems_sensor.hwt_mems           # Project manifest
├── process/                           # Process definition
│   └── polymumps.process             # Foundry process file
├── mechanical/                        # Mechanical design
│   ├── proof_mass.hwt_mech           # Proof mass design
│   ├── springs.hwt_mech              # Spring design
│   └── electrodes.hwt_mech           # Electrode design
├── electrical/                        # Electrical design
│   ├── readout.hwt_sch               # Readout circuit
│   └── interface.hwt_sch             # Interface circuit
├── layout/                            # Physical layout
│   ├── sensor.hwt_mlay               # MEMS layout
│   └── asic.hwt_lay                  # ASIC layout (if integrated)
├── simulation/                        # Simulation data
│   ├── fea/                          # FEA results
│   │   ├── modal.sim                 # Modal analysis
│   │   ├── stress.sim                # Stress analysis
│   │   └── thermal.sim               # Thermal analysis
│   ├── electrostatic/                # Electrostatic results
│   └── system/                       # System-level simulation
├── packaging/                         # Packaging design
│   ├── cavity.hwt_pkg                # Cavity package
│   └── wire_bond.hwt_pkg             # Wire bond map
└── output/                            # Manufacturing output
    ├── gdsii/                        # Mask files
    ├── process_flow/                 # Process documentation
    └── test/                         # Test procedures
```

## Project Manifest (.hwt_mems)

```toml
[project]
name = "3axis_accelerometer"
version = "1.0.0"
type = "inertial_sensor"
description = "3-axis capacitive accelerometer"

[process]
foundry = "MEMSCAP"
process = "PolyMUMPs"
device_layer = "Poly2"
device_thickness = 1.5e-6  # m

[sensor]
type = "accelerometer"
axes = 3
range = 16.0              # g
sensitivity = 1.0         # pF/g
noise_density = 100e-6    # g/√Hz
bandwidth = 1000.0        # Hz

[mechanical]
proof_mass = 30e-9        # kg
spring_constant = 10.0    # N/m
resonant_frequency = 3000.0  # Hz

[electrical]
readout_type = "capacitive"
nominal_capacitance = 1e-12  # F
supply_voltage = 3.3      # V

[packaging]
type = "LGA"
cavity = "hermetic"
atmosphere = "nitrogen"
```

## Rust API

```rust
// Create MEMS accelerometer project
let project = MEMSProject::new("3axis_accelerometer")?;

// Set process
project.set_process(Process::MEMSCAP_PolyMUMPs)?;

// Configure sensor
project.set_sensor_type(SensorType::Accelerometer {
    axes: 3,
    range: 16.0,
    bandwidth: 1000.0,
})?;

// Design mechanical elements
let proof_mass = project.design_proof_mass(ProofMassConfig {
    target_mass: 30e-9,
    perforation: true,
})?;

let springs = project.design_springs(SpringConfig {
    target_frequency: 3000.0,
    type_: SpringType::FoldedBeam,
})?;

// Run simulations
let modal = project.simulate_modal(10)?;
let stress = project.simulate_stress(16.0)?;

// Export for fabrication
project.export_gdsii("output/gdsii/accelerometer.gds")?;
```

## Related Topics

- [Unified MEMS Design File Format](./unified-mems-design-file-format.md)
- [Mechanical Capture Workflow](./mechanical-capture-workflow.md)
- [Sensor Integration Workflow](./sensor-integration-workflow.md)
