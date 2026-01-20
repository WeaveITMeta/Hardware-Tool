# Resonance & Sensitivity Calculators

## Overview

Hardware Tool provides integrated calculators for MEMS device performance, including resonant frequency, sensitivity, noise floor, and bandwidth analysis.

## Resonance Calculator

```rust
ResonanceCalculator {
    // Mechanical parameters
    mechanical: MechanicalParams {
        mass: 30e-9,                  // kg (30 µg)
        spring_constant: 10.0,        // N/m
        damping_ratio: 0.001,         // Q = 500
    },
    
    // Calculations
    calculations: vec![
        Calculation::ResonantFrequency,
        Calculation::QualityFactor,
        Calculation::Bandwidth,
        Calculation::PhaseMargin,
    ],
}
```

## Calculator UI

```
┌─────────────────────────────────────────────────────────────────┐
│ MEMS Performance Calculator: Accelerometer                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Mechanical Parameters:                                          │
│   Proof mass: [30      ] µg                                    │
│   Spring constant: [10      ] N/m                              │
│   Damping (Q): [500     ]                                      │
│                                                                 │
│ Electrical Parameters:                                          │
│   Nominal capacitance: [1.0     ] pF                           │
│   Gap: [2.0     ] µm                                           │
│   Overlap area: [0.5     ] mm²                                 │
│                                                                 │
│ Results:                                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Parameter              │ Value      │ Unit                  │ │
│ │ ───────────────────────┼────────────┼────────────────────── │ │
│ │ Resonant frequency     │ 2,906      │ Hz                    │ │
│ │ Quality factor         │ 500        │ -                     │ │
│ │ Bandwidth (-3dB)       │ 5.8        │ Hz                    │ │
│ │ Mechanical sensitivity │ 3.4        │ nm/g                  │ │
│ │ Capacitive sensitivity │ 1.7        │ fF/g                  │ │
│ │ Brownian noise floor   │ 42         │ µg/√Hz                │ │
│ │ Full-scale range       │ ±16        │ g                     │ │
│ │ Pull-in voltage        │ 8.5        │ V                     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Frequency Response:                                             │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Gain                                                        │ │
│ │  0dB├────────────────────────────────────────────────────── │ │
│ │     │─────────────────────╲                                 │ │
│ │-20dB├──────────────────────╲────────────────────────────── │ │
│ │     │                       ╲                               │ │
│ │-40dB├────────────────────────╲──────────────────────────── │ │
│ │     └──┬────┬────┬────┬────┬─╲──┬────┬────┬────┬────┬──── │ │
│ │       10  100  1k   2k   3k  5k  10k                       │ │
│ │                    Frequency (Hz)                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Optimize] [Compare] [Export] [Close]                           │
└─────────────────────────────────────────────────────────────────┘
```

## Noise Analysis

```rust
NoiseAnalysis {
    // Noise sources
    sources: vec![
        NoiseSource::Brownian {
            temperature: 300.0,       // K
            damping: 2e-6,            // N·s/m
        },
        NoiseSource::Electronic {
            voltage_noise: 10e-9,     // V/√Hz
            current_noise: 1e-15,     // A/√Hz
        },
        NoiseSource::Quantization {
            bits: 16,
            full_scale: 16.0,         // g
        },
    ],
    
    // Total noise
    integration: NoiseIntegration {
        bandwidth: (1.0, 1000.0),     // Hz
        weighting: Weighting::Flat,
    },
}
```

## Rust API

```rust
// Calculate resonance
let sensor = project.get_sensor("accelerometer")?;

let resonance = sensor.calculate_resonance()?;
println!("Resonant frequency: {} Hz", resonance.frequency);
println!("Q factor: {}", resonance.quality_factor);

// Calculate sensitivity
let sensitivity = sensor.calculate_sensitivity()?;
println!("Mechanical: {} nm/g", sensitivity.mechanical * 1e9);
println!("Capacitive: {} fF/g", sensitivity.capacitive * 1e15);

// Calculate noise floor
let noise = sensor.calculate_noise(NoiseConfig {
    temperature: 300.0,
    bandwidth: (1.0, 1000.0),
})?;
println!("Noise floor: {} µg/√Hz", noise.total * 1e6);
```

## Related Topics

- [MEMS Simulation Integration](../device-editor/mems-simulation-integration.md)
- [Design for Reliability Checks](./design-for-reliability-checks.md)
