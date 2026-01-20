# Timing & Power Calculators

## Overview

Hardware Tool provides integrated timing and power analysis tools for IC design, supporting static timing analysis (STA), power estimation, and optimization guidance.

## Static Timing Analysis

```rust
TimingAnalysis {
    // Clock definition
    clocks: vec![
        Clock {
            name: "clk",
            period: 10.0,             // ns
            waveform: (0.0, 5.0),     // rise, fall
            uncertainty: 0.1,          // ns
        },
    ],
    
    // Analysis modes
    modes: vec![
        AnalysisMode::Setup,
        AnalysisMode::Hold,
        AnalysisMode::Recovery,
        AnalysisMode::Removal,
    ],
    
    // Corners
    corners: vec![
        Corner { name: "ss_0p81v_125c", process: "slow", voltage: 0.81, temp: 125.0 },
        Corner { name: "tt_0p90v_25c", process: "typical", voltage: 0.90, temp: 25.0 },
        Corner { name: "ff_0p99v_m40c", process: "fast", voltage: 0.99, temp: -40.0 },
    ],
}
```

## Timing Report UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Static Timing Analysis: my_soc                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Clock: clk (100 MHz, 10 ns period)                             │
│ Corner: ss_0p81v_125c (worst case)                             │
│                                                                 │
│ Summary:                                                        │
│   Setup WNS: -0.15 ns ✗ (1 violation)                          │
│   Setup TNS: -0.15 ns                                          │
│   Hold WNS: +0.05 ns ✓                                         │
│   Hold TNS: 0.00 ns                                            │
│                                                                 │
│ Critical Path:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Startpoint: reg_a/Q (rising edge clk)                       │ │
│ │ Endpoint: reg_b/D (rising edge clk)                         │ │
│ │ Path Type: max (setup)                                      │ │
│ │                                                             │ │
│ │ Point                    │ Incr   │ Path   │ Description   │ │
│ │ ─────────────────────────┼────────┼────────┼────────────── │ │
│ │ clock clk (rise)         │        │  0.00  │               │ │
│ │ reg_a/CK (DFF)           │  0.10  │  0.10  │ clock network │ │
│ │ reg_a/Q (DFF)            │  0.25  │  0.35  │ Tcq           │ │
│ │ u_and2/X (and2_2)        │  0.12  │  0.47  │               │ │
│ │ u_or4/X (or4_1)          │  0.35  │  0.82  │               │ │
│ │ u_mux/X (mux2_1)         │  0.28  │  1.10  │               │ │
│ │ ... (15 more stages)     │  8.95  │ 10.05  │               │ │
│ │ reg_b/D (DFF)            │  0.10  │ 10.15  │ setup         │ │
│ │ data required time       │        │ 10.00  │               │ │
│ │ ─────────────────────────┼────────┼────────┼────────────── │ │
│ │ slack (VIOLATED)         │        │ -0.15  │               │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix Timing] [View Path] [Export Report] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## Power Analysis

```rust
PowerAnalysis {
    // Power components
    components: PowerComponents {
        leakage: true,
        internal: true,
        switching: true,
    },
    
    // Activity
    activity: ActivitySource::VCD("sim/activity.vcd"),
    
    // Reporting
    reporting: PowerReporting {
        by_hierarchy: true,
        by_power_domain: true,
        by_clock_domain: true,
    },
}
```

## Power Report UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Power Analysis: my_soc @ 100 MHz                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Total Power: 125.3 mW                                          │
│                                                                 │
│ Power Breakdown:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Component     │ Power    │ Percentage │ Bar                 │ │
│ │ ──────────────┼──────────┼────────────┼──────────────────── │ │
│ │ Switching     │  75.2 mW │    60%     │ ████████████        │ │
│ │ Internal      │  35.1 mW │    28%     │ ██████              │ │
│ │ Leakage       │  15.0 mW │    12%     │ ███                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ By Hierarchy:                                                   │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Module        │ Power    │ Percentage │                     │ │
│ │ ──────────────┼──────────┼────────────┼                     │ │
│ │ cpu_core      │  65.2 mW │    52%     │ ██████████          │ │
│ │ memory        │  32.1 mW │    26%     │ █████               │ │
│ │ peripherals   │  18.5 mW │    15%     │ ███                 │ │
│ │ clock_tree    │   9.5 mW │     7%     │ ██                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Optimize] [Export Report] [Close]                              │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Run timing analysis
let design = project.get_design("my_soc")?;

let timing = design.analyze_timing(TimingConfig {
    clock: Clock::new("clk", 10.0),
    corners: Corner::all_pvt(),
})?;

println!("Setup WNS: {} ns", timing.setup_wns);
println!("Hold WNS: {} ns", timing.hold_wns);

// Run power analysis
let power = design.analyze_power(PowerConfig {
    activity: Activity::from_vcd("sim/activity.vcd"),
    clock_frequency: 100e6,
})?;

println!("Total power: {} mW", power.total * 1000.0);
```

## Related Topics

- [Command-Line EDA Flows](./command-line-eda-flows.md)
- [Layout Constraints & Floorplanning](./layout-constraints-and-floorplanning.md)
