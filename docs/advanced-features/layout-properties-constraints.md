# Layout Properties & Constraints

## Overview

Layout properties and constraints control component placement, routing behavior, and design rules at various scopes. Hardware Tool supports manual nudges, auto-layout helpers, keep-out zones, and hierarchical constraint management.

## Constraint Hierarchy

```
┌─────────────────────────────────────────┐
│           Global Defaults               │
├─────────────────────────────────────────┤
│           Net Class Rules               │
├─────────────────────────────────────────┤
│           Net-Specific Rules            │
├─────────────────────────────────────────┤
│           Area/Region Rules             │
├─────────────────────────────────────────┤
│           Component Rules               │
└─────────────────────────────────────────┘
        ▲
        │ Higher priority overrides lower
```

## Global Properties

### Board Properties

```rust
BoardProperties {
    // Dimensions
    width: 100.0,
    height: 80.0,
    thickness: 1.6,
    
    // Layers
    layer_count: 4,
    
    // Units
    units: Units::Millimeters,
    
    // Grid
    grid: GridSettings {
        x: 0.1,
        y: 0.1,
        origin: Point::new(0.0, 0.0),
    },
}
```

### Default Design Rules

```rust
DefaultRules {
    // Clearances
    clearance: 0.15,
    
    // Track
    track_width: 0.2,
    
    // Via
    via_diameter: 0.6,
    via_drill: 0.3,
    
    // Differential pair
    diff_pair_width: 0.1,
    diff_pair_gap: 0.15,
}
```

## Net Class Constraints

### Defining Net Classes

```rust
NetClass::new("power")
    .description("Power distribution nets")
    
    // Track properties
    .track_width(0.5)
    .track_width_range(0.3, 1.0)
    
    // Clearance
    .clearance(0.3)
    
    // Via properties
    .via_diameter(0.8)
    .via_drill(0.4)
    
    // Differential pair (if applicable)
    .diff_pair_width(0.15)
    .diff_pair_gap(0.15)
    
    // Assign nets
    .add_nets(&["VCC", "VDD", "VBAT", "+5V", "+3V3"]);
```

### Net Class Table

| Class | Width | Clearance | Via Dia | Via Drill |
|-------|-------|-----------|---------|-----------|
| default | 0.2mm | 0.15mm | 0.6mm | 0.3mm |
| power | 0.5mm | 0.3mm | 0.8mm | 0.4mm |
| high_speed | 0.15mm | 0.2mm | 0.5mm | 0.25mm |
| analog | 0.25mm | 0.25mm | 0.6mm | 0.3mm |

## Net-Specific Constraints

### Individual Net Rules

```rust
NetRule::new("CLK_100MHz")
    // Override net class
    .track_width(0.12)
    .clearance(0.25)
    
    // Length constraints
    .max_length(50.0)
    .min_length(30.0)
    
    // Via limits
    .max_vias(2)
    
    // Layer restrictions
    .allowed_layers(&[Layer::FCu, Layer::In1Cu])
    
    // Routing style
    .corner_style(CornerStyle::Mitered45);
```

### Differential Pair Rules

```rust
DiffPairRule::new("USB_D+", "USB_D-")
    // Impedance
    .impedance(90.0)
    .impedance_tolerance(10.0)
    
    // Geometry
    .trace_width(0.1)
    .trace_gap(0.15)
    
    // Matching
    .max_skew(0.1)
    .max_uncoupled_length(5.0);
```

## Area/Region Constraints

### Keep-Out Zones

```rust
KeepOut::new("mounting_area")
    .shape(Rect::new(5.0, 5.0, 15.0, 15.0))
    
    // What to exclude
    .exclude_tracks(true)
    .exclude_vias(true)
    .exclude_pours(true)
    .exclude_components(true)
    
    // Layer scope
    .layers(&[Layer::FCu, Layer::BCu]);
```

### Routing Regions

```rust
RoutingRegion::new("bga_fanout")
    .shape(Rect::new(20.0, 20.0, 60.0, 60.0))
    
    // Override rules in region
    .track_width(0.1)
    .clearance(0.1)
    .via_diameter(0.4)
    .via_drill(0.2);
```

### Placement Regions

```rust
PlacementRegion::new("power_section")
    .shape(Rect::new(0.0, 0.0, 30.0, 80.0))
    
    // Component restrictions
    .allowed_components(&["U1", "L1", "C1", "C2", "D1"])
    .required_components(&["U1"])  // Must be in region
    
    // Placement rules
    .component_spacing(1.0);
```

## Component Constraints

### Position Constraints

```rust
ComponentConstraint::new("J1")
    // Fixed position
    .fixed_position(0.0, 40.0)
    .fixed_rotation(0.0)
    
    // Or constrained region
    .allowed_region(Rect::new(0.0, 30.0, 10.0, 50.0))
    
    // Layer
    .layer(Layer::FCu);
```

### Relative Constraints

```rust
// Keep components together
RelativeConstraint::group(&["U1", "C1", "C2", "C3"])
    .max_distance(5.0);

// Alignment
RelativeConstraint::align_horizontal(&["R1", "R2", "R3", "R4"]);

// Spacing
RelativeConstraint::equal_spacing(&["C1", "C2", "C3", "C4"])
    .direction(Direction::Horizontal);
```

## Manual Nudge Tools

### Nudge Operations

```rust
// Nudge component
editor.nudge_component("R1", Direction::Right, 0.5);

// Nudge selection
editor.nudge_selection(Direction::Up, 1.0);

// Nudge with snap
editor.nudge_to_grid("U1");
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Arrow keys | Nudge by grid |
| Shift+Arrow | Nudge by fine grid |
| Ctrl+Arrow | Nudge by large step |
| `N` | Nudge dialog |

### Nudge Dialog

```
┌─────────────────────────────────────────┐
│            Nudge Component              │
├─────────────────────────────────────────┤
│                                         │
│  Component: R1                          │
│                                         │
│  Direction:  ○ Up   ○ Down              │
│              ● Right ○ Left             │
│                                         │
│  Distance:   [0.5   ] mm                │
│                                         │
│  ☑ Snap to grid after nudge             │
│  ☐ Move connected traces                │
│                                         │
│  [Apply] [Cancel]                       │
│                                         │
└─────────────────────────────────────────┘
```

## Auto-Layout Helpers

### Alignment Tools

```rust
// Align to component
editor.align_to("R1", &["R2", "R3", "R4"], Alignment::Top);

// Align to grid
editor.align_to_grid(&selection);

// Distribute evenly
editor.distribute(&["C1", "C2", "C3", "C4"], Distribution::Horizontal);
```

### Smart Placement

```rust
SmartPlacement {
    // Automatic grouping
    group_by_net: true,
    group_by_function: true,
    
    // Spacing
    min_spacing: 0.5,
    preferred_spacing: 1.0,
    
    // Orientation
    align_similar: true,
    optimize_routing: true,
}
```

## Constraint Visualization

### Display Options

```rust
ConstraintDisplay {
    show_keepouts: true,
    show_regions: true,
    show_constraints: true,
    
    // Colors
    keepout_color: Color::rgba(1.0, 0.0, 0.0, 0.3),
    region_color: Color::rgba(0.0, 1.0, 0.0, 0.2),
}
```

### Constraint Markers

```
┌─────────────────────────────────────────┐
│                                         │
│  ┌─────────┐                            │
│  │ KEEP-OUT│  ← Red hatched area        │
│  │xxxxxxxx │                            │
│  └─────────┘                            │
│                                         │
│  ┌─────────────────┐                    │
│  │  Power Region   │  ← Green border    │
│  │  ┌───┐ ┌───┐    │                    │
│  │  │U1 │ │C1 │    │                    │
│  │  └───┘ └───┘    │                    │
│  └─────────────────┘                    │
│                                         │
│  R1 ──●── R2 ──●── R3  ← Alignment line │
│                                         │
└─────────────────────────────────────────┘
```

## Constraint Validation

### Check Constraints

```rust
let violations = pcb.check_constraints();

for violation in violations {
    println!("{}: {}", violation.constraint, violation.message);
}
```

### Violation Types

| Type | Description |
|------|-------------|
| Position | Component outside allowed region |
| Spacing | Components too close |
| Alignment | Alignment constraint broken |
| Keep-out | Item in keep-out zone |
| Net rule | Routing violates net constraint |

## Related Topics

- [Component Placement](../pcb-layout/component-placement.md)
- [Interactive Routing](../pcb-layout/interactive-routing.md)
- [Design Rule Check](../pcb-layout/drc.md)
