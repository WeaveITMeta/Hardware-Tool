# Quantum Project Structure & Management

## Overview

Hardware Tool organizes quantum hardware projects with a unified structure supporting superconducting, photonic, trapped ion, and silicon spin qubit systems. Projects integrate circuit-level design with physical layout and control system configuration.

## Project Structure

```
my_quantum_processor/
├── my_quantum_processor.hwt_quantum  # Project manifest
├── circuits/                          # Quantum circuits
│   ├── algorithms/                   # Algorithm implementations
│   │   ├── grover.qasm              # Grover's algorithm
│   │   └── vqe.qasm                 # VQE ansatz
│   └── calibration/                  # Calibration sequences
│       ├── rabi.qasm                # Rabi oscillation
│       └── t1_t2.qasm               # Coherence measurement
├── physical/                          # Physical design
│   ├── qubits/                       # Qubit designs
│   │   ├── transmon_v1.hwt_qubit    # Transmon design
│   │   └── resonator_v1.hwt_res     # Resonator design
│   ├── layout/                       # Chip layout
│   │   └── processor.hwt_qlay       # Full processor layout
│   └── control/                      # Control lines
│       ├── xy_lines.hwt_ctrl        # XY control
│       └── z_lines.hwt_ctrl         # Z control (flux)
├── simulation/                        # Simulation data
│   ├── em/                           # EM simulation results
│   ├── hamiltonian/                  # Hamiltonian simulation
│   └── pulses/                       # Pulse optimization
├── calibration/                       # Calibration data
│   ├── frequencies.json             # Qubit frequencies
│   ├── couplings.json               # Coupling strengths
│   └── gates.json                   # Gate parameters
├── control/                           # Control system
│   ├── waveforms/                    # Pulse waveforms
│   ├── sequences/                    # Gate sequences
│   └── config/                       # Hardware config
└── output/                            # Manufacturing output
    ├── gdsii/                        # Mask files
    ├── process/                      # Process documentation
    └── test/                         # Test procedures
```

## Project Manifest (.hwt_quantum)

```toml
[project]
name = "5_qubit_processor"
version = "1.0.0"
type = "superconducting"
description = "5-qubit transmon processor"

[technology]
qubit_type = "transmon"
substrate = "silicon"
superconductor = "aluminum"
junction_type = "Al/AlOx/Al"

[qubits]
count = 5
frequency_range = [4.5e9, 5.5e9]  # Hz
anharmonicity = -300e6            # Hz
t1_target = 100e-6                # s
t2_target = 50e-6                 # s

[resonators]
count = 5
frequency_range = [6.5e9, 7.5e9]  # Hz
coupling_strength = 100e6         # Hz

[topology]
type = "linear"
couplings = [
    ["Q0", "Q1", 30e6],
    ["Q1", "Q2", 30e6],
    ["Q2", "Q3", 30e6],
    ["Q3", "Q4", 30e6],
]

[control]
xy_frequency = 5e9                # Hz
z_bandwidth = 500e6               # Hz
readout_frequency = 7e9           # Hz

[fabrication]
foundry = "custom"
process = "single_layer_Al"
junction_dose = 500               # µC/cm²
```

## Rust API

```rust
// Create quantum processor project
let project = QuantumProject::new("5_qubit_processor")?;

// Configure technology
project.set_technology(QuantumTechnology::Superconducting {
    qubit_type: QubitType::Transmon,
    substrate: Substrate::Silicon,
    superconductor: Superconductor::Aluminum,
})?;

// Add qubits
for i in 0..5 {
    project.add_qubit(Transmon::new(
        format!("Q{}", i),
        5.0e9 + i as f64 * 0.15e9,
    ))?;
}

// Define topology
project.set_topology(Topology::Linear)?;

// Add readout resonators
for i in 0..5 {
    project.add_resonator(format!("R{}", i), 7.0e9 + i as f64 * 0.2e9)?;
}

// Generate layout
project.generate_layout()?;

// Run EM simulation
project.simulate_em()?;

// Export for fabrication
project.export_gdsii("output/gdsii/processor.gds")?;
```

## Related Topics

- [Unified Quantum Design File Format](./unified-quantum-design-file-format.md)
- [Quantum Circuit Capture Workflow](./quantum-circuit-capture-workflow.md)
- [Qubit Layout Workflow](./qubit-layout-workflow.md)
