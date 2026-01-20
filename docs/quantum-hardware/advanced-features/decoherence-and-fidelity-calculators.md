# Decoherence & Fidelity Calculators

## Overview

Hardware Tool provides integrated calculators for quantum coherence times, gate fidelities, and error budgets. These tools help optimize qubit designs and predict processor performance.

## Coherence Time Calculator

```rust
CoherenceCalculator {
    // T1 (energy relaxation)
    t1_sources: vec![
        T1Source::Purcell {
            resonator_kappa: 1e6,     // Hz
            detuning: 2e9,            // Hz
            coupling: 100e6,          // Hz
        },
        T1Source::DielectricLoss {
            participation: 1e-3,
            loss_tangent: 1e-6,
        },
        T1Source::QuasiparticleTunneling {
            density: 1e-7,            // per Cooper pair
        },
    ],
    
    // T2 (dephasing)
    t2_sources: vec![
        T2Source::FluxNoise {
            amplitude: 1e-6,          // Φ0/√Hz
            sensitivity: 1e9,         // Hz/Φ0
        },
        T2Source::ChargeNoise {
            amplitude: 1e-4,          // e/√Hz
            sensitivity: 1e6,         // Hz/e
        },
        T2Source::PhotonShot {
            n_thermal: 0.01,
            chi: 1e6,                 // Hz
        },
    ],
}
```

## Coherence Calculator UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Coherence Time Calculator: Q0                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Qubit Parameters:                                               │
│   Frequency: 5.00 GHz    Anharmonicity: -300 MHz               │
│   EJ/EC: 50              Resonator: 7.00 GHz                   │
│                                                                 │
│ T1 Analysis:                                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Source                  │ Contribution │ T1 Limit           │ │
│ │ ────────────────────────┼──────────────┼─────────────────── │ │
│ │ Purcell decay           │    5%        │ 2.0 ms             │ │
│ │ Dielectric loss (sub)   │   45%        │ 220 µs             │ │
│ │ Dielectric loss (cap)   │   30%        │ 330 µs             │ │
│ │ Quasiparticle tunneling │   15%        │ 670 µs             │ │
│ │ Other                   │    5%        │ 2.0 ms             │ │
│ │ ────────────────────────┼──────────────┼─────────────────── │ │
│ │ Total T1                │   100%       │ 100 µs             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ T2 Analysis:                                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Source                  │ Contribution │ T2 Limit           │ │
│ │ ────────────────────────┼──────────────┼─────────────────── │ │
│ │ T1 limit (T2 ≤ 2T1)    │   50%        │ 200 µs             │ │
│ │ 1/f flux noise          │   30%        │ 165 µs             │ │
│ │ Photon shot noise       │   15%        │ 330 µs             │ │
│ │ Charge noise            │    5%        │ 1.0 ms             │ │
│ │ ────────────────────────┼──────────────┼─────────────────── │ │
│ │ Total T2                │   100%       │ 50 µs              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Optimize] [Compare Designs] [Export] [Close]                   │
└─────────────────────────────────────────────────────────────────┘
```

## Gate Fidelity Calculator

```rust
GateFidelityCalculator {
    // Single-qubit gates
    single_qubit: SingleQubitFidelity {
        gate_time: 20e-9,             // 20 ns
        t1: 100e-6,
        t2: 50e-6,
        leakage: 0.001,               // To |2⟩ state
        control_error: 0.0005,
    },
    
    // Two-qubit gates
    two_qubit: TwoQubitFidelity {
        gate_type: GateType::CZ,
        gate_time: 200e-9,            // 200 ns
        t1: vec![100e-6, 95e-6],
        t2: vec![50e-6, 48e-6],
        zz_coupling: 0.5e6,           // Hz (residual)
        control_error: 0.002,
    },
    
    // Readout
    readout: ReadoutFidelity {
        integration_time: 1e-6,       // 1 µs
        snr: 10.0,                    // dB
        t1_during_readout: 100e-6,
    },
}
```

## Fidelity Report

```
┌─────────────────────────────────────────────────────────────────┐
│ Gate Fidelity Analysis: 5-Qubit Processor                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Single-Qubit Gates (X, Y, Z, H):                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Qubit │ Gate Time │ Fidelity │ Error Budget                 │ │
│ │ ──────┼───────────┼──────────┼───────────────────────────── │ │
│ │ Q0    │   20 ns   │  99.95%  │ T1: 0.01%, T2: 0.02%, Leak: 0.01% │
│ │ Q1    │   20 ns   │  99.94%  │ T1: 0.01%, T2: 0.02%, Leak: 0.02% │
│ │ Q2    │   20 ns   │  99.95%  │ T1: 0.01%, T2: 0.02%, Leak: 0.01% │
│ │ Q3    │   20 ns   │  99.93%  │ T1: 0.01%, T2: 0.03%, Leak: 0.02% │
│ │ Q4    │   20 ns   │  99.94%  │ T1: 0.01%, T2: 0.02%, Leak: 0.02% │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Two-Qubit Gates (CZ):                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Pair  │ Gate Time │ Fidelity │ Error Budget                 │ │
│ │ ──────┼───────────┼──────────┼───────────────────────────── │ │
│ │ Q0-Q1 │  200 ns   │  99.20%  │ T1: 0.20%, T2: 0.40%, ZZ: 0.15% │
│ │ Q1-Q2 │  200 ns   │  99.15%  │ T1: 0.20%, T2: 0.45%, ZZ: 0.15% │
│ │ Q2-Q3 │  200 ns   │  99.10%  │ T1: 0.22%, T2: 0.48%, ZZ: 0.18% │
│ │ Q3-Q4 │  200 ns   │  99.18%  │ T1: 0.20%, T2: 0.42%, ZZ: 0.16% │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Readout Fidelity:                                               │
│   Average: 98.5%    |0⟩: 99.2%    |1⟩: 97.8%                  │
│                                                                 │
│ [Optimize Gates] [Export] [Close]                               │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Calculate coherence times
let qubit = processor.get_qubit("Q0")?;

let coherence = qubit.calculate_coherence(CoherenceConfig {
    include_purcell: true,
    include_dielectric: true,
    include_flux_noise: true,
})?;

println!("T1: {} µs", coherence.t1 * 1e6);
println!("T2: {} µs", coherence.t2 * 1e6);

// Calculate gate fidelity
let fidelity = qubit.calculate_gate_fidelity(Gate::X90, GateConfig {
    duration: 20e-9,
    drag_coefficient: 0.5,
})?;

println!("X90 fidelity: {}%", fidelity * 100.0);
```

## Related Topics

- [Quantum Simulation Integration](../circuit-editor/quantum-simulation-integration.md)
- [Design for Noise Resilience Checks](./design-for-noise-resilience-checks.md)
