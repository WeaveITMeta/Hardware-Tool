# Power Integrity Calculators

## Overview

Hardware Tool provides integrated power integrity analysis tools for advanced packaging, including IR drop, PDN impedance, decoupling optimization, and power delivery network analysis.

## PDN Impedance Calculator

```rust
PDNImpedanceCalculator {
    // Target impedance
    target: TargetImpedance {
        voltage: 0.9,                 // V
        ripple: 0.05,                 // 5%
        max_current_step: 10.0,       // A
        // Z_target = V × ripple / I_step = 4.5 mΩ
    },
    
    // PDN components
    components: PDNComponents {
        vrm: VRM {
            output_impedance: 1e-3,   // 1 mΩ
            bandwidth: 100e3,         // 100 kHz
        },
        bulk_caps: vec![
            Capacitor { value: 100e-6, esr: 5e-3, esl: 1e-9 },
        ],
        mlcc: vec![
            Capacitor { value: 10e-6, esr: 3e-3, esl: 0.5e-9 },
            Capacitor { value: 100e-9, esr: 10e-3, esl: 0.3e-9 },
        ],
        die_caps: vec![
            Capacitor { value: 100e-12, esr: 50e-3, esl: 10e-12 },
        ],
    },
}
```

## Calculator UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Power Integrity Calculator: HPC Accelerator                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Power Domain: VDD_CORE (0.9V, 150W)                            │
│                                                                 │
│ Target Impedance:                                               │
│   Voltage: [0.9   ] V    Ripple: [5     ] %                    │
│   Max ΔI: [10    ] A     Z_target: 4.5 mΩ                      │
│                                                                 │
│ PDN Impedance Profile:                                          │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ |Z| (mΩ)                                                    │ │
│ │  100├────────────────────────────────────────────────────── │ │
│ │     │╲                                                      │ │
│ │   10├──╲─────────────────────────────────────────────────── │ │
│ │     │   ╲    ╱╲                                             │ │
│ │    1├────╲──╱──╲────────────────────────────────────────── │ │
│ │     │     ╲╱    ╲_______________                            │ │
│ │  0.1├───────────────────────────────────────────────────── │ │
│ │     └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬──── │ │
│ │       1k  10k 100k  1M  10M 100M  1G                       │ │
│ │                    Frequency (Hz)                           │ │
│ │                                                             │ │
│ │ ─── Actual    ─ ─ Target (4.5 mΩ)                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ IR Drop Analysis:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Location          │ Voltage │ Drop   │ Status              │ │
│ │ ──────────────────┼─────────┼────────┼──────────────────── │ │
│ │ VRM output        │ 0.900 V │ 0 mV   │ Reference           │ │
│ │ Substrate bump    │ 0.892 V │ 8 mV   │ ✓ OK               │ │
│ │ Interposer TSV    │ 0.888 V │ 12 mV  │ ✓ OK               │ │
│ │ Die µ-bump        │ 0.882 V │ 18 mV  │ ✓ OK               │ │
│ │ Die core (worst)  │ 0.865 V │ 35 mV  │ ⚠ 3.9% drop        │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Decoupling Recommendation:                                      │
│   Add 20× 100nF MLCC near die edge                             │
│   Add 5× 10µF bulk caps on substrate                           │
│                                                                 │
│ [Optimize Decoupling] [Run Simulation] [Export] [Close]         │
└─────────────────────────────────────────────────────────────────┘
```

## IR Drop Analysis

```rust
IRDropAnalysis {
    // Power network
    network: PowerNetwork {
        layers: vec![
            PowerLayer { name: "RDL_M4", sheet_resistance: 0.01 },
            PowerLayer { name: "RDL_M3", sheet_resistance: 0.01 },
            PowerLayer { name: "TSV", resistance_per_via: 0.05 },
            PowerLayer { name: "Substrate", sheet_resistance: 0.005 },
        ],
        vias: ViaConfig {
            count: 1000,
            resistance: 0.05,         // Ω per via
        },
    },
    
    // Current distribution
    current: CurrentDistribution {
        total: 166.0,                 // A (150W @ 0.9V)
        distribution: Distribution::Uniform,
    },
    
    // Analysis
    analysis: IRDropConfig {
        max_drop: 0.045,              // 5% of 0.9V
        hotspot_threshold: 0.03,
    },
}
```

## Rust API

```rust
// Calculate target impedance
let target_z = calculate_target_impedance(
    0.9,    // Voltage
    0.05,   // Ripple (5%)
    10.0,   // Max current step
)?;
println!("Target Z: {} mΩ", target_z * 1000.0);

// Analyze PDN impedance
let pdn = system.analyze_pdn(PDNConfig {
    power_domain: "VDD_CORE",
    frequency_range: (1e3, 1e9),
})?;

// Check against target
let violations = pdn.check_target(target_z)?;
if violations.is_empty() {
    println!("PDN meets target impedance!");
}

// Analyze IR drop
let ir_drop = system.analyze_ir_drop(IRDropConfig {
    power_domain: "VDD_CORE",
    current: 166.0,
})?;

println!("Max IR drop: {} mV", ir_drop.max_drop * 1000.0);
```

## Related Topics

- [Thermal-Mechanical Export/Import](../3d-visualization-thermal/thermal-mechanical-export-import.md)
- [Signal Integrity Analysis](../../ic-design/analog-mixed-signal/signal-integrity-analysis.md)
