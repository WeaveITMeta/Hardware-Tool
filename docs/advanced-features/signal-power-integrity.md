# Signal Integrity & Power Integrity Analysis

## Overview

Hardware Tool includes comprehensive Signal Integrity (SI) and Power Integrity (PI) analysis for high-speed digital and mixed-signal PCB designs. The integrated 2.5D/3D field solver enables accurate prediction of signal quality, crosstalk, power delivery network (PDN) performance, and electromagnetic behavior—all within the native Rust environment.

### Why SI/PI Matters

| Speed Class | Data Rate | SI/PI Critical? | Consequences of Ignoring |
|-------------|-----------|-----------------|--------------------------|
| **Low-speed** | <50 MHz | Optional | Usually works |
| **Medium-speed** | 50-500 MHz | Recommended | Marginal timing, EMI |
| **High-speed** | 500 MHz-5 GHz | Required | Non-functional, compliance fail |
| **Ultra-high-speed** | >5 GHz | Critical | Complete failure |

### Integrated Analysis Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Schematic  │───▶│   Layout    │───▶│  SI/PI Sim  │───▶│  Validate   │
│   Design    │    │   Design    │    │   Engine    │    │   Results   │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                  │                  │                  │
       ▼                  ▼                  ▼                  ▼
   Net classes       Stackup def       S-parameters        Eye diagrams
   Constraints       Trace geom        Impedance           Timing margin
   IBIS models       Via models        Crosstalk           PDN impedance
```

## Signal Integrity Analysis

### Transmission Line Modeling

Every trace is modeled as a transmission line with distributed RLGC parameters:

```
┌────────────────────────────────────────────────────────────┐
│                    Transmission Line Model                  │
├────────────────────────────────────────────────────────────┤
│                                                            │
│    R (Ω/m)      L (H/m)       G (S/m)       C (F/m)       │
│   ┌─────┐      ┌─────┐       ┌─────┐       ┌─────┐        │
│ ──┤     ├──────┤     ├───────┤     ├───────┤     ├──      │
│   └─────┘      └─────┘       └─────┘       └─────┘        │
│                    │             │             │           │
│                    └─────────────┴─────────────┘           │
│                              GND                           │
│                                                            │
│  Z₀ = √(L/C)           Characteristic impedance            │
│  v = 1/√(LC)           Propagation velocity                │
│  τ = length × √(LC)    Propagation delay                   │
│  α = R/(2Z₀) + GZ₀/2   Attenuation constant               │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### Impedance Control

```rust
ImpedanceControl {
    // Target impedances
    targets: vec![
        ImpedanceTarget {
            net_class: "DDR4_DQ",
            topology: Topology::SingleEnded,
            target_z0: 40.0,           // Ω
            tolerance: 0.10,           // ±10%
        },
        ImpedanceTarget {
            net_class: "USB3_DP",
            topology: Topology::DifferentialPair,
            target_z0: 90.0,           // Ω differential
            tolerance: 0.10,
        },
        ImpedanceTarget {
            net_class: "PCIE_TX",
            topology: Topology::DifferentialPair,
            target_z0: 85.0,
            tolerance: 0.05,           // ±5% (tighter)
        },
    ],
    
    // Stackup-aware calculation
    stackup: StackupRef::Current,
    
    // Include frequency-dependent effects
    frequency_dependent: true,
    frequency_range: (1e6, 10e9),  // 1 MHz to 10 GHz
}
```

### 2.5D Field Solver

Accurate impedance extraction using Method of Moments (MoM):

```rust
FieldSolver2D5 {
    // Solver configuration
    method: SolverMethod::MethodOfMoments,
    
    // Mesh settings
    mesh: MeshConfig {
        edge_mesh_density: 10,        // Elements per edge
        adaptive_refinement: true,
        max_elements: 100_000,
    },
    
    // Frequency sweep
    frequency: FrequencySweep {
        start: 1e6,                   // 1 MHz
        stop: 20e9,                   // 20 GHz
        points_per_decade: 10,
    },
    
    // Material models
    conductor_model: ConductorModel::SurfaceRoughness {
        rms_roughness: 0.5e-6,        // 0.5 μm RMS
        model: RoughnessModel::HammerstadJensen,
    },
    
    dielectric_model: DielectricModel::WidebandDebye {
        dk_at_1ghz: 4.2,
        df_at_1ghz: 0.02,
    },
}
```

### S-Parameter Extraction

Extract scattering parameters for any net or net group:

```rust
SParameterExtraction {
    // Nets to analyze
    nets: vec!["USB3_DP", "USB3_DN"],
    
    // Port definitions
    ports: vec![
        Port { net: "USB3_DP", location: PortLocation::ComponentPin("U1", "D+") },
        Port { net: "USB3_DN", location: PortLocation::ComponentPin("U1", "D-") },
        Port { net: "USB3_DP", location: PortLocation::ComponentPin("J1", "D+") },
        Port { net: "USB3_DN", location: PortLocation::ComponentPin("J1", "D-") },
    ],
    
    // Output format
    output: SParamOutput {
        format: TouchstoneFormat::S4P,  // 4-port Touchstone
        frequency_unit: FrequencyUnit::GHz,
        parameter: SParamType::S,       // S, Y, or Z
        format_type: DataFormat::MagnitudeAngle,
    },
}
```

### S-Parameter Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ S-Parameter Analysis: USB3_DP/DN                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Insertion Loss (S21)              Return Loss (S11)             │
│ ┌─────────────────────────┐      ┌─────────────────────────┐   │
│ │ dB                      │      │ dB                      │   │
│ │  0├─────────────────────│      │  0├─────────────────────│   │
│ │ -5├──────────╲──────────│      │-10├─────────────────────│   │
│ │-10├───────────╲─────────│      │-20├──────╲──────────────│   │
│ │-15├────────────╲────────│      │-30├───────╲─────────────│   │
│ │-20├─────────────╲───────│      │-40├────────╲────────────│   │
│ │   └──┬──┬──┬──┬──┬──┬───│      │   └──┬──┬──┬──┬──┬──┬───│   │
│ │      1  2  4  6  8  10  │      │      1  2  4  6  8  10  │   │
│ │           GHz           │      │           GHz           │   │
│ └─────────────────────────┘      └─────────────────────────┘   │
│                                                                 │
│ ✓ S21 > -3dB @ 5GHz (USB3 spec: -3.5dB max)                   │
│ ✓ S11 < -10dB @ 5GHz (USB3 spec: -10dB max)                   │
│                                                                 │
│ [Export Touchstone] [View Smith Chart] [Time Domain] [Close]    │
└─────────────────────────────────────────────────────────────────┘
```

### Eye Diagram Generation

Predict signal quality at the receiver:

```rust
EyeDiagramAnalysis {
    // Signal configuration
    signal: SignalConfig {
        net: "DDR4_DQ0",
        data_rate: 3200e6,            // 3.2 Gbps (DDR4-3200)
        pattern: BitPattern::PRBS7,   // Pseudo-random
        bits: 10_000,                 // Bits to simulate
    },
    
    // Driver model
    driver: DriverModel::IBIS {
        model_file: "ddr4_controller.ibs",
        component: "DDR4_PHY",
        pin: "DQ0",
    },
    
    // Receiver model
    receiver: ReceiverModel::IBIS {
        model_file: "ddr4_sdram.ibs",
        component: "DDR4_SDRAM",
        pin: "DQ0",
    },
    
    // Channel
    channel: ChannelModel::Extracted,  // Use field solver results
    
    // Analysis
    analysis: EyeAnalysis {
        include_jitter: true,
        include_noise: true,
        isi_bits: 5,                  // Inter-symbol interference
        bathtub_curve: true,
        ber_target: 1e-12,
    },
}
```

### Eye Diagram Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ Eye Diagram: DDR4_DQ0 @ 3.2 Gbps                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│      ╱╲      ╱╲      ╱╲      ╱╲      ╱╲      ╱╲                │
│     ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲               │
│    ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲              │
│   ╱      ╲╱      ╲╱      ╲╱      ╲╱      ╲╱      ╲             │
│ ─────────────────────────────────────────────────────           │
│   ╲      ╱╲      ╱╲      ╱╲      ╱╲      ╱╲      ╱             │
│    ╲    ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲    ╱  ╲    ╱              │
│     ╲  ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲  ╱               │
│      ╲╱      ╲╱      ╲╱      ╲╱      ╲╱      ╲╱                │
│                                                                 │
│ Eye Metrics:                                                    │
│   Eye Height:     320 mV (spec: 250 mV min) ✓                  │
│   Eye Width:      0.42 UI (spec: 0.35 UI min) ✓                │
│   Jitter (p-p):   45 ps (spec: 70 ps max) ✓                    │
│   Rise Time:      85 ps                                         │
│   Fall Time:      82 ps                                         │
│                                                                 │
│ Timing Margin:    +67 ps (21% margin)                          │
│ Voltage Margin:   +70 mV (28% margin)                          │
│                                                                 │
│ [Bathtub Curve] [Histogram] [Export] [Optimize] [Close]         │
└─────────────────────────────────────────────────────────────────┘
```

### IBIS Model Support

Industry-standard I/O buffer models:

```rust
IBISModelLibrary {
    // Model search paths
    search_paths: vec![
        "./ibis_models/",
        "~/.hwt/ibis/",
    ],
    
    // Loaded models
    models: vec![
        IBISModel {
            file: "stm32f4.ibs",
            components: vec!["STM32F407VG"],
            pins: 100,
            io_types: vec![
                IOType::PushPull,
                IOType::OpenDrain,
                IOType::ThreeState,
            ],
        },
        IBISModel {
            file: "ddr4_sdram.ibs",
            components: vec!["MT40A512M16"],
            pins: 96,
            io_types: vec![IOType::SSTL, IOType::POD],
        },
    ],
    
    // Corner selection
    corner: IBISCorner::Typical,  // Slow, Typical, Fast
}
```

### Crosstalk Analysis

Predict coupling between adjacent traces:

```rust
CrosstalkAnalysis {
    // Aggressor nets
    aggressors: vec!["CLK_100M", "DATA_BUS[0:7]"],
    
    // Victim nets
    victims: vec!["ANALOG_IN", "RESET_N"],
    
    // Analysis type
    analysis: CrosstalkType::Both,  // NEXT, FEXT, or Both
    
    // Thresholds
    thresholds: CrosstalkThresholds {
        max_next: -20.0,              // dB
        max_fext: -25.0,              // dB
        max_coupled_voltage: 0.1,     // V peak
    },
    
    // Simulation
    simulation: CrosstalkSimulation {
        aggressor_pattern: BitPattern::WorstCase,
        frequency_range: (1e6, 1e9),
        time_domain: true,
    },
}
```

### Crosstalk Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ Crosstalk Analysis                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Aggressor: CLK_100M                                             │
│ Victim: ANALOG_IN                                               │
│                                                                 │
│ Coupling Region:                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │  CLK_100M  ════════════════════════════════════════════     │ │
│ │                    ↕ 0.15mm gap (coupling zone)             │ │
│ │  ANALOG_IN ════════════════════════════════════════════     │ │
│ │            ├──────── 25mm parallel run ────────┤            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Results:                                                        │
│   NEXT: -18.5 dB ⚠ (limit: -20 dB)                            │
│   FEXT: -22.3 dB ✓ (limit: -25 dB)                            │
│   Peak coupled voltage: 85 mV                                   │
│                                                                 │
│ Recommendations:                                                │
│   1. Increase spacing to 0.25mm → NEXT: -24 dB ✓              │
│   2. Add ground guard trace → NEXT: -28 dB ✓                  │
│   3. Route on different layers → NEXT: -35 dB ✓               │
│                                                                 │
│ [Apply Fix #1] [Apply Fix #2] [Apply Fix #3] [Close]            │
└─────────────────────────────────────────────────────────────────┘
```

## Power Integrity Analysis

### Power Delivery Network (PDN) Modeling

```
┌─────────────────────────────────────────────────────────────────┐
│                    PDN Impedance Model                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   VRM        Bulk Caps      Ceramic Caps     Die Cap    IC     │
│  ┌───┐       ┌─────┐        ┌─────┐         ┌───┐     ┌───┐   │
│  │   │──┬────┤     ├────┬───┤     ├────┬────┤   ├─────┤   │   │
│  │VRM│  │    │100µF│    │   │10µF │    │    │1nF│     │IC │   │
│  │   │  │    │     │    │   │     │    │    │   │     │   │   │
│  └───┘  │    └─────┘    │   └─────┘    │    └───┘     └───┘   │
│         │               │              │                       │
│        ─┴─             ─┴─            ─┴─                      │
│        GND             GND            GND                      │
│                                                                 │
│  Z_target = ΔV_max / ΔI_max                                    │
│                                                                 │
│  Example: 50mV ripple, 5A transient → Z_target = 10mΩ          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### PDN Impedance Analysis

```rust
PDNAnalysis {
    // Power rail
    power_rail: "VDD_CORE",
    voltage: 1.0,                     // V
    
    // Current profile
    current: CurrentProfile {
        dc_current: 2.0,              // A
        transient_di_dt: 5.0,         // A/ns
        transient_duration: 10e-9,    // 10 ns
    },
    
    // Target impedance
    target_impedance: TargetImpedance {
        max_ripple: 0.05,             // 5% = 50mV
        frequency_range: (1e3, 1e9),  // 1 kHz to 1 GHz
    },
    
    // Decoupling capacitors
    decoupling: DecouplingConfig {
        auto_place: true,
        capacitor_library: "murata_grm.lib",
        optimization: DecouplingOptimization::MinimizeCost,
    },
    
    // Analysis
    analysis: PDNAnalysisType {
        impedance_vs_frequency: true,
        transient_response: true,
        resonance_detection: true,
        capacitor_contribution: true,
    },
}
```

### PDN Impedance Plot

```
┌─────────────────────────────────────────────────────────────────┐
│ PDN Impedance: VDD_CORE (1.0V)                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Impedance (Ω)                                                   │
│     1   ├─────────────────────────────────────────────────────  │
│         │╲                                                      │
│   100m  ├──╲────────────────────────────────────────────────── │
│         │   ╲        ╱╲                                         │
│    10m  ├────╲──────╱──╲────────────────────────────────────── │
│         │     ╲    ╱    ╲    ╱╲                                │
│     1m  ├──────╲──╱──────╲──╱──╲────────────────────────────── │
│         │       ╲╱        ╲╱    ╲                              │
│   100µ  ├────────────────────────╲─────────────────────────── │
│         │                         ╲                            │
│    10µ  ├──────────────────────────╲───────────────────────── │
│         └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬───  │
│           1k   10k  100k  1M   10M  100M  1G                   │
│                        Frequency (Hz)                          │
│                                                                 │
│ ─── Actual    ─ ─ Target (10mΩ)                                │
│                                                                 │
│ ⚠ Resonance at 2.5 MHz: 45mΩ (exceeds 10mΩ target)            │
│ ⚠ Anti-resonance at 50 MHz: 25mΩ                              │
│                                                                 │
│ Recommendations:                                                │
│   1. Add 22µF capacitor near U1 → Fixes 2.5 MHz resonance     │
│   2. Add 100nF capacitors (x4) → Reduces 50 MHz peak          │
│                                                                 │
│ [Auto-Optimize] [Add Capacitor] [Export] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

### Decoupling Capacitor Optimization

```rust
DecouplingOptimizer {
    // Optimization goals
    goals: OptimizationGoals {
        meet_target_impedance: true,
        minimize_capacitor_count: true,
        minimize_cost: true,
        minimize_area: false,
    },
    
    // Capacitor library
    library: CapacitorLibrary {
        vendors: vec!["Murata", "Samsung", "TDK"],
        packages: vec!["0201", "0402", "0603", "0805"],
        values: vec![100e-12, 1e-9, 10e-9, 100e-9, 1e-6, 10e-6, 100e-6],
    },
    
    // Placement constraints
    placement: PlacementConstraints {
        max_distance_from_ic: 5.0,    // mm
        prefer_bottom_side: true,
        avoid_via_fanout: true,
    },
    
    // Algorithm
    algorithm: OptimizationAlgorithm::GeneticAlgorithm {
        population: 100,
        generations: 500,
        mutation_rate: 0.1,
    },
}
```

### Voltage Drop (IR Drop) Analysis

```rust
IRDropAnalysis {
    // Power nets
    power_nets: vec!["VDD_3V3", "VDD_1V8", "VDD_CORE"],
    
    // Current sources (from components)
    current_sources: CurrentSourceMethod::FromSchematic,
    
    // Analysis
    analysis: IRDropConfig {
        dc_analysis: true,
        ac_analysis: true,
        frequency: 100e6,             // For AC
    },
    
    // Thresholds
    thresholds: IRDropThresholds {
        max_dc_drop: 0.05,            // 5% of nominal
        max_ac_drop: 0.03,            // 3% of nominal
    },
    
    // Visualization
    visualization: IRDropVisualization {
        voltage_heatmap: true,
        current_density: true,
        hotspot_markers: true,
    },
}
```

### IR Drop Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ IR Drop Analysis: VDD_3V3                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Voltage Distribution (V)                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   3.30V ████████████████████████████████████████████████   │ │
│ │         ████████████████████████████████████████████████   │ │
│ │   3.28V ████████████████████░░░░████████████████████████   │ │
│ │         ████████████████░░░░░░░░░░░░████████████████████   │ │
│ │   3.26V ████████████░░░░░░░░░░░░░░░░░░░░████████████████   │ │
│ │         ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████████   │ │
│ │   3.24V ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████   │ │
│ │         ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████   │ │
│ │   3.22V ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │ │
│ │                          ▲                                  │ │
│ │                     U1 (MCU)                                │ │
│ │                     Min: 3.21V                              │ │
│ │                     Drop: 2.7%                              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Summary:                                                        │
│   Input voltage: 3.30V                                          │
│   Min voltage: 3.21V at U1 pin 45                              │
│   Max drop: 90mV (2.7%) ⚠ (limit: 5%)                         │
│   Total current: 1.2A                                           │
│                                                                 │
│ Hotspots:                                                       │
│   1. U1 VDD pins: 3.21V (widen trace from 0.2mm to 0.5mm)     │
│   2. Via bottleneck at (25, 30): Add 4 more vias              │
│                                                                 │
│ [Apply Fixes] [Current Density] [Export] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## Stackup-Aware Analysis

### Stackup Definition

```rust
Stackup {
    name: "6-layer_1.6mm",
    total_thickness: 1.6,             // mm
    
    layers: vec![
        Layer { name: "Top", type_: LayerType::Signal, thickness: 0.035, material: "Copper" },
        Layer { name: "PP1", type_: LayerType::Dielectric, thickness: 0.2, material: "FR4", dk: 4.2, df: 0.02 },
        Layer { name: "GND1", type_: LayerType::Plane, thickness: 0.035, material: "Copper" },
        Layer { name: "Core", type_: LayerType::Dielectric, thickness: 0.8, material: "FR4", dk: 4.2, df: 0.02 },
        Layer { name: "PWR", type_: LayerType::Plane, thickness: 0.035, material: "Copper" },
        Layer { name: "PP2", type_: LayerType::Dielectric, thickness: 0.2, material: "FR4", dk: 4.2, df: 0.02 },
        Layer { name: "Bot", type_: LayerType::Signal, thickness: 0.035, material: "Copper" },
    ],
    
    // Impedance targets per layer
    impedance_targets: vec![
        LayerImpedance { layer: "Top", single_ended: 50.0, differential: 100.0 },
        LayerImpedance { layer: "Bot", single_ended: 50.0, differential: 100.0 },
    ],
}
```

### Impedance Calculator

```
┌─────────────────────────────────────────────────────────────────┐
│ Stackup Impedance Calculator                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Layer: Top (microstrip)                                         │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │     ┌─────────┐                                             │ │
│ │     │  Trace  │ W = 0.15mm, T = 35µm                       │ │
│ │ ════╧═════════╧════════════════════════════════════════     │ │
│ │     │← H=0.2mm →│  εr = 4.2                                │ │
│ │ ════════════════════════════════════════════════════════    │ │
│ │     GND Plane                                               │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Single-Ended:                                                   │
│   Target: 50Ω    Calculated: 49.2Ω    Error: -1.6% ✓          │
│   Trace width for 50Ω: 0.152mm                                 │
│                                                                 │
│ Differential Pair:                                              │
│   Target: 100Ω   Calculated: 98.5Ω    Error: -1.5% ✓          │
│   Trace width: 0.12mm, Gap: 0.15mm                             │
│                                                                 │
│ Propagation Delay: 6.8 ps/mm                                    │
│ Effective εr: 3.2                                               │
│                                                                 │
│ [Apply to Net Class] [Export] [Close]                           │
└─────────────────────────────────────────────────────────────────┘
```

## Time-Domain Reflectometry (TDR)

### TDR Simulation

```rust
TDRSimulation {
    // Net to analyze
    net: "DDR4_CLK",
    
    // TDR pulse
    pulse: TDRPulse {
        rise_time: 35e-12,            // 35 ps
        amplitude: 1.0,               // V
        source_impedance: 50.0,       // Ω
    },
    
    // Analysis
    analysis: TDRAnalysis {
        time_window: 10e-9,           // 10 ns
        resolution: 1e-12,            // 1 ps
    },
}
```

### TDR Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ TDR Analysis: DDR4_CLK                                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Impedance (Ω)                                                   │
│    70├─────────────────────────────────────────────────────────│
│      │                    ╱╲                                    │
│    60├───────────────────╱──╲───────────────────────────────── │
│      │                  ╱    ╲                                  │
│    50├─────────────────╱──────╲─────────────────────────────── │
│      │    ╱╲          ╱        ╲                               │
│    40├───╱──╲────────╱──────────╲───────────────────────────── │
│      │  ╱    ╲      ╱            ╲                             │
│    30├─╱──────╲────╱──────────────╲─────────────────────────── │
│      └──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──── │
│         0  1  2  3  4  5  6  7  8  9  10                       │
│                     Time (ns)                                   │
│                                                                 │
│ Discontinuities:                                                │
│   @ 1.2ns: Via transition (35Ω) - Add via stitching            │
│   @ 3.5ns: Connector (65Ω) - Impedance mismatch                │
│   @ 5.8ns: IC pad (45Ω) - Normal                               │
│                                                                 │
│ [Locate on PCB] [Optimize] [Export] [Close]                     │
└─────────────────────────────────────────────────────────────────┘
```

## Continuous SI/PI Verification

### Real-Time Checks

Every routing action triggers instant SI/PI verification:

```rust
ContinuousVerification {
    // Enable real-time checks
    enabled: true,
    
    // Check triggers
    triggers: vec![
        Trigger::TraceRouted,
        Trigger::ViaPlaced,
        Trigger::ComponentMoved,
        Trigger::StackupChanged,
    ],
    
    // Checks to run
    checks: vec![
        Check::ImpedanceDeviation { threshold: 0.10 },
        Check::CouplingLength { max_parallel: 10.0 },  // mm
        Check::ReturnPathContinuity,
        Check::ViaStubLength { max_stub: 0.2 },        // mm
    ],
    
    // Feedback
    feedback: FeedbackConfig {
        inline_warnings: true,
        impedance_colormap: true,
        coupling_highlights: true,
    },
}
```

### SI/PI Dashboard

```
┌─────────────────────────────────────────────────────────────────┐
│ SI/PI Dashboard                                          [Live] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Signal Integrity                    Power Integrity             │
│ ┌─────────────────────────────┐    ┌─────────────────────────┐ │
│ │ DDR4 Bus         ✓ Pass    │    │ VDD_CORE    ⚠ Warning  │ │
│ │ USB3             ✓ Pass    │    │ VDD_IO      ✓ Pass     │ │
│ │ PCIe             ✓ Pass    │    │ VDD_3V3     ✓ Pass     │ │
│ │ HDMI             ⚠ Warning │    │ VDD_5V      ✓ Pass     │ │
│ │ Ethernet         ✓ Pass    │    │                         │ │
│ └─────────────────────────────┘    └─────────────────────────┘ │
│                                                                 │
│ Active Warnings (2):                                            │
│   ⚠ HDMI_D2: Impedance 58Ω (target 50Ω ±10%)                  │
│   ⚠ VDD_CORE: Resonance at 5MHz exceeds target                │
│                                                                 │
│ Quick Actions:                                                  │
│   [Fix HDMI Impedance] [Optimize VDD_CORE Decoupling]          │
│                                                                 │
│ Last Full Analysis: 2 minutes ago                               │
│ [Run Full Analysis] [Export Report] [Settings]                  │
└─────────────────────────────────────────────────────────────────┘
```

## Rust Implementation

### Core SI/PI Engine

```rust
use nalgebra::{DMatrix, DVector, Complex};
use ndarray::Array2;

// ═══════════════════════════════════════════════════════════════
// Transmission Line RLGC Extraction
// ═══════════════════════════════════════════════════════════════

struct TransmissionLine {
    r: f64,  // Resistance per unit length (Ω/m)
    l: f64,  // Inductance per unit length (H/m)
    g: f64,  // Conductance per unit length (S/m)
    c: f64,  // Capacitance per unit length (F/m)
}

impl TransmissionLine {
    fn characteristic_impedance(&self, freq: f64) -> Complex<f64> {
        let omega = 2.0 * std::f64::consts::PI * freq;
        let z = Complex::new(self.r, omega * self.l);
        let y = Complex::new(self.g, omega * self.c);
        (z / y).sqrt()
    }
    
    fn propagation_constant(&self, freq: f64) -> Complex<f64> {
        let omega = 2.0 * std::f64::consts::PI * freq;
        let z = Complex::new(self.r, omega * self.l);
        let y = Complex::new(self.g, omega * self.c);
        (z * y).sqrt()
    }
    
    fn s_parameters(&self, freq: f64, length: f64) -> SMatrix2x2 {
        let z0 = self.characteristic_impedance(freq);
        let gamma = self.propagation_constant(freq);
        let theta = gamma * length;
        
        // S-parameter calculation for transmission line
        let s11 = Complex::new(0.0, 0.0);  // Matched
        let s21 = (-theta).exp();          // Through
        let s12 = s21;
        let s22 = s11;
        
        SMatrix2x2 { s11, s12, s21, s22 }
    }
}

// ═══════════════════════════════════════════════════════════════
// Eye Diagram Generator
// ═══════════════════════════════════════════════════════════════

struct EyeDiagramGenerator {
    data_rate: f64,
    samples_per_bit: usize,
    bits: usize,
}

impl EyeDiagramGenerator {
    fn generate(&self, channel: &Channel, driver: &IBISModel) -> EyeDiagram {
        let ui = 1.0 / self.data_rate;  // Unit interval
        let dt = ui / self.samples_per_bit as f64;
        
        // Generate PRBS pattern
        let pattern = self.generate_prbs7(self.bits);
        
        // Convolve with channel impulse response
        let impulse = channel.impulse_response(dt, self.bits * self.samples_per_bit);
        let output = self.convolve(&pattern, &impulse);
        
        // Fold into eye diagram
        let mut eye = EyeDiagram::new(self.samples_per_bit * 2);
        for (i, &sample) in output.iter().enumerate() {
            let phase = i % (self.samples_per_bit * 2);
            eye.add_sample(phase, sample);
        }
        
        eye.calculate_metrics();
        eye
    }
    
    fn generate_prbs7(&self, bits: usize) -> Vec<f64> {
        let mut lfsr: u8 = 0x7F;
        let mut pattern = Vec::with_capacity(bits);
        
        for _ in 0..bits {
            let bit = lfsr & 1;
            pattern.push(if bit == 1 { 1.0 } else { -1.0 });
            let new_bit = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
            lfsr = (lfsr >> 1) | (new_bit << 6);
        }
        
        pattern
    }
    
    fn convolve(&self, signal: &[f64], impulse: &[f64]) -> Vec<f64> {
        // FFT-based convolution for efficiency
        // ... implementation
        vec![]
    }
}

// ═══════════════════════════════════════════════════════════════
// PDN Impedance Calculator
// ═══════════════════════════════════════════════════════════════

struct PDNImpedanceCalculator {
    capacitors: Vec<Capacitor>,
    vrm: VRMModel,
    plane: PlaneModel,
}

impl PDNImpedanceCalculator {
    fn impedance_at_frequency(&self, freq: f64) -> Complex<f64> {
        let omega = 2.0 * std::f64::consts::PI * freq;
        
        // VRM impedance (inductive at high frequency)
        let z_vrm = self.vrm.impedance(freq);
        
        // Capacitor impedances in parallel
        let mut y_caps = Complex::new(0.0, 0.0);
        for cap in &self.capacitors {
            y_caps += 1.0 / cap.impedance(freq);
        }
        
        // Plane impedance
        let z_plane = self.plane.impedance(freq);
        
        // Total: VRM || Caps || Plane
        let y_total = 1.0 / z_vrm + y_caps + 1.0 / z_plane;
        1.0 / y_total
    }
    
    fn sweep(&self, f_start: f64, f_stop: f64, points: usize) -> Vec<(f64, f64)> {
        let mut results = Vec::with_capacity(points);
        
        for i in 0..points {
            let log_f = (f_start.log10() + 
                (f_stop.log10() - f_start.log10()) * i as f64 / (points - 1) as f64);
            let freq = 10.0_f64.powf(log_f);
            let z = self.impedance_at_frequency(freq);
            results.push((freq, z.norm()));
        }
        
        results
    }
}

struct Capacitor {
    capacitance: f64,  // F
    esr: f64,          // Ω
    esl: f64,          // H
}

impl Capacitor {
    fn impedance(&self, freq: f64) -> Complex<f64> {
        let omega = 2.0 * std::f64::consts::PI * freq;
        let z_c = Complex::new(0.0, -1.0 / (omega * self.capacitance));
        let z_l = Complex::new(0.0, omega * self.esl);
        let z_r = Complex::new(self.esr, 0.0);
        z_c + z_l + z_r
    }
    
    fn resonant_frequency(&self) -> f64 {
        1.0 / (2.0 * std::f64::consts::PI * (self.esl * self.capacitance).sqrt())
    }
}
```

## API Usage

```rust
// Run SI analysis on high-speed nets
let si_result = pcb.analyze_signal_integrity(SIConfig {
    nets: NetFilter::ByClass("HIGH_SPEED"),
    analysis: vec![
        SIAnalysis::Impedance,
        SIAnalysis::Crosstalk,
        SIAnalysis::EyeDiagram { data_rate: 5e9 },
    ],
})?;

// Check results
for net in si_result.nets {
    if net.impedance_error > 0.10 {
        println!("⚠ {}: {}Ω (target {}Ω)", net.name, net.impedance, net.target);
    }
}

// Run PI analysis
let pi_result = pcb.analyze_power_integrity(PIConfig {
    rails: vec!["VDD_CORE", "VDD_IO"],
    target_impedance: 0.010,  // 10mΩ
    frequency_range: (1e3, 1e9),
})?;

// Auto-optimize decoupling
let optimized = pcb.optimize_decoupling(pi_result, OptimizeConfig {
    budget: 100,              // Max capacitors
    cost_limit: 5.0,          // $5 max
})?;

println!("Optimized: {} capacitors, ${:.2}", optimized.count, optimized.cost);
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `I` | Toggle impedance colormap |
| `Shift+I` | Run SI analysis |
| `Ctrl+I` | SI/PI dashboard |
| `P` | Toggle PDN visualization |
| `Shift+P` | Run PI analysis |
| `E` | Generate eye diagram |

## Related Topics

- [Interactive Routing](../pcb-layout/interactive-routing.md) - Impedance-controlled routing
- [Multi-Layer Support](../pcb-layout/multi-layer.md) - Stackup definition
- [Thermal Simulation](./thermal-simulation.md) - Current-induced heating
- [DRC](../pcb-layout/drc.md) - Design rule checks
- [Calculator Tools](./calculator-tools.md) - Impedance calculators
