# Benchmarking Simulator

## Overview

Hardware Tool includes a **native SOTA benchmarking simulator** that validates every design decision in real-time. Unlike traditional post-layout verification, benchmarking is integrated into the design flow — every placement, every route, every change is immediately evaluated against physics-based models.

## Simulation Capabilities

### Integrated Analysis Types

| Analysis | Purpose | Real-time | Post-layout |
|----------|---------|-----------|-------------|
| Signal Integrity | Impedance, crosstalk, reflections | ✅ | ✅ |
| Power Integrity | IR drop, PDN impedance | ✅ | ✅ |
| Thermal | Temperature distribution | ⚠️ Simplified | ✅ Full |
| EMC/EMI | Radiation, susceptibility | ❌ | ✅ |
| Timing | Setup/hold, propagation delay | ✅ | ✅ |

### Physics Engines

```rust
PhysicsEngines {
    // Electromagnetic
    em_solver: EmSolver {
        method: Method::FDTD,  // or MoM, FEM
        frequency_range: (1e6, 10e9),
        accuracy: Accuracy::High,
    },
    
    // Thermal
    thermal_solver: ThermalSolver {
        method: Method::FiniteElement,
        steady_state: true,
        transient: true,
    },
    
    // Circuit
    circuit_solver: CircuitSolver {
        spice_compatible: true,
        frequency_domain: true,
        time_domain: true,
    },
}
```

## Real-Time Benchmarking

### Live Metrics Dashboard

```
┌─────────────────────────────────────────────────────────────────┐
│ Live Benchmarks                                    [⚙️ Settings]│
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Signal Integrity                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Impedance:  ████████████████████░░░░ 85Ω (target: 90Ω)     │ │
│ │ Crosstalk:  ██████░░░░░░░░░░░░░░░░░░ 12% (max: 20%)        │ │
│ │ Reflections:████████░░░░░░░░░░░░░░░░ 8% (max: 15%)         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Power Delivery                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ IR Drop:    ██████████░░░░░░░░░░░░░░ 45mV (max: 100mV)     │ │
│ │ PDN Z:      ████████████████░░░░░░░░ 0.08Ω (target: 0.1Ω)  │ │
│ │ Ripple:     ████░░░░░░░░░░░░░░░░░░░░ 15mV (max: 50mV)      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Thermal                                                         │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Max Temp:   ████████████████████░░░░ 72°C (max: 85°C)      │ │
│ │ Hot Spots:  2 identified                      [Show Map]    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Overall Score: 91/100  ████████████████████░░░░                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Instant Feedback

```rust
InstantFeedback {
    // Update frequency
    update_rate: UpdateRate::OnChange,
    debounce_ms: 100,
    
    // What to show
    show_impedance: true,
    show_length: true,
    show_delay: true,
    show_crosstalk_risk: true,
    
    // Visual indicators
    color_coding: true,  // Green/yellow/red
    inline_annotations: true,
    hover_details: true,
}
```

## Signal Integrity Analysis

### Impedance Analysis

```rust
ImpedanceAnalysis {
    // Trace impedance
    trace: TraceImpedance {
        target: 50.0,  // ohms
        tolerance: 10.0,  // percent
        
        // Factors considered
        geometry: true,
        stackup: true,
        frequency_dependent: true,
    },
    
    // Differential impedance
    differential: DifferentialImpedance {
        target: 100.0,
        tolerance: 10.0,
        coupling_analysis: true,
    },
    
    // Visualization
    show_impedance_profile: true,
    highlight_discontinuities: true,
}
```

### Crosstalk Analysis

```
Crosstalk Analysis: NET_CLK ↔ NET_DATA

┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  Coupling Region: 15.2mm parallel run                           │
│  Spacing: 0.15mm                                                │
│  Coupling: -28dB (NEXT), -35dB (FEXT)                          │
│                                                                 │
│  Waveform:                                                      │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │     ╭──╮                                                │   │
│  │    ╱    ╲     Aggressor (CLK)                          │   │
│  │ ──╯      ╰──────────────────────────────────────────   │   │
│  │                                                         │   │
│  │      ╭╮                                                 │   │
│  │     ╱  ╲    Victim (DATA) - Crosstalk induced          │   │
│  │ ───╯    ╰───────────────────────────────────────────   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Recommendation: Increase spacing to 0.25mm or add guard trace │
│                                                                 │
│  [Apply Fix] [Ignore] [Add to Waiver]                          │
└─────────────────────────────────────────────────────────────────┘
```

### Eye Diagram Generation

```rust
EyeDiagram {
    // Configuration
    data_rate: 5e9,  // 5 Gbps
    pattern: Pattern::Prbs7,
    samples: 10000,
    
    // Analysis
    measure_eye_height: true,
    measure_eye_width: true,
    measure_jitter: true,
    
    // Pass/fail
    mask: EyeMask::Pcie_Gen3,
}
```

## Power Integrity Analysis

### PDN Impedance

```rust
PdnAnalysis {
    // Frequency sweep
    frequency_range: (1e3, 1e9),
    points: 1000,
    
    // Target impedance
    target_impedance: TargetImpedance {
        value: 0.1,  // ohms
        method: Method::RippleBased {
            voltage: 1.0,
            ripple_percent: 5.0,
            transient_current: 0.5,
        },
    },
    
    // Components
    include_vrm: true,
    include_bulk_caps: true,
    include_mlcc: true,
    include_planes: true,
    include_vias: true,
}
```

### IR Drop Analysis

```
IR Drop Analysis: VDD_CORE (1.0V)

┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │   [Heatmap showing voltage distribution across board]   │   │
│  │                                                         │   │
│  │   🔴 0.92V (worst)                                      │   │
│  │   🟡 0.95V                                              │   │
│  │   🟢 0.98V                                              │   │
│  │   🔵 1.00V (source)                                     │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Summary:                                                       │
│    Source voltage: 1.00V                                        │
│    Minimum voltage: 0.92V (at U3 pin 45)                       │
│    Maximum drop: 80mV (8%)                                      │
│    Current: 2.5A total                                          │
│                                                                 │
│  ⚠️ Warning: Drop exceeds 5% tolerance at 3 locations          │
│                                                                 │
│  [Show Problem Areas] [Suggest Fixes] [Export Report]           │
└─────────────────────────────────────────────────────────────────┘
```

## Thermal Analysis

### Steady-State Thermal

```rust
ThermalAnalysis {
    // Environment
    ambient_temp: 25.0,  // °C
    airflow: Airflow::NaturalConvection,
    
    // Heat sources
    auto_detect_sources: true,
    power_from_simulation: true,
    
    // Analysis
    steady_state: true,
    include_copper: true,
    include_vias: true,
    include_components: true,
}
```

### Thermal Visualization

```
Thermal Analysis Results

┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │   [3D thermal heatmap of PCB]                          │   │
│  │                                                         │   │
│  │   Temperature Scale:                                    │   │
│  │   25°C ░░░░░▒▒▒▒▒▓▓▓▓▓████ 85°C                       │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Hot Spots:                                                     │
│    1. U1 (Regulator): 78°C - Within spec (max 125°C)          │
│    2. U2 (MCU): 65°C - Within spec (max 85°C)                 │
│    3. Q1 (MOSFET): 82°C - ⚠️ Near limit (max 85°C)            │
│                                                                 │
│  Recommendations:                                               │
│    • Add thermal vias under Q1 (est. -12°C)                    │
│    • Increase copper pour near U1 (est. -5°C)                  │
│                                                                 │
│  [Apply Recommendations] [Run Transient] [Export]               │
└─────────────────────────────────────────────────────────────────┘
```

## Benchmark Reports

### Comprehensive Report

```rust
BenchmarkReport {
    // Sections
    sections: vec![
        Section::ExecutiveSummary,
        Section::SignalIntegrity,
        Section::PowerIntegrity,
        Section::Thermal,
        Section::Manufacturability,
        Section::Recommendations,
    ],
    
    // Format
    format: ReportFormat::Html,  // or Pdf, Markdown
    
    // Include
    include_waveforms: true,
    include_heatmaps: true,
    include_3d_views: true,
    include_raw_data: false,
}
```

### Export Options

```bash
# Generate benchmark report
hwt benchmark --full --output report.html

# Quick signal integrity check
hwt benchmark --signal-integrity --nets "DDR_*"

# Power analysis
hwt benchmark --power --rail VDD_CORE

# Thermal analysis
hwt benchmark --thermal --ambient 40
```

## AI Integration

### AI-Guided Optimization

The benchmarking simulator feeds directly into AI optimization:

```rust
AiBenchmarkLoop {
    // Continuous feedback
    feed_to_ai: true,
    
    // AI uses benchmarks to
    guide_routing: true,
    guide_placement: true,
    suggest_improvements: true,
    
    // Optimization targets
    targets: vec![
        Target::SignalIntegrity { min_score: 90 },
        Target::PowerIntegrity { min_score: 85 },
        Target::Thermal { max_temp: 75.0 },
    ],
}
```

### Predictive Analysis

```rust
PredictiveAnalysis {
    // Before routing
    predict_routing_difficulty: true,
    predict_signal_integrity_risk: true,
    predict_thermal_issues: true,
    
    // Confidence heatmaps
    show_confidence: true,
    confidence_threshold: 0.8,
}
```

## Related Topics

- [AI-Powered Routing & Optimization](./ai-routing-optimization.md)
- [Design Rule Check](../pcb-layout/drc.md)
- [DFM Checks](../advanced-features/dfm-checks.md)
