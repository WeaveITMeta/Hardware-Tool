# Via & Via Stitching

## Overview

Vias provide electrical connections between PCB layers. Via stitching creates arrays of vias to improve ground plane connectivity, reduce impedance, and provide shielding. Hardware Tool supports all via types and automated stitching patterns.

## Via Types

### Through-Hole Via (PTH)

Standard via connecting all layers:

```rust
Via {
    type_: ViaType::Through,
    position: Point::new(25.0, 30.0),
    diameter: 0.6,      // Pad diameter
    drill: 0.3,         // Hole diameter
    net: "GND",
    
    // Optional
    tented: true,       // Cover with solder mask
    filled: false,      // Fill with epoxy
}
```

### Blind Via

Connects outer layer to inner layer:

```rust
Via {
    type_: ViaType::Blind {
        start: Layer::FCu,
        end: Layer::In1Cu,
    },
    diameter: 0.4,
    drill: 0.2,
}
```

### Buried Via

Connects inner layers only:

```rust
Via {
    type_: ViaType::Buried {
        start: Layer::In1Cu,
        end: Layer::In2Cu,
    },
    diameter: 0.4,
    drill: 0.2,
}
```

### Microvia

Laser-drilled single-layer transition:

```rust
Via {
    type_: ViaType::Micro {
        start: Layer::FCu,
        end: Layer::In1Cu,
    },
    diameter: 0.2,
    drill: 0.1,
}
```

## Via Properties

### Annular Ring

Minimum copper around drill hole:

```
    ┌─────────┐
    │ ┌─────┐ │
    │ │     │ │  ← Annular ring
    │ │  ○  │ │  ← Drill hole
    │ │     │ │
    │ └─────┘ │
    └─────────┘
```

```rust
ViaRules {
    min_annular_ring: 0.1,  // mm
    
    // Calculate pad from drill
    pad_diameter: drill + (2.0 * annular_ring),
}
```

### Via-in-Pad

Via placed directly in SMD pad:

```rust
ViaInPad {
    enabled: true,
    
    // Requirements
    filled: true,           // Must be filled
    capped: true,           // Copper cap
    planarized: true,       // Flat surface
    
    // For BGA
    allow_in_bga: true,
}
```

### Tenting

Cover via with solder mask:

```rust
ViaTenting {
    top: TentMode::Full,      // Completely covered
    bottom: TentMode::Partial, // Partially covered
}

pub enum TentMode {
    None,       // Open (exposed copper)
    Partial,    // Partially covered
    Full,       // Completely covered
}
```

## Via Stitching

### Purpose

- **Reduce ground impedance** between layers
- **Improve thermal dissipation**
- **Provide EMI shielding**
- **Connect split planes**

### Stitching Patterns

#### Grid Pattern

```rust
ViaStitching::grid("GND")
    .region(Rect::new(10.0, 10.0, 90.0, 70.0))
    .spacing(5.0, 5.0)  // x, y spacing in mm
    .via_size(0.6, 0.3) // diameter, drill
    .build();
```

```
● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ●
```

#### Perimeter Pattern

```rust
ViaStitching::perimeter("GND")
    .region(Rect::new(10.0, 10.0, 50.0, 40.0))
    .spacing(3.0)
    .rows(2)  // Double row
    .build();
```

```
● ● ● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ● ● ●
●                   ●
●                   ●
● ● ● ● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ● ● ● ●
```

#### Shield Fence

```rust
ViaStitching::fence("GND")
    .path(&[
        Point::new(10.0, 10.0),
        Point::new(50.0, 10.0),
        Point::new(50.0, 40.0),
    ])
    .spacing(2.0)
    .build();
```

### Automatic Stitching

```rust
AutoStitch {
    net: "GND",
    
    // Placement rules
    min_spacing: 5.0,       // mm between vias
    max_spacing: 10.0,      // mm maximum gap
    
    // Avoid
    clearance_to_signal: 0.5,
    clearance_to_pad: 0.3,
    
    // Via properties
    via_diameter: 0.6,
    via_drill: 0.3,
}
```

## Thermal Vias

### Under Component

```rust
ThermalVias::new("GND")
    .component("U1")           // Under this component
    .pattern(ThermalPattern::Grid)
    .spacing(1.0)
    .via_size(0.4, 0.2)
    .filled(true)              // Filled for thermal
    .build();
```

### Thermal Pad Array

```
┌─────────────────────┐
│  ● ● ● ● ● ● ● ●   │
│  ● ● ● ● ● ● ● ●   │  ← Thermal vias under
│  ● ● ● ● ● ● ● ●   │     exposed pad
│  ● ● ● ● ● ● ● ●   │
└─────────────────────┘
```

### Thermal Calculations

```rust
ThermalViaCalc {
    power_dissipation: 2.0,     // Watts
    max_temperature_rise: 20.0, // °C
    
    // Via thermal resistance
    via_thermal_resistance: 50.0,  // °C/W per via
    
    // Calculate required vias
    required_vias: power / (temp_rise / thermal_resistance),
}
```

## Via Design Rules

### Size Constraints

```rust
ViaRules {
    // Through-hole
    through: ViaSizeRules {
        min_diameter: 0.4,
        min_drill: 0.2,
        min_annular: 0.1,
    },
    
    // Blind/buried
    blind: ViaSizeRules {
        min_diameter: 0.35,
        min_drill: 0.15,
        min_annular: 0.1,
    },
    
    // Microvia
    micro: ViaSizeRules {
        min_diameter: 0.2,
        min_drill: 0.1,
        min_annular: 0.05,
    },
}
```

### Aspect Ratio

```rust
AspectRatio {
    // Board thickness / drill diameter
    through_max: 10.0,    // 1.6mm board / 0.2mm drill = 8:1
    blind_max: 1.0,       // Depth / diameter
    micro_max: 0.75,
}
```

### Spacing Rules

```rust
ViaSpacing {
    via_to_via: 0.3,
    via_to_track: 0.2,
    via_to_pad: 0.2,
    via_to_edge: 0.5,
}
```

## Via Optimization

### Minimize Via Count

```rust
ViaOptimization {
    // Combine nearby vias
    merge_distance: 0.5,
    
    // Remove redundant vias
    remove_redundant: true,
    
    // Prefer layer pairs
    prefer_direct_routes: true,
}
```

### Via Cost Analysis

```
Via Analysis Report
═══════════════════

Total vias: 245
  Through-hole: 180 (73%)
  Blind: 45 (18%)
  Buried: 15 (6%)
  Micro: 5 (2%)

Cost impact:
  Through-hole: Base cost
  Blind/Buried: +$50 setup
  Micro: +$100 setup

Recommendations:
  - 12 vias could be eliminated
  - 8 blind vias could be through-hole
```

## Via Shielding & Fencing

Automatic via fence generation around sensitive nets for EMI reduction and signal isolation.

### Via Fence Configuration

```rust
ViaFence {
    // Target nets
    nets: vec!["RF_OUT", "CLK_100MHz", "USB_D+", "USB_D-"],
    
    // Fence parameters
    fence_net: "GND",           // Net for fence vias
    spacing: 2.0,               // mm between fence vias
    offset: 0.5,                // mm from trace edge
    
    // Rows
    rows: 1,                    // Single or double row
    row_spacing: 1.0,           // mm between rows (if >1)
    
    // Via properties
    via_diameter: 0.6,
    via_drill: 0.3,
    
    // Extent
    fence_entire_net: true,     // Fence full trace length
    fence_regions: vec![],      // Or specific regions only
}
```

### Via Fence Patterns

```
Single Row:              Double Row:
                         
  ●   ●   ●   ●   ●        ●   ●   ●   ●   ●
  ═══════════════════      ═══════════════════
  ●   ●   ●   ●   ●        ●   ●   ●   ●   ●
                           ●   ●   ●   ●   ●
                         
Staggered:               Coaxial (differential):
                         
  ●     ●     ●            ●   ●   ●   ●   ●
    ●     ●     ●          ═══════════════════
  ═══════════════════      ═══════════════════
    ●     ●     ●          ●   ●   ●   ●   ●
  ●     ●     ●          
```

### Automatic Via Shielding

```rust
// Auto-shield sensitive nets
pcb.auto_shield_nets(AutoShieldConfig {
    // Net detection
    detect_high_speed: true,    // Auto-detect by frequency
    frequency_threshold: 100e6, // 100 MHz
    
    // Or explicit net classes
    net_classes: vec!["high_speed", "rf", "clock"],
    
    // Shielding style
    style: ShieldStyle::ViaFence,
    
    // Parameters
    via_spacing: 2.0,
    rows: 1,
    
    // Optimization
    optimize_via_count: true,
    max_spacing_wavelength: 0.1, // λ/10 rule
});
```

### Via Fence UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Via Fence Generator                                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Target Nets:                                                    │
│   [☑] RF_OUT        [☑] CLK_100MHz                             │
│   [☑] USB_D+        [☑] USB_D-                                 │
│                                                                 │
│ Fence Net: [GND          ▼]                                    │
│                                                                 │
│ Via Spacing:    [2.0    ] mm    Offset: [0.5   ] mm            │
│ Rows:           [1      ]       Row Gap: [1.0   ] mm           │
│                                                                 │
│ Pattern: [● Single] [○ Double] [○ Staggered] [○ Coaxial]       │
│                                                                 │
│ Via Size:       [0.6    ] mm    Drill: [0.3   ] mm             │
│                                                                 │
│ [☑] Optimize for λ/10 at [1000   ] MHz                         │
│     Recommended spacing: 1.5 mm                                 │
│                                                                 │
│ Preview: 48 vias will be added                                  │
│                                                                 │
│ [Generate Fence] [Preview] [Cancel]                             │
└─────────────────────────────────────────────────────────────────┘
```

### Coaxial Shielding for Differential Pairs

```rust
CoaxialShield {
    differential_pair: ("USB_D+", "USB_D-"),
    
    // Surround both traces
    shield_net: "GND",
    
    // Via placement
    spacing: 1.5,
    
    // Full enclosure
    top_row: true,
    bottom_row: true,
    side_vias: true,  // At corners/bends
}
```

### EMI Optimization

```rust
EMIOptimizer {
    // Target frequency
    max_frequency: 1e9,  // 1 GHz
    
    // Calculate optimal spacing
    // λ = c / (f × √εr)
    // Spacing ≤ λ/10
    
    dielectric_constant: 4.5,
    
    // Result
    recommended_spacing: 1.5,  // mm for 1 GHz
}
```

## Related Topics

- [Multi-Layer Support](./multi-layer.md)
- [Copper Zones](./copper-zones.md)
- [Interactive Routing](./interactive-routing.md)
