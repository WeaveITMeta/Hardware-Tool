# Calculator Tools

## Overview

Hardware Tool includes built-in calculators for common PCB design calculations including track width, via current capacity, impedance, and thermal relief. These tools help ensure designs meet electrical and thermal requirements.

## Track Width Calculator

### Current Capacity

Calculate minimum track width for given current:

```rust
let width = TrackWidthCalculator::for_current(
    current: 2.0,           // Amps
    temperature_rise: 10.0, // °C
    copper_weight: 1.0,     // oz (35µm)
    layer: LayerType::External,
)?;

println!("Minimum width: {:.2}mm", width);
```

### IPC-2221 Formula

```
Width (mils) = (I / (k × ΔT^b))^(1/c) / (thickness × 1.378)

Where:
  I = Current (Amps)
  k = 0.048 (external) or 0.024 (internal)
  b = 0.44
  c = 0.725
  ΔT = Temperature rise (°C)
  thickness = Copper thickness (oz)
```

### Calculator UI

```
┌─────────────────────────────────────────┐
│        Track Width Calculator           │
├─────────────────────────────────────────┤
│                                         │
│  Current:        [2.0    ] A            │
│  Temp Rise:      [10     ] °C           │
│  Copper Weight:  [1 oz   ] ▼            │
│  Layer Type:     [External] ▼           │
│  Ambient Temp:   [25     ] °C           │
│                                         │
│  ─────────────────────────────────────  │
│                                         │
│  Minimum Width:  0.76 mm (30 mil)       │
│  Recommended:    1.00 mm (40 mil)       │
│                                         │
│  [Apply to Net Class] [Copy]            │
│                                         │
└─────────────────────────────────────────┘
```

## Via Current Calculator

### Via Current Capacity

```rust
let current = ViaCurrentCalculator::capacity(
    drill_diameter: 0.3,    // mm
    plating_thickness: 25.0, // µm
    temperature_rise: 10.0,  // °C
)?;

println!("Via current capacity: {:.2}A", current);
```

### Multiple Vias

```rust
let vias_needed = ViaCurrentCalculator::vias_for_current(
    current: 5.0,           // Amps
    drill_diameter: 0.3,    // mm
    plating_thickness: 25.0, // µm
    temperature_rise: 10.0,  // °C
)?;

println!("Vias needed: {}", vias_needed);
```

### Via Calculator UI

```
┌─────────────────────────────────────────┐
│         Via Current Calculator          │
├─────────────────────────────────────────┤
│                                         │
│  Drill Diameter:    [0.3   ] mm         │
│  Plating Thickness: [25    ] µm         │
│  Temperature Rise:  [10    ] °C         │
│                                         │
│  ─────────────────────────────────────  │
│                                         │
│  Single Via Capacity: 0.8 A             │
│                                         │
│  For 5.0 A current:                     │
│    Vias needed: 7                       │
│    Recommended: 8 (with margin)         │
│                                         │
└─────────────────────────────────────────┘
```

## Impedance Calculator

### Microstrip Impedance

```rust
let impedance = ImpedanceCalculator::microstrip(
    trace_width: 0.15,       // mm
    dielectric_height: 0.2,  // mm
    dielectric_constant: 4.5,
    copper_thickness: 0.035, // mm
)?;

println!("Impedance: {:.1}Ω", impedance);
```

### Stripline Impedance

```rust
let impedance = ImpedanceCalculator::stripline(
    trace_width: 0.12,       // mm
    dielectric_height: 0.2,  // mm (each side)
    dielectric_constant: 4.5,
    copper_thickness: 0.035, // mm
)?;
```

### Differential Pair Impedance

```rust
let impedance = ImpedanceCalculator::differential_microstrip(
    trace_width: 0.1,        // mm
    trace_gap: 0.15,         // mm
    dielectric_height: 0.2,  // mm
    dielectric_constant: 4.5,
    copper_thickness: 0.035, // mm
)?;

println!("Differential impedance: {:.1}Ω", impedance);
```

### Impedance Calculator UI

```
┌─────────────────────────────────────────┐
│        Impedance Calculator             │
├─────────────────────────────────────────┤
│                                         │
│  Type: [Microstrip        ] ▼           │
│                                         │
│  Trace Width:      [0.15  ] mm          │
│  Dielectric Height:[0.2   ] mm          │
│  Dielectric Er:    [4.5   ]             │
│  Copper Thickness: [35    ] µm          │
│                                         │
│  ─────────────────────────────────────  │
│                                         │
│  Calculated Impedance: 52.3 Ω           │
│                                         │
│  For 50Ω target:                        │
│    Required width: 0.17 mm              │
│                                         │
│  [Apply to Net Class] [Copy]            │
│                                         │
└─────────────────────────────────────────┘
```

### Solve for Width

```rust
let width = ImpedanceCalculator::solve_microstrip_width(
    target_impedance: 50.0,  // Ω
    dielectric_height: 0.2,  // mm
    dielectric_constant: 4.5,
    copper_thickness: 0.035, // mm
)?;

println!("Required width for 50Ω: {:.3}mm", width);
```

## Thermal Relief Calculator

### Thermal Via Array

```rust
let thermal = ThermalCalculator::via_array(
    power_dissipation: 2.0,  // Watts
    via_diameter: 0.3,       // mm
    via_plating: 25.0,       // µm
    board_thickness: 1.6,    // mm
)?;

println!("Thermal resistance: {:.1}°C/W", thermal.resistance);
println!("Vias needed: {}", thermal.vias_recommended);
```

### Copper Pour Thermal

```rust
let thermal = ThermalCalculator::copper_pour(
    power_dissipation: 1.0,  // Watts
    copper_area: 100.0,      // mm²
    copper_thickness: 35.0,  // µm
    ambient_temp: 25.0,      // °C
)?;

println!("Temperature rise: {:.1}°C", thermal.temp_rise);
```

## Wavelength Calculator

### Signal Wavelength

```rust
let wavelength = WavelengthCalculator::in_pcb(
    frequency: 1e9,          // Hz (1 GHz)
    dielectric_constant: 4.5,
)?;

println!("Wavelength: {:.1}mm", wavelength);
println!("λ/10 rule: {:.1}mm max trace", wavelength / 10.0);
```

### Propagation Delay

```rust
let delay = WavelengthCalculator::propagation_delay(
    trace_length: 50.0,      // mm
    dielectric_constant: 4.5,
)?;

println!("Propagation delay: {:.2}ns", delay * 1e9);
```

## Resistance Calculator

### Trace Resistance

```rust
let resistance = ResistanceCalculator::trace(
    length: 100.0,           // mm
    width: 0.2,              // mm
    copper_thickness: 35.0,  // µm
    temperature: 25.0,       // °C
)?;

println!("Trace resistance: {:.3}Ω", resistance);
```

### Via Resistance

```rust
let resistance = ResistanceCalculator::via(
    drill_diameter: 0.3,     // mm
    plating_thickness: 25.0, // µm
    board_thickness: 1.6,    // mm
)?;

println!("Via resistance: {:.3}mΩ", resistance * 1000.0);
```

## Decoupling Capacitor Calculator

### Resonant Frequency

```rust
let resonance = DecouplingCalculator::resonant_frequency(
    capacitance: 100e-9,     // F (100nF)
    esl: 1e-9,               // H (1nH typical)
)?;

println!("Resonant frequency: {:.1}MHz", resonance / 1e6);
```

### Capacitor Selection

```rust
let caps = DecouplingCalculator::select_capacitors(
    target_impedance: 0.1,   // Ω
    frequency_range: (1e6, 100e6),  // 1MHz to 100MHz
)?;

for cap in caps {
    println!("{}: {} at {:.1}MHz", 
             cap.value, cap.package, cap.effective_freq / 1e6);
}
```

## Calculator Integration

### Apply to Design

```rust
// Apply calculated width to net class
let width = TrackWidthCalculator::for_current(2.0, 10.0, 1.0, External)?;
pcb.net_class("power").set_track_width(width);

// Apply impedance-matched width
let width = ImpedanceCalculator::solve_microstrip_width(50.0, 0.2, 4.5, 0.035)?;
pcb.net_class("high_speed").set_track_width(width);
```

### Calculator Panel

Access all calculators from unified panel:

```
┌─────────────────────────────────────────┐
│           Design Calculators            │
├─────────────────────────────────────────┤
│                                         │
│  ► Track Width                          │
│  ► Via Current                          │
│  ► Impedance                            │
│  ► Thermal Relief                       │
│  ► Wavelength / Delay                   │
│  ► Resistance                           │
│  ► Decoupling                           │
│                                         │
│  Recent Calculations:                   │
│  • 50Ω microstrip: 0.17mm              │
│  • 2A track: 0.76mm                    │
│  • 5A vias: 7 needed                   │
│                                         │
└─────────────────────────────────────────┘
```

## Impedance Profile Editor

Define target impedance curves per layer and get automatic trace width suggestions.

### Impedance Profile Configuration

```rust
ImpedanceProfile {
    name: "DDR4_Impedance",
    
    // Layer-specific targets
    layers: vec![
        LayerImpedance {
            layer: Layer::FCu,
            single_ended: 50.0,      // Ω
            differential: 100.0,      // Ω
        },
        LayerImpedance {
            layer: Layer::In1Cu,
            single_ended: 50.0,
            differential: 85.0,       // Stripline
        },
    ],
    
    // Stackup reference
    stackup: "4-layer-1.6mm",
    
    // Tolerance
    tolerance_percent: 10.0,
}
```

### Profile Editor UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Impedance Profile Editor                                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Profile: [DDR4_Impedance    ▼]  Stackup: [4-layer-1.6mm ▼]     │
│                                                                 │
│ Layer Impedance Targets:                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Layer    │ Type       │ Target │ Width  │ Gap    │ Status  │ │
│ ├──────────┼────────────┼────────┼────────┼────────┼─────────┤ │
│ │ F.Cu     │ Single     │ 50Ω    │ 0.17mm │ -      │ ✓       │ │
│ │ F.Cu     │ Diff Pair  │ 100Ω   │ 0.12mm │ 0.15mm │ ✓       │ │
│ │ In1.Cu   │ Single     │ 50Ω    │ 0.14mm │ -      │ ✓       │ │
│ │ In1.Cu   │ Diff Pair  │ 85Ω    │ 0.10mm │ 0.12mm │ ✓       │ │
│ │ B.Cu     │ Single     │ 50Ω    │ 0.17mm │ -      │ ✓       │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Stackup Parameters:                                             │
│   Dielectric: FR4 (εr = 4.5)                                   │
│   Copper: 1oz (35µm)                                           │
│   Prepreg: 0.2mm                                               │
│                                                                 │
│ [Apply to Net Classes] [Export] [Import from Fab]               │
└─────────────────────────────────────────────────────────────────┘
```

### Apply Profile to Nets

```rust
// Apply impedance profile to net classes
pcb.apply_impedance_profile("DDR4_Impedance", &[
    "high_speed",
    "ddr_data",
    "ddr_clk",
]);

// Auto-calculate widths for all controlled impedance nets
pcb.calculate_impedance_widths()?;
```

## Live Thermal Simulation Overlay

Real-time temperature gradient visualization during placement and routing.

### Thermal Overlay Configuration

```rust
ThermalOverlay {
    enabled: true,
    
    // Heat sources
    sources: vec![
        HeatSource::Component("U1", 2.5),   // 2.5W
        HeatSource::Component("Q1", 1.0),   // 1.0W
        HeatSource::Trace("VCC", 0.5),      // 0.5W I²R
    ],
    
    // Visualization
    display: ThermalDisplay {
        color_map: ColorMap::Thermal,  // Blue→Green→Yellow→Red
        min_temp: 25.0,                // °C (ambient)
        max_temp: 85.0,                // °C (max)
        opacity: 0.6,
        show_isotherms: true,
        isotherm_interval: 10.0,       // °C
    },
    
    // Simulation
    simulation: ThermalSimulation {
        ambient_temp: 25.0,
        airflow: Airflow::Natural,
        include_copper_spreading: true,
        include_vias: true,
    },
}
```

### Thermal Overlay UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Thermal Simulation Overlay                          [Toggle: T] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │   ░░░░░▒▒▒▒▓▓▓▓████▓▓▓▓▒▒▒▒░░░░░                       │   │
│  │   ░░░▒▒▒▒▓▓▓▓████████▓▓▓▓▒▒▒░░░                       │   │
│  │   ░░▒▒▒▓▓▓▓████[U1]████▓▓▓▓▒▒▒░░                       │   │
│  │   ░░░▒▒▒▒▓▓▓▓████████▓▓▓▓▒▒▒░░░                       │   │
│  │   ░░░░░▒▒▒▒▓▓▓▓████▓▓▓▓▒▒▒▒░░░░░                       │   │
│  │                                                         │   │
│  │   Legend: ░ 25-40°C  ▒ 40-55°C  ▓ 55-70°C  █ 70-85°C   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ Hot spots:                                                      │
│   U1: 78°C (MCU, 2.5W)                                         │
│   Q1: 65°C (MOSFET, 1.0W)                                      │
│                                                                 │
│ Recommendations:                                                │
│   ⚠ U1 approaching thermal limit - add thermal vias           │
│   ✓ Q1 within safe range                                       │
│                                                                 │
│ [Add Thermal Vias] [Adjust Placement] [Export Report]           │
└─────────────────────────────────────────────────────────────────┘
```

## Via Current & Voltage Drop Heatmap

Real-time visualization of current capacity and voltage drop on power planes.

### Power Plane Analysis

```rust
PowerPlaneAnalysis {
    // Target nets
    nets: vec!["VCC", "VCC_3V3", "VCC_5V"],
    
    // Current sources/sinks
    currents: vec![
        CurrentSource::Pin("J1", "VIN", 5.0),    // 5A input
        CurrentSink::Pin("U1", "VCC", 0.5),      // 500mA
        CurrentSink::Pin("U2", "VCC", 1.2),      // 1.2A
    ],
    
    // Visualization
    display: PowerDisplay {
        show_current_density: true,
        show_voltage_drop: true,
        show_via_utilization: true,
        
        // Thresholds
        max_current_density: 30.0,  // A/mm²
        max_voltage_drop: 0.1,      // V
        max_via_current: 1.0,       // A per via
    },
}
```

### Heatmap Display

```
┌─────────────────────────────────────────────────────────────────┐
│ Power Plane Analysis: VCC                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ View: [● Current Density] [○ Voltage Drop] [○ Via Utilization] │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  VIN ════════════════════════════════════════════════   │   │
│  │   ║                                                      │   │
│  │   ║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │   │
│  │   ╠══●══════════════════════════════════════════════   │   │
│  │   ║  ░░░░░░░▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │   │
│  │   ║  ░░░░░▒▒▒▓▓▓▓▓▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │   │
│  │   ╠══════●[U1]●════════════════════════════●[U2]●═══   │   │
│  │   ║  ░░░░░▒▒▒▓▓▓▓▓▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │   │
│  │   ║  ░░░░░░░▒▒▒▒▒▒▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   │   │
│  │                                                         │   │
│  │   Legend: ░ <10A/mm²  ▒ 10-20A/mm²  ▓ 20-30A/mm²       │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ Via Analysis:                                                   │
│   Total vias on VCC: 24                                         │
│   Max via current: 0.8A (80% capacity)                         │
│   Voltage drop: 45mV (VIN to U2)                               │
│                                                                 │
│ [Add Vias] [Widen Traces] [Export Report]                       │
└─────────────────────────────────────────────────────────────────┘
```

## Auto Test Point Placement

One-click test point addition on selected nets with customizable styles.

### Test Point Configuration

```rust
TestPointConfig {
    // Style
    style: TestPointStyle::Pad,  // Pad, Via, LoopPad
    
    // Size
    pad_diameter: 1.5,           // mm
    drill_diameter: 0.8,         // mm (if via)
    
    // Placement
    placement: TestPointPlacement {
        prefer_edge: true,
        min_spacing: 2.54,       // mm (0.1" grid)
        avoid_components: true,
        clearance: 1.0,
    },
    
    // Naming
    naming: TestPointNaming {
        prefix: "TP",
        start_number: 1,
        include_net_name: true,
    },
}
```

### Test Point Generator UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Add Test Points                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Select Nets:                                                    │
│   [☑] VCC          [☑] GND          [☑] CLK                   │
│   [☑] SDA          [☑] SCL          [☐] USB_D+                │
│   [☑] RESET        [☐] USB_D-       [☑] TX                    │
│                                                                 │
│ Style: [● Pad] [○ Via] [○ Loop Pad]                            │
│                                                                 │
│ Pad Size:    [1.5   ] mm    Drill: [0.8   ] mm                 │
│                                                                 │
│ Placement:                                                      │
│   [☑] Prefer board edge                                        │
│   [☑] Align to 2.54mm grid                                     │
│   [☑] Avoid component courtyards                               │
│                                                                 │
│ Preview: 8 test points will be added                            │
│                                                                 │
│ [Generate] [Preview] [Cancel]                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Test Point API

```rust
// Add test points to selected nets
pcb.add_test_points(&["VCC", "GND", "CLK", "SDA", "SCL"], TestPointConfig::default())?;

// Add test point at specific location
pcb.add_test_point("TP1", "VCC", Point::new(10.0, 20.0))?;

// Auto-place test points for all power nets
pcb.add_test_points_for_net_class("power", TestPointConfig {
    style: TestPointStyle::Pad,
    placement: TestPointPlacement { prefer_edge: true, ..Default::default() },
    ..Default::default()
})?;
```

## Net-Class Color Themes

Assign visual themes per net class for easy identification.

### Color Theme Configuration

```rust
NetClassColors {
    themes: vec![
        NetClassTheme {
            net_class: "power",
            color: Color::rgb(255, 100, 100),  // Red
            highlight_color: Color::rgb(255, 150, 150),
            trace_style: TraceStyle::Bold,
        },
        NetClassTheme {
            net_class: "ground",
            color: Color::rgb(100, 100, 255),  // Blue
            highlight_color: Color::rgb(150, 150, 255),
            trace_style: TraceStyle::Bold,
        },
        NetClassTheme {
            net_class: "high_speed",
            color: Color::rgb(255, 200, 100),  // Orange
            highlight_color: Color::rgb(255, 220, 150),
            trace_style: TraceStyle::Normal,
        },
        NetClassTheme {
            net_class: "analog",
            color: Color::rgb(100, 255, 100),  // Green
            highlight_color: Color::rgb(150, 255, 150),
            trace_style: TraceStyle::Normal,
        },
    ],
    
    // Default for unassigned
    default_color: Color::rgb(200, 200, 200),
}
```

### Color Theme UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Net Class Color Themes                                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Net Class      │ Color    │ Style  │ Nets │ Preview            │
│ ───────────────┼──────────┼────────┼──────┼─────────────────── │
│ power          │ [█ Red ] │ Bold   │ 12   │ ════════════       │
│ ground         │ [█ Blue] │ Bold   │ 8    │ ════════════       │
│ high_speed     │ [█ Orng] │ Normal │ 24   │ ────────────       │
│ analog         │ [█ Grn ] │ Normal │ 6    │ ────────────       │
│ default        │ [█ Gray] │ Normal │ 45   │ ────────────       │
│                                                                 │
│ [☑] Apply colors to ratsnest                                   │
│ [☑] Apply colors to routed traces                              │
│ [☑] Apply colors to zones                                      │
│                                                                 │
│ [Apply] [Reset to Default] [Import Theme] [Export Theme]        │
└─────────────────────────────────────────────────────────────────┘
```

## Interactive Solder Mask Tool

Adjust solder mask expansion/shrink with live visual preview.

### Solder Mask Configuration

```rust
SolderMaskConfig {
    // Global expansion
    expansion: 0.05,             // mm (positive = larger opening)
    
    // Per-pad overrides
    pad_overrides: vec![
        PadMaskOverride {
            footprint: "BGA*",
            expansion: 0.0,      // No expansion for BGA
        },
        PadMaskOverride {
            footprint: "QFN*",
            expansion: 0.03,     // Smaller for QFN
        },
    ],
    
    // Via tenting
    via_tenting: ViaTenting {
        tent_vias: true,
        min_via_diameter: 0.4,   // Tent vias smaller than this
    },
}
```

### Solder Mask Editor UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Solder Mask Editor                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Global Expansion: [0.05  ] mm                                   │
│                                                                 │
│ Live Preview:                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │     ┌───────┐         ┌───────┐                        │   │
│  │     │ ┌───┐ │         │ ┌───┐ │                        │   │
│  │     │ │PAD│ │  ──→    │ │PAD│ │                        │   │
│  │     │ └───┘ │         │ └───┘ │                        │   │
│  │     └───────┘         └───────┘                        │   │
│  │      0.00mm            0.05mm                          │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ Via Tenting:                                                    │
│   [☑] Tent vias smaller than [0.4   ] mm                       │
│   [☐] Tent all vias                                            │
│                                                                 │
│ [Apply] [Preview DFM] [Cancel]                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Related Topics

- [Interactive Routing](../pcb-layout/interactive-routing.md)
- [Multi-Layer Support](../pcb-layout/multi-layer.md)
- [Via Stitching](../pcb-layout/via-stitching.md)
