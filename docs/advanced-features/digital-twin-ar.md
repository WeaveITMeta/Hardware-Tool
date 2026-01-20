# Digital Twin & Augmented Reality Debug

## Overview

Hardware Tool provides a real-time digital twin of your PCB that simulates actual board behavior, combined with augmented reality (AR) capabilities for debugging physical prototypes. See live current flow, voltage distribution, thermal evolution, and EMI patterns—both in simulation and overlaid on real hardware.

### The Vision

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  DIGITAL TWIN                        AR DEBUG                   │
│  ┌─────────────────────┐            ┌─────────────────────┐    │
│  │                     │            │   📱 Phone Camera   │    │
│  │  Live simulation    │            │                     │    │
│  │  of PCB behavior    │◀──────────▶│  Point at real PCB  │    │
│  │                     │    Sync    │  See overlay data   │    │
│  │  • Current flow     │            │                     │    │
│  │  • Voltage levels   │            │  • Net highlighting │    │
│  │  • Temperature      │            │  • Component info   │    │
│  │  • EMI radiation    │            │  • Live measurements│    │
│  │                     │            │                     │    │
│  └─────────────────────┘            └─────────────────────┘    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Digital Twin Simulation

### Real-Time Board Simulation

The digital twin continuously simulates all electrical, thermal, and electromagnetic behavior:

```rust
DigitalTwin {
    // Simulation domains
    domains: SimulationDomains {
        electrical: ElectricalSimulation {
            enabled: true,
            solver: Solver::SPICE,
            update_rate: 1000,        // Hz
        },
        thermal: ThermalSimulation {
            enabled: true,
            solver: Solver::FEM,
            update_rate: 10,          // Hz
        },
        electromagnetic: EMSimulation {
            enabled: true,
            solver: Solver::FDTD,
            update_rate: 1,           // Hz (computationally intensive)
        },
    },
    
    // Input sources
    inputs: InputSources {
        // From schematic netlist
        netlist: true,
        
        // From component models
        spice_models: true,
        ibis_models: true,
        
        // From layout
        parasitics: true,
        thermal_model: true,
    },
    
    // Real-time sync
    sync: SyncConfig {
        // Sync with physical measurements
        measurement_sync: true,
        
        // Sync with test equipment
        equipment_sync: vec!["oscilloscope", "multimeter", "thermal_camera"],
    },
}
```

### Current Flow Visualization

See animated current flowing through traces in real-time:

```rust
CurrentFlowVisualization {
    // Display mode
    mode: CurrentDisplayMode::Animated,
    
    // Animation settings
    animation: AnimationConfig {
        particle_density: 100,        // Particles per amp
        particle_speed: 1.0,          // Relative to actual velocity
        color_by: ColorBy::Magnitude, // or Direction, Net
    },
    
    // Current sources
    sources: CurrentSources {
        from_simulation: true,
        from_measurement: false,      // When connected to equipment
    },
    
    // Thresholds
    thresholds: CurrentThresholds {
        low: 0.001,                   // A (below = dim)
        high: 1.0,                    // A (above = bright)
        warning: 0.8,                 // Fraction of trace rating
        critical: 0.95,
    },
}
```

### Current Flow UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Digital Twin: Current Flow Visualization                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │   VIN ══●═●═●═●═●═●═●═●═●══▶ U1 ══●═●═●═●══▶ VOUT         ││
│  │   12V    ▲ ▲ ▲ ▲ ▲ ▲ ▲ ▲      │                            ││
│  │          │ │ │ │ │ │ │ │      │                            ││
│  │          └─┴─┴─┴─┴─┴─┴─┴──────┼──────────────────          ││
│  │                               │                             ││
│  │                               ▼                             ││
│  │                          ●═●═●═●═●                         ││
│  │                          │ │ │ │ │                         ││
│  │                          ▼ ▼ ▼ ▼ ▼                         ││
│  │                            GND                              ││
│  │                                                             ││
│  │  Legend: ● = current particle (animated)                   ││
│  │          ══ = high current    ── = low current             ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Current Summary:                                                │
│   VIN rail:  2.34A (rated 5A) ✓                                │
│   VOUT rail: 2.31A (rated 3A) ⚠ 77%                           │
│   GND return: 2.34A                                            │
│                                                                 │
│ [Pause] [Speed ▼] [Color by Net] [Show Values] [Settings]       │
└─────────────────────────────────────────────────────────────────┘
```

### Voltage Distribution

Real-time voltage levels across the board:

```rust
VoltageVisualization {
    // Display mode
    mode: VoltageDisplayMode::Heatmap,
    
    // Color mapping
    colormap: Colormap {
        scheme: ColorScheme::Viridis,
        min_voltage: 0.0,
        max_voltage: 12.0,
        auto_scale: true,
    },
    
    // Overlay options
    overlay: VoltageOverlay {
        show_values: true,
        show_net_names: true,
        show_tolerance_bands: true,
    },
    
    // Probing
    probing: ProbingConfig {
        click_to_probe: true,
        show_waveform: true,
        differential_probe: true,
    },
}
```

### Voltage Distribution UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Digital Twin: Voltage Distribution                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │   ████████████████  VIN = 12.0V                            ││
│  │   ████████████████                                          ││
│  │   ████████████████                                          ││
│  │         │                                                   ││
│  │         ▼                                                   ││
│  │   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  VDD_5V = 5.02V                        ││
│  │   ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                                          ││
│  │         │                                                   ││
│  │         ▼                                                   ││
│  │   ░░░░░░░░░░░░░░░░  VDD_3V3 = 3.31V                        ││
│  │   ░░░░░░░░░░░░░░░░                                          ││
│  │         │                                                   ││
│  │         ▼                                                   ││
│  │   ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁  GND = 0.00V                            ││
│  │                                                             ││
│  │   📍 Probe point: U1.VDD = 3.28V (spec: 3.3V ±5%) ✓        ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Voltage Rails:                                                  │
│   VIN:     12.00V ✓    VDD_5V:  5.02V ✓    VDD_3V3: 3.31V ✓   │
│   VDD_1V8:  1.79V ✓    VREF:    2.50V ✓    GND:     0.00V ✓   │
│                                                                 │
│ [Add Probe] [Differential] [Waveform] [Export] [Settings]       │
└─────────────────────────────────────────────────────────────────┘
```

### Thermal Evolution

Watch temperature changes over time:

```rust
ThermalEvolution {
    // Time control
    time_control: TimeControl {
        mode: TimeMode::RealTime,     // or Accelerated, StepByStep
        acceleration: 1.0,            // 1x = real-time
    },
    
    // Visualization
    visualization: ThermalVisualization {
        colormap: Colormap::Thermal,  // Blue → Red
        min_temp: 20.0,               // °C
        max_temp: 100.0,              // °C
        show_isotherms: true,
        show_hotspots: true,
    },
    
    // Recording
    recording: RecordingConfig {
        enabled: true,
        interval: Duration::seconds(1),
        max_duration: Duration::hours(24),
    },
    
    // Alerts
    alerts: ThermalAlerts {
        warning_threshold: 70.0,      // °C
        critical_threshold: 85.0,     // °C
        notify: true,
    },
}
```

### EMI Radiation Patterns

Visualize electromagnetic emissions in 3D:

```rust
EMIVisualization {
    // Field type
    field: FieldType::Both,           // Electric, Magnetic, or Both
    
    // Frequency selection
    frequency: FrequencySelection {
        mode: FrequencyMode::Harmonic,
        fundamental: 100e6,           // 100 MHz clock
        harmonics: vec![1, 2, 3, 5],  // 100, 200, 300, 500 MHz
    },
    
    // 3D visualization
    visualization_3d: Visualization3D {
        show_field_lines: true,
        show_magnitude_surface: true,
        show_antenna_pattern: true,
        animation: true,
    },
    
    // Overlay on PCB
    pcb_overlay: PCBOverlay {
        near_field_height: 5.0,       // mm above board
        show_hotspots: true,
        show_coupling_paths: true,
    },
}
```

## Augmented Reality Debug

### AR System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    AR Debug Architecture                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   Camera    │───▶│   Board     │───▶│   Overlay   │         │
│  │   Input     │    │ Recognition │    │  Renderer   │         │
│  └─────────────┘    └──────┬──────┘    └──────┬──────┘         │
│                            │                  │                 │
│                            ▼                  ▼                 │
│                    ┌─────────────┐    ┌─────────────┐          │
│                    │   Pose      │    │   Data      │          │
│                    │ Estimation  │    │   Fusion    │          │
│                    └──────┬──────┘    └──────┬──────┘          │
│                            │                  │                 │
│                            └────────┬─────────┘                 │
│                                     │                           │
│                                     ▼                           │
│                            ┌─────────────┐                     │
│                            │   Display   │                     │
│                            │   Output    │                     │
│                            └─────────────┘                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Board Recognition

```rust
BoardRecognition {
    // Recognition methods
    methods: RecognitionMethods {
        // Fiducial markers
        fiducials: FiducialRecognition {
            enabled: true,
            marker_type: MarkerType::ArUco,
            marker_size: 5.0,         // mm
        },
        
        // Feature matching
        feature_matching: FeatureMatching {
            enabled: true,
            algorithm: Algorithm::ORB,
            min_matches: 50,
        },
        
        // Silkscreen OCR
        silkscreen_ocr: SilkscreenOCR {
            enabled: true,
            read_ref_des: true,
            read_values: true,
        },
    },
    
    // Pose estimation
    pose: PoseEstimation {
        method: PoseMethod::PnP,
        refinement: true,
        smoothing: 0.8,               // Temporal smoothing
    },
}
```

### AR Overlay Modes

```rust
AROverlayModes {
    // Net highlighting
    net_highlight: NetHighlightMode {
        enabled: true,
        color_by_net_class: true,
        show_net_names: true,
        trace_glow: true,
    },
    
    // Component identification
    component_id: ComponentIDMode {
        enabled: true,
        show_ref_des: true,
        show_value: true,
        show_footprint: true,
        tap_for_details: true,
    },
    
    // Pin identification
    pin_id: PinIDMode {
        enabled: true,
        show_pin_numbers: true,
        show_pin_names: true,
        show_net_connection: true,
    },
    
    // Measurement overlay
    measurement: MeasurementMode {
        enabled: true,
        show_live_voltage: true,
        show_live_current: true,
        show_temperature: true,
    },
    
    // Debug guidance
    debug_guide: DebugGuideMode {
        enabled: true,
        show_test_points: true,
        show_probe_locations: true,
        step_by_step_guide: true,
    },
}
```

### AR Debug UI (Mobile)

```
┌─────────────────────────────────────────────────────────────────┐
│ 📱 AR Debug View                                    [⚙] [📷]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │     ┌─────────────────────────────────────────────────┐    ││
│  │     │  CAMERA VIEW OF PHYSICAL PCB                    │    ││
│  │     │                                                 │    ││
│  │     │   ┌─────┐         ┌─────┐                      │    ││
│  │     │   │ U1  │─────────│ U2  │                      │    ││
│  │     │   │     │  VDD    │     │                      │    ││
│  │     │   └──┬──┘  3.3V   └──┬──┘                      │    ││
│  │     │      │    ═════      │                         │    ││
│  │     │      │    ▲          │                         │    ││
│  │     │      │    │ AR overlay: net highlighted        │    ││
│  │     │      │    │                                    │    ││
│  │     │   ┌──┴──┐ │      ┌─────┐                      │    ││
│  │     │   │ C1  │ │      │ R1  │                      │    ││
│  │     │   │100µF│ │      │ 10k │                      │    ││
│  │     │   └─────┘ │      └─────┘                      │    ││
│  │     │           │                                    │    ││
│  │     │    📍 Tap component for details               │    ││
│  │     │                                                 │    ││
│  │     └─────────────────────────────────────────────────┘    ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Selected: U1 (STM32F407VG)                                     │
│ VDD: 3.28V ✓  |  Temp: 45°C ✓  |  Status: Running             │
│                                                                 │
│ [Highlight Net] [Show Pins] [Measure] [Debug Guide]             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Live Measurement Integration

Connect to test equipment for real-time overlay:

```rust
MeasurementIntegration {
    // Supported equipment
    equipment: EquipmentSupport {
        oscilloscopes: vec![
            Equipment::new("Rigol DS1054Z", Protocol::SCPI),
            Equipment::new("Keysight DSOX1204G", Protocol::SCPI),
            Equipment::new("Tektronix TBS1052C", Protocol::SCPI),
        ],
        multimeters: vec![
            Equipment::new("Fluke 87V", Protocol::Bluetooth),
            Equipment::new("Keysight U1282A", Protocol::USB),
        ],
        thermal_cameras: vec![
            Equipment::new("FLIR One Pro", Protocol::USB),
            Equipment::new("Seek Thermal", Protocol::USB),
        ],
        logic_analyzers: vec![
            Equipment::new("Saleae Logic Pro", Protocol::USB),
            Equipment::new("DSLogic Plus", Protocol::USB),
        ],
    },
    
    // Data fusion
    fusion: DataFusion {
        // Combine simulation with measurement
        mode: FusionMode::MeasurementOverride,
        
        // Calibration
        auto_calibrate: true,
        
        // Discrepancy detection
        discrepancy_threshold: 0.10,  // 10% difference triggers alert
    },
}
```

### Measurement Overlay

```
┌─────────────────────────────────────────────────────────────────┐
│ AR Debug: Live Measurements                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Connected Equipment:                                           │
│  🟢 Rigol DS1054Z (CH1: VDD, CH2: CLK)                         │
│  🟢 FLIR One Pro (thermal overlay)                             │
│  🟢 Fluke 87V (VIN measurement)                                │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │   CAMERA + THERMAL OVERLAY                                  ││
│  │                                                             ││
│  │   ┌─────┐  45°C                    ┌─────┐  38°C           ││
│  │   │ U1  │  ████                    │ U2  │  ▓▓▓            ││
│  │   │     │  ████                    │     │  ▓▓▓            ││
│  │   └──┬──┘                          └──┬──┘                 ││
│  │      │                                │                     ││
│  │      │ VDD = 3.28V (scope)           │                     ││
│  │      │ ════════════════════════════════                    ││
│  │      │                                                      ││
│  │   ┌──┴──┐  52°C                                            ││
│  │   │ Q1  │  ████  ← Hotspot detected!                       ││
│  │   │     │  ████                                             ││
│  │   └─────┘                                                   ││
│  │                                                             ││
│  │   VIN = 12.02V (DMM)                                       ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ ⚠ Alert: Q1 temperature 52°C (simulation predicted 48°C)      │
│   Possible cause: Higher than expected load current            │
│                                                                 │
│ [Investigate] [Adjust Simulation] [Log Data] [Settings]        │
└─────────────────────────────────────────────────────────────────┘
```

## Debug Guidance

### Step-by-Step Debug Wizard

```rust
DebugWizard {
    // Problem categories
    categories: vec![
        DebugCategory::PowerSupply,
        DebugCategory::SignalIntegrity,
        DebugCategory::Communication,
        DebugCategory::Thermal,
        DebugCategory::EMI,
    ],
    
    // Guided procedures
    procedures: vec![
        DebugProcedure {
            name: "Power-on sequence verification",
            steps: vec![
                Step::Measure { point: "VIN", expected: "12V ±5%" },
                Step::Measure { point: "VDD_5V", expected: "5V ±2%" },
                Step::Measure { point: "VDD_3V3", expected: "3.3V ±2%" },
                Step::Measure { point: "VDD_1V8", expected: "1.8V ±2%" },
                Step::Check { condition: "All LEDs lit" },
            ],
        },
        DebugProcedure {
            name: "Communication bus check",
            steps: vec![
                Step::Connect { equipment: "Logic Analyzer", channels: 4 },
                Step::Capture { signal: "SPI_CLK", duration: "1ms" },
                Step::Verify { pattern: "Clock present, 10MHz" },
                Step::Capture { signal: "SPI_MOSI", duration: "1ms" },
                Step::Decode { protocol: "SPI", verify: "Valid frames" },
            ],
        },
    ],
    
    // AR integration
    ar_guidance: ARGuidance {
        show_probe_points: true,
        highlight_current_step: true,
        show_expected_waveforms: true,
    },
}
```

### Debug Wizard UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Debug Wizard: Power Supply Verification                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Step 2 of 5: Measure VDD_5V                                    │
│ ════════════════════════════════════════════════════════════   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   AR VIEW: Probe point highlighted                         │ │
│ │                                                             │ │
│ │              ┌─────────────────────────────┐               │ │
│ │              │                             │               │ │
│ │              │    ┌─────┐                  │               │ │
│ │              │    │ U3  │  LDO             │               │ │
│ │              │    │     │                  │               │ │
│ │              │    └──┬──┘                  │               │ │
│ │              │       │                     │               │ │
│ │              │    ┌──┴──┐                  │               │ │
│ │              │    │ TP2 │ ← PROBE HERE    │               │ │
│ │              │    │     │   📍            │               │ │
│ │              │    └─────┘                  │               │ │
│ │              │                             │               │ │
│ │              └─────────────────────────────┘               │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Instructions:                                                   │
│ 1. Set multimeter to DC voltage                                │
│ 2. Connect black probe to GND (TP_GND)                         │
│ 3. Connect red probe to TP2 (VDD_5V)                           │
│ 4. Read voltage and enter below                                │
│                                                                 │
│ Expected: 5.00V ±2% (4.90V - 5.10V)                           │
│                                                                 │
│ Measured: [5.02    ] V   [Auto-read from DMM]                  │
│                                                                 │
│ Status: ✓ PASS (5.02V within tolerance)                       │
│                                                                 │
│ [◀ Previous] [Next ▶] [Skip] [Abort]                           │
└─────────────────────────────────────────────────────────────────┘
```

## Fault Injection Simulation

### Virtual Fault Testing

```rust
FaultInjection {
    // Fault types
    faults: vec![
        Fault::OpenCircuit { net: "VDD_3V3", location: "R5" },
        Fault::ShortCircuit { net_a: "VDD", net_b: "GND", resistance: 0.1 },
        Fault::ComponentFailure { ref_des: "U1", mode: FailureMode::Open },
        Fault::ParameterDrift { ref_des: "R1", parameter: "resistance", drift: 0.5 },
        Fault::Intermittent { net: "SPI_CLK", probability: 0.01 },
        Fault::ESD { location: "J1", voltage: 8000.0 },
    ],
    
    // Simulation
    simulation: FaultSimulation {
        // Run with fault active
        duration: Duration::seconds(10),
        
        // Monitor effects
        monitor: vec![
            Monitor::Voltage { nets: vec!["VDD_3V3", "VDD_1V8"] },
            Monitor::Current { nets: vec!["VIN"] },
            Monitor::Temperature { components: vec!["U1", "Q1"] },
            Monitor::Functionality { test: "uart_loopback" },
        ],
    },
    
    // Analysis
    analysis: FaultAnalysis {
        // Fault coverage
        coverage_report: true,
        
        // Detectability
        detectability_analysis: true,
        
        // Recommendations
        design_improvements: true,
    },
}
```

### Fault Injection UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Fault Injection Simulation                                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Active Fault: Short circuit VDD_3V3 to GND (0.1Ω)              │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   Digital Twin View (with fault)                           │ │
│ │                                                             │ │
│ │   VIN ══●═●═●═●═●══▶ U3 ══●═●═●══▶ VDD_3V3               │ │
│ │   12V    ▲ ▲ ▲ ▲ ▲      │  ▲ ▲ ▲      │                   │ │
│ │          │ │ │ │ │      │  │ │ │      │                   │ │
│ │          HIGH CURRENT   │  │ │ │      ⚡ SHORT            │ │
│ │          (2.5A!)        │  │ │ │      │                   │ │
│ │                         │  │ │ │      ▼                   │ │
│ │                         │  │ │ │     GND                  │ │
│ │                         │  │ │ │                          │ │
│ │   U3 Temperature: 125°C ████████ CRITICAL!               │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Fault Effects:                                                  │
│   ⚠ VDD_3V3 collapsed to 0.25V (expected 3.3V)                │
│   ⚠ VIN current increased to 2.5A (normal 0.5A)               │
│   ⚠ U3 (LDO) temperature 125°C - thermal shutdown imminent    │
│   ⚠ U1 (MCU) not functioning - brownout detected              │
│                                                                 │
│ Protection Analysis:                                            │
│   ✓ F1 (fuse) would blow in 2.3 seconds                       │
│   ⚠ U3 thermal shutdown at 1.8 seconds (before fuse)          │
│   Recommendation: Add PTC resettable fuse for faster response  │
│                                                                 │
│ [Remove Fault] [Try Another] [Export Report] [Close]            │
└─────────────────────────────────────────────────────────────────┘
```

## Rust Implementation

### Digital Twin Core

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════
// Digital Twin Engine
// ═══════════════════════════════════════════════════════════════

struct DigitalTwin {
    // Board model
    board: Arc<RwLock<BoardModel>>,
    
    // Simulation engines
    electrical: ElectricalSimulator,
    thermal: ThermalSimulator,
    electromagnetic: EMSimulator,
    
    // State
    state: SimulationState,
    
    // Visualization
    visualizer: Visualizer,
}

impl DigitalTwin {
    async fn run(&mut self) {
        loop {
            // Update electrical simulation
            let electrical_state = self.electrical.step().await;
            
            // Update thermal simulation (uses electrical results)
            let thermal_state = self.thermal.step(&electrical_state).await;
            
            // Update EM simulation (periodic, computationally intensive)
            if self.state.frame % 100 == 0 {
                let em_state = self.electromagnetic.step(&electrical_state).await;
                self.state.em = em_state;
            }
            
            // Update state
            self.state.electrical = electrical_state;
            self.state.thermal = thermal_state;
            
            // Update visualization
            self.visualizer.update(&self.state).await;
            
            self.state.frame += 1;
            
            // Rate limiting
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
    
    fn inject_fault(&mut self, fault: Fault) {
        match fault {
            Fault::OpenCircuit { net, location } => {
                self.electrical.open_circuit(&net, &location);
            }
            Fault::ShortCircuit { net_a, net_b, resistance } => {
                self.electrical.short_circuit(&net_a, &net_b, resistance);
            }
            Fault::ComponentFailure { ref_des, mode } => {
                self.electrical.fail_component(&ref_des, mode);
            }
            _ => {}
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// AR System
// ═══════════════════════════════════════════════════════════════

struct ARSystem {
    // Camera
    camera: CameraCapture,
    
    // Board recognition
    recognizer: BoardRecognizer,
    
    // Overlay renderer
    renderer: OverlayRenderer,
    
    // Digital twin reference
    twin: Arc<RwLock<DigitalTwin>>,
    
    // Equipment connections
    equipment: Vec<Box<dyn MeasurementEquipment>>,
}

impl ARSystem {
    async fn process_frame(&mut self, frame: &CameraFrame) -> ARFrame {
        // Recognize board in frame
        let recognition = self.recognizer.recognize(frame).await;
        
        if let Some(board_pose) = recognition.pose {
            // Get current simulation state
            let twin_state = self.twin.read().await.state.clone();
            
            // Get live measurements
            let measurements = self.collect_measurements().await;
            
            // Fuse simulation and measurements
            let fused_data = self.fuse_data(&twin_state, &measurements);
            
            // Render overlay
            let overlay = self.renderer.render(
                frame,
                &board_pose,
                &fused_data,
                &self.overlay_config,
            );
            
            ARFrame {
                base: frame.clone(),
                overlay,
                recognition,
                data: fused_data,
            }
        } else {
            ARFrame::no_recognition(frame)
        }
    }
    
    async fn collect_measurements(&self) -> Measurements {
        let mut measurements = Measurements::new();
        
        for equipment in &self.equipment {
            if let Ok(data) = equipment.read().await {
                measurements.add(data);
            }
        }
        
        measurements
    }
}

// ═══════════════════════════════════════════════════════════════
// Board Recognition
// ═══════════════════════════════════════════════════════════════

struct BoardRecognizer {
    // Feature database
    features: FeatureDatabase,
    
    // Fiducial detector
    fiducial_detector: FiducialDetector,
    
    // OCR engine
    ocr: OCREngine,
}

impl BoardRecognizer {
    async fn recognize(&self, frame: &CameraFrame) -> RecognitionResult {
        // Try fiducial markers first (fastest)
        if let Some(pose) = self.fiducial_detector.detect(frame) {
            return RecognitionResult {
                pose: Some(pose),
                method: RecognitionMethod::Fiducial,
                confidence: 0.99,
            };
        }
        
        // Try feature matching
        if let Some((pose, confidence)) = self.features.match_frame(frame) {
            if confidence > 0.8 {
                return RecognitionResult {
                    pose: Some(pose),
                    method: RecognitionMethod::FeatureMatching,
                    confidence,
                };
            }
        }
        
        // Try OCR on silkscreen
        if let Some(ref_des) = self.ocr.find_reference_designators(frame) {
            if let Some(pose) = self.estimate_pose_from_refdes(&ref_des, frame) {
                return RecognitionResult {
                    pose: Some(pose),
                    method: RecognitionMethod::OCR,
                    confidence: 0.7,
                };
            }
        }
        
        RecognitionResult::not_found()
    }
}
```

## API Usage

```rust
// Create digital twin
let twin = DigitalTwin::new(&pcb)?;

// Start real-time simulation
twin.start();

// Visualize current flow
twin.visualize(Visualization::CurrentFlow {
    animated: true,
    color_by: ColorBy::Magnitude,
})?;

// Inject fault for testing
twin.inject_fault(Fault::ShortCircuit {
    net_a: "VDD".to_string(),
    net_b: "GND".to_string(),
    resistance: 0.1,
});

// Observe effects
let effects = twin.observe_fault_effects(Duration::seconds(5))?;
println!("Fault effects: {:?}", effects);

// Start AR debug session
let ar = ARDebug::new(&pcb, &twin)?;
ar.connect_equipment(Equipment::Oscilloscope("Rigol DS1054Z"))?;
ar.start_camera()?;

// Get AR frame with overlay
let frame = ar.capture_frame()?;
frame.save("ar_debug.png")?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `D` | Toggle digital twin view |
| `Shift+D` | Start/stop simulation |
| `Ctrl+D` | Digital twin settings |
| `A` | Toggle AR mode |
| `F` | Inject fault |
| `Ctrl+F` | Fault injection menu |

## Related Topics

- [Thermal Simulation](./thermal-simulation.md) - Thermal analysis
- [Signal Integrity](./signal-power-integrity.md) - SI/PI analysis
- [EMC Simulation](./electromagnetic-simulation.md) - EMC/EMI analysis
- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md) - 3D visualization
- [Real-Time Preview](./realtime-preview.md) - Live preview
