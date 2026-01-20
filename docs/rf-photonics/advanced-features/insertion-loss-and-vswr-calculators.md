# Insertion Loss & VSWR Calculators

## Overview

Hardware Tool provides integrated RF performance calculators for insertion loss, return loss, VSWR, and impedance matching analysis.

## VSWR Calculator

```rust
VSWRCalculator {
    // Input parameters
    input: VSWRInput {
        z_source: Complex::new(50.0, 0.0),
        z_load: Complex::new(75.0, 25.0),
        frequency: 2.45e9,
    },
    
    // Calculations
    outputs: vec![
        Output::ReflectionCoefficient,
        Output::VSWR,
        Output::ReturnLoss,
        Output::MismatchLoss,
    ],
}
```

## Calculator UI

```
┌─────────────────────────────────────────────────────────────────┐
│ RF Performance Calculator                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ VSWR & Impedance Matching                                   │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │                                                             │ │
│ │ Source Impedance: [50    ] + j[0     ] Ω                   │ │
│ │ Load Impedance:   [75    ] + j[25    ] Ω                   │ │
│ │ Frequency:        [2.45  ] GHz                             │ │
│ │                                                             │ │
│ │ Results:                                                    │ │
│ │ ┌─────────────────────────────────────────────────────────┐ │ │
│ │ │ Parameter              │ Value      │ Status            │ │ │
│ │ │ ───────────────────────┼────────────┼────────────────── │ │ │
│ │ │ Reflection coeff (Γ)   │ 0.28∠45°  │                   │ │ │
│ │ │ VSWR                   │ 1.78:1     │ ⚠ Marginal       │ │ │
│ │ │ Return loss            │ 11.0 dB    │ ⚠ < 15 dB        │ │ │
│ │ │ Mismatch loss          │ 0.35 dB    │ ✓ OK             │ │ │
│ │ │ Power reflected        │ 7.8%       │                   │ │ │
│ │ │ Power transmitted      │ 92.2%      │                   │ │ │
│ │ └─────────────────────────────────────────────────────────┘ │ │
│ │                                                             │ │
│ │ [Design Matching Network]                                   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Insertion Loss Calculator                                   │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │                                                             │ │
│ │ Component: [Bandpass Filter ▼]                             │ │
│ │                                                             │ │
│ │ Loss Contributions:                                         │ │
│ │ ┌─────────────────────────────────────────────────────────┐ │ │
│ │ │ Source               │ Loss (dB) │ Percentage          │ │ │
│ │ │ ─────────────────────┼───────────┼──────────────────── │ │ │
│ │ │ Conductor loss       │ 0.45      │ 45%                 │ │ │
│ │ │ Dielectric loss      │ 0.35      │ 35%                 │ │ │
│ │ │ Radiation loss       │ 0.10      │ 10%                 │ │ │
│ │ │ Mismatch loss        │ 0.10      │ 10%                 │ │ │
│ │ │ ─────────────────────┼───────────┼──────────────────── │ │ │
│ │ │ Total insertion loss │ 1.00      │ 100%                │ │ │
│ │ └─────────────────────────────────────────────────────────┘ │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Calculate] [Export] [Close]                                    │
└─────────────────────────────────────────────────────────────────┘
```

## Matching Network Synthesis

```rust
MatchingNetwork {
    // Topology options
    topologies: vec![
        Topology::LNetwork,
        Topology::PiNetwork,
        Topology::TNetwork,
        Topology::TransmissionLine,
    ],
    
    // Synthesis
    synthesis: MatchingSynthesis {
        source_z: Complex::new(50.0, 0.0),
        load_z: Complex::new(75.0, 25.0),
        frequency: 2.45e9,
        q_constraint: Some(5.0),      // Maximum Q
    },
}
```

## Rust API

```rust
// Calculate VSWR
let vswr = calculate_vswr(
    Complex::new(50.0, 0.0),   // Source
    Complex::new(75.0, 25.0),  // Load
)?;

println!("VSWR: {}:1", vswr.vswr);
println!("Return loss: {} dB", vswr.return_loss);

// Design matching network
let match_net = design_matching_network(MatchConfig {
    source_z: Complex::new(50.0, 0.0),
    load_z: Complex::new(75.0, 25.0),
    frequency: 2.45e9,
    topology: Topology::LNetwork,
})?;

println!("L = {} nH", match_net.inductor * 1e9);
println!("C = {} pF", match_net.capacitor * 1e12);
```

## Related Topics

- [Impedance Matching Rules Check](../schematic-editor/impedance-matching-rules-check.md)
- [Smith Chart & Matching](../schematic-editor/transmission-lines-and-connectivity.md)
