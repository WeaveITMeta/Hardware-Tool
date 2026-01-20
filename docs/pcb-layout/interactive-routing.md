# Interactive Routing

## Overview

Interactive routing is the process of manually creating copper traces to connect component pads according to the netlist. Hardware Tool provides advanced routing features including push-and-shove, differential pairs, and length tuning.

## Routing Modes

### Point-to-Point

Basic click-to-route:

```
Pad A ●────┐
           │
           └────● Pad B
```

### Push-and-Shove

Automatically moves existing traces to make room:

```
Before:                    After:
═══════════════           ═══════════════
                          
    ────────────              ────────────
                    →     ────────────────
═══════════════           ═══════════════
```

### Walkaround

Routes around obstacles without moving them:

```
    ┌─────────┐
    │ Obstacle│
────┘         └────
```

## Routing Controls

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `X` | Start routing |
| `V` | Insert via |
| `/` | Toggle routing mode |
| `+/-` | Change track width |
| `Backspace` | Undo last segment |
| `Esc` | Cancel route |
| `Space` | Switch layer |

### Track Width

```rust
TrackWidth {
    default: 0.2,      // mm
    min: 0.1,
    max: 2.0,
    
    // Quick presets
    presets: vec![0.15, 0.2, 0.3, 0.5, 1.0],
}
```

### Corner Styles

```rust
pub enum CornerStyle {
    Sharp,           // 90° corners
    Mitered45,       // 45° chamfer
    Rounded,         // Arc corners
}
```

```
Sharp:        Mitered:      Rounded:
    │             │             │
    └────     ────┘         ────╯
```

## Via Insertion

### Via Types

```rust
pub enum ViaType {
    Through,         // All layers
    BlindTop,        // Top to inner
    BlindBottom,     // Bottom to inner
    Buried,          // Inner to inner
    Micro,           // Single layer transition
}
```

### Via Properties

```rust
Via {
    type_: ViaType::Through,
    position: Point::new(25.0, 30.0),
    diameter: 0.6,
    drill: 0.3,
    net: "VCC",
    
    // Optional
    tented: true,
    filled: false,
}
```

### Via Placement

```
Layer 1 (F.Cu)  ════●════
                    │
Layer 2 (In1.Cu) ───┼───
                    │
Layer 3 (In2.Cu) ───┼───
                    │
Layer 4 (B.Cu)  ════●════
```

## Differential Pairs

### Configuration

```rust
DifferentialPair {
    positive: "USB_D+",
    negative: "USB_D-",
    
    // Geometry
    trace_width: 0.15,
    trace_gap: 0.15,
    
    // Impedance target
    impedance: 90.0,  // ohms
    tolerance: 10.0,  // percent
    
    // Length matching
    max_skew: 0.1,    // mm
}
```

### Routing Differential Pairs

```
┌─────────────────────────────────────┐
│                                     │
│  ════════════════════════════════   │  ← D+
│  ════════════════════════════════   │  ← D-
│                                     │
└─────────────────────────────────────┘
```

### Skew Tuning

```
Before (skewed):
════════════════════════════════
════════════════════

After (matched):
════════════════════════════════
════╦══╦══╦═════════════════════
    ╚══╝  ╚══╝  (serpentine added)
```

## Length Tuning

### Length Matching Groups

```rust
LengthMatchGroup {
    name: "DDR_DQ",
    nets: vec!["DQ0", "DQ1", "DQ2", "DQ3", "DQ4", "DQ5", "DQ6", "DQ7"],
    target_length: None,  // Match to longest
    tolerance: 0.5,       // mm
}
```

### Serpentine/Meander Patterns

```rust
Meander {
    style: MeanderStyle::Rounded,
    amplitude: 0.5,       // mm
    spacing: 0.3,         // mm
    min_amplitude: 0.2,
    max_amplitude: 1.0,
}
```

```
Rounded:                  Trapezoidal:
    ╭──╮  ╭──╮               ┌──┐  ┌──┐
════╯  ╰══╯  ╰════       ════┘  └──┘  └════
```

### Length Display

```
Net Length Report:
  DQ0: 45.2mm ✓
  DQ1: 45.1mm ✓
  DQ2: 44.8mm ✓ (tuned +0.4mm)
  DQ3: 45.3mm ✓
  CLK: 42.0mm (reference)
```

## Layer Transitions

### Layer Stack Navigation

```rust
// Define preferred layer pairs
LayerPairs {
    signal_layers: vec![
        (Layer::FCu, Layer::In1Cu),
        (Layer::In2Cu, Layer::BCu),
    ],
    
    // Via preferences
    prefer_through: false,
    use_microvias: true,
}
```

### Automatic Layer Selection

```rust
AutoLayer {
    horizontal_layers: vec![Layer::FCu, Layer::In2Cu],
    vertical_layers: vec![Layer::In1Cu, Layer::BCu],
}
```

## Routing Constraints

### Net-Specific Rules

```rust
NetRule::new("CLK")
    .min_width(0.15)
    .max_width(0.15)
    .min_clearance(0.2)
    .max_via_count(2)
    .preferred_layers(&[Layer::FCu]);
```

### Area Rules

```rust
AreaRule::new("high_speed_region")
    .region(Rect::new(20.0, 20.0, 60.0, 60.0))
    .min_clearance(0.25)
    .no_acute_angles(true);
```

## Interactive Features

### Real-Time DRC

- Clearance violations highlighted
- Connectivity errors shown
- Width violations marked

### Ratsnest Display

```rust
RatsnestConfig {
    show_all: false,
    show_selected: true,
    show_net: Some("VCC"),
    
    // Visual options
    color: Color::Yellow,
    style: LineStyle::Dashed,
}
```

### Net Highlighting

```rust
// Highlight specific net
pcb.highlight_net("USB_D+");

// Highlight net class
pcb.highlight_class("power");

// Dim other nets
pcb.dim_unselected(0.3);
```

## Teardrops

### Automatic Teardrops

```rust
TeardropConfig {
    enabled: true,
    
    // Pad connections
    pad_teardrops: true,
    pad_ratio: 0.5,
    
    // Via connections
    via_teardrops: true,
    via_ratio: 0.7,
    
    // Style
    curved: true,
}
```

```
Without:          With teardrop:
    │                 │
────●────         ────◢●◣────
    │                 │
```

## Related Topics

- [Component Placement](./component-placement.md)
- [Auto-Routing](./auto-routing.md)
- [Design Rule Check](./drc.md)
