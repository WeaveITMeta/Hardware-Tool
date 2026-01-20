# Copper Zones / Pours

## Overview

Copper zones (also called pours or planes) are filled copper regions used for power distribution, ground planes, and shielding. Hardware Tool provides comprehensive zone management with priority handling, thermal relief, and clearance rules.

## Zone Types

### Ground Planes

```rust
Zone::new("GND")
    .layer(Layer::In1Cu)
    .net("GND")
    .priority(1)
    .fill_mode(FillMode::Solid);
```

### Power Planes

```rust
Zone::new("VCC")
    .layer(Layer::In2Cu)
    .net("VCC")
    .priority(1)
    .fill_mode(FillMode::Solid);
```

### Signal Shielding

```rust
Zone::new("Shield")
    .layer(Layer::FCu)
    .net("GND")
    .region(Rect::new(10.0, 10.0, 50.0, 50.0))
    .priority(2);
```

## Zone Properties

### Basic Configuration

```rust
Zone {
    name: "GND_PLANE",
    net: "GND",
    layer: Layer::In1Cu,
    
    // Outline
    outline: Polygon::from_points(&[
        (0.0, 0.0), (100.0, 0.0),
        (100.0, 80.0), (0.0, 80.0),
    ]),
    
    // Fill settings
    fill_mode: FillMode::Solid,
    min_thickness: 0.2,
    
    // Clearance
    clearance: 0.3,
    
    // Priority (higher = filled first)
    priority: 1,
}
```

### Fill Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| `Solid` | Complete fill | Power/ground planes |
| `Hatched` | Cross-hatch pattern | Flexible PCBs |
| `None` | Outline only | Keep-out definition |

```
Solid:              Hatched:
████████████        ╱╲╱╲╱╲╱╲╱╲
████████████        ╲╱╲╱╲╱╲╱╲╱
████████████        ╱╲╱╲╱╲╱╲╱╲
████████████        ╲╱╲╱╲╱╲╱╲╱
```

### Hatch Settings

```rust
HatchConfig {
    orientation: 45.0,    // degrees
    pitch: 0.5,           // mm
    line_width: 0.2,      // mm
    smoothing_level: 1,
}
```

## Priority Management

### Zone Stacking

```
Priority 3: Signal shield (smallest)
    ┌─────────┐
    │ ▓▓▓▓▓▓▓ │
    │ ▓▓▓▓▓▓▓ │
    └─────────┘

Priority 2: Power island
  ┌───────────────┐
  │ ░░░░░░░░░░░░░ │
  │ ░░░▓▓▓▓▓░░░░░ │
  │ ░░░▓▓▓▓▓░░░░░ │
  │ ░░░░░░░░░░░░░ │
  └───────────────┘

Priority 1: Ground plane (largest)
┌───────────────────────┐
│ ████████████████████ │
│ ███░░░░░░░░░░░░░████ │
│ ███░░░▓▓▓▓▓░░░░░████ │
│ ███░░░▓▓▓▓▓░░░░░████ │
│ ███░░░░░░░░░░░░░████ │
│ ████████████████████ │
└───────────────────────┘
```

### Priority Rules

```rust
// Higher priority zones cut into lower priority
zone_vcc.priority(2);   // Fills first
zone_gnd.priority(1);   // Fills around VCC

// Same priority = no overlap allowed
zone_a.priority(1);
zone_b.priority(1);  // Error if overlapping
```

## Thermal Relief

### Thermal Connections

Thermal relief prevents heat sinking during soldering:

```
Direct:              Thermal Relief:
    │                    │
████████████         ████╳████
████████████         ████ ████
████████████         ████╳████
    │                    │
```

### Thermal Settings

```rust
ThermalRelief {
    enabled: true,
    
    // Spoke configuration
    spoke_count: 4,
    spoke_width: 0.3,      // mm
    spoke_angle: 45.0,     // degrees
    
    // Gap
    gap: 0.25,             // mm
    
    // Minimum connection
    min_connection: 2,     // spokes
}
```

### Pad Connection Modes

```rust
pub enum PadConnection {
    Thermal,        // Thermal relief spokes
    Solid,          // Direct connection
    None,           // No connection (isolated)
}
```

```rust
Zone::new("GND")
    .pad_connection(PadConnection::Thermal)
    .thermal_spokes(4)
    .thermal_gap(0.25);
```

## Clearance Rules

### Zone Clearance

```rust
ZoneClearance {
    // Global clearance
    default: 0.3,
    
    // Per-net overrides
    net_clearances: hashmap! {
        "VCC" => 0.5,
        "HIGH_VOLTAGE" => 1.0,
    },
    
    // Per-class overrides
    class_clearances: hashmap! {
        "power" => 0.4,
        "high_speed" => 0.35,
    },
}
```

### Clearance Types

| Type | Description |
|------|-------------|
| **Track** | Clearance to traces |
| **Pad** | Clearance to pads |
| **Via** | Clearance to vias |
| **Zone** | Clearance to other zones |

## Zone Outline

### Shape Definition

```rust
// Rectangle
Zone::rect(0.0, 0.0, 100.0, 80.0);

// Polygon
Zone::polygon(&[
    (0.0, 0.0), (50.0, 0.0), (50.0, 20.0),
    (100.0, 20.0), (100.0, 80.0), (0.0, 80.0),
]);

// From board outline
Zone::from_board_outline()
    .inset(1.0);  // 1mm from edge
```

### Cutouts

```rust
zone.add_cutout(Cutout::rect(20.0, 20.0, 30.0, 30.0));
zone.add_cutout(Cutout::circle(50.0, 40.0, 5.0));
```

## Zone Fill

### Fill Algorithm

```rust
FillConfig {
    // Island removal
    remove_islands: RemoveIslands::Always,
    min_island_area: 1.0,  // mm²
    
    // Smoothing
    smoothing: Smoothing::Chamfer,
    smoothing_value: 0.2,
    
    // Arc approximation
    arc_segments: 32,
}
```

### Island Handling

```rust
pub enum RemoveIslands {
    Never,          // Keep all islands
    Below(f64),     // Remove if smaller than area
    Always,         // Remove all islands
}
```

### Fill Operations

```rust
// Fill single zone
zone.fill();

// Fill all zones on layer
pcb.fill_zones(Layer::In1Cu);

// Fill all zones
pcb.fill_all_zones();

// Unfill (show outline only)
zone.unfill();
```

## Split Planes

### Power Plane Splits

```rust
// Create split power plane
let vcc_3v3 = Zone::new("VCC_3V3")
    .layer(Layer::In2Cu)
    .net("VCC_3V3")
    .region(Rect::new(0.0, 0.0, 50.0, 80.0));

let vcc_5v = Zone::new("VCC_5V")
    .layer(Layer::In2Cu)
    .net("VCC_5V")
    .region(Rect::new(50.0, 0.0, 100.0, 80.0));
```

### Split Gap

```
┌────────────────┬────────────────┐
│                │                │
│    VCC_3V3     │    VCC_5V      │
│                │                │
│                │                │
└────────────────┴────────────────┘
         ↑
    Split gap (0.5mm typical)
```

## Zone Visualization

### Display Options

```rust
ZoneDisplay {
    show_filled: true,
    show_outline: true,
    show_clearance: false,
    
    // Transparency
    opacity: 0.7,
    
    // Colors
    color_by_net: true,
}
```

### Zone Statistics

```
Zone: GND_PLANE
  Layer: In1.Cu
  Net: GND
  Area: 7,234 mm²
  Fill: 89.2%
  Islands: 0
  Thermal connections: 45
  Solid connections: 12
```

## Related Topics

- [Multi-Layer Support](./multi-layer.md)
- [Via & Via Stitching](./via-stitching.md)
- [Design Rule Check](./drc.md)
