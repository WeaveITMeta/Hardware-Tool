# Component Placement

## Overview

Component placement is the process of positioning footprints on the PCB. Good placement is critical for signal integrity, thermal management, manufacturability, and routability. Hardware Tool provides both manual and automated placement tools.

## Placement Workflow

```
┌─────────────────┐
│ Import Netlist  │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Initial Spread  │  ← Components outside board
└────────┬────────┘
         ▼
┌─────────────────┐
│ Group by        │  ← Functional clustering
│ Function        │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Place Critical  │  ← Connectors, MCU, power
│ Components      │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Place Remaining │  ← Fill in remaining parts
└────────┬────────┘
         ▼
┌─────────────────┐
│ Optimize        │  ← Fine-tune positions
└─────────────────┘
```

## Manual Placement

### Basic Operations

| Action | Shortcut | Description |
|--------|----------|-------------|
| Move | `M` | Pick up and move component |
| Rotate | `R` | Rotate 90° CCW |
| Flip | `F` | Move to opposite layer |
| Align | `Ctrl+A` | Align selected components |
| Distribute | `Ctrl+D` | Even spacing |

### Grid Settings

```rust
PlacementGrid {
    x_spacing: 0.5,      // mm
    y_spacing: 0.5,      // mm
    origin: Point::new(0.0, 0.0),
    
    // Fine grid for precision
    fine_grid: 0.1,
    
    // Snap options
    snap_to_grid: true,
    snap_to_pads: true,
}
```

### Rotation Options

```rust
pub enum RotationStep {
    Deg90,      // Standard 90° steps
    Deg45,      // 45° for angled routing
    Deg1,       // Fine 1° adjustment
    Free,       // Arbitrary angle
}
```

## Component Grouping

### Functional Groups

```rust
// Define component groups
let power_group = Group::new("Power Supply")
    .components(&["U1", "C1", "C2", "L1", "D1"])
    .color(Color::Red);

let mcu_group = Group::new("MCU Section")
    .components(&["U2", "Y1", "C3", "C4", "R1", "R2"])
    .color(Color::Blue);
```

### Group Operations

- Move group as unit
- Lock group positions
- Show/hide groups
- Highlight group members

### Clustering

```rust
// Auto-cluster by connectivity
schematic.cluster_by_connectivity(ClusterConfig {
    max_distance: 10.0,  // mm
    min_connections: 2,
});
```

## Auto-Placement Engines

### TSCircuit-Inspired Engines

Hardware Tool provides multiple auto-placement algorithms inspired by TSCircuit:

#### pcbFlex

Flexible constraint-based placement:

```rust
PcbFlex::new()
    .constraint(Constraint::near("U1", "C1", 5.0))
    .constraint(Constraint::align_horizontal(&["R1", "R2", "R3"]))
    .constraint(Constraint::edge("J1", Edge::Left))
    .run(&mut pcb);
```

#### pcbGrid

Grid-aligned regular placement:

```rust
PcbGrid::new()
    .grid_size(2.54)  // 0.1" grid
    .start_position(10.0, 10.0)
    .direction(Direction::LeftToRight)
    .components(&["R1", "R2", "R3", "R4"])
    .run(&mut pcb);
```

#### pcbPack

Density-optimized packing:

```rust
PcbPack::new()
    .target_density(0.7)  // 70% fill
    .respect_groups(true)
    .thermal_spacing(2.0)  // mm from hot components
    .run(&mut pcb);
```

## Placement Constraints

### Position Constraints

```rust
Constraint::fixed("J1", Position::new(0.0, 40.0));
Constraint::region("U1", Region::rect(20.0, 20.0, 60.0, 60.0));
Constraint::edge("J2", Edge::Right, 5.0);  // 5mm from edge
```

### Relative Constraints

```rust
Constraint::near("C1", "U1", 3.0);           // Within 3mm
Constraint::adjacent("R1", "R2");             // Side by side
Constraint::align_horizontal(&["C1", "C2", "C3"]);
Constraint::align_vertical(&["R1", "R2"]);
```

### Keep-Out Zones

```rust
KeepOut::rect(10.0, 10.0, 20.0, 20.0)
    .layers(&[Layer::FCu, Layer::BCu])
    .type_(KeepOutType::Components);

KeepOut::circle(50.0, 40.0, 5.0)
    .type_(KeepOutType::All);
```

## Layer Assignment

### Top vs Bottom Placement

```rust
// Force component to specific side
component.layer(Layer::FCu);   // Top
component.layer(Layer::BCu);   // Bottom

// Auto-assign based on rules
PlacementRules {
    smd_default: Layer::FCu,
    through_hole: Layer::FCu,
    
    // Exceptions
    bottom_components: vec!["J3", "J4"],  // Bottom connectors
}
```

### Mixed Technology

| Component Type | Preferred Side |
|----------------|----------------|
| Fine-pitch SMD | Top |
| Large SMD | Either |
| Through-hole | Top (soldered bottom) |
| Connectors | Edge, either side |
| Heat sinks | Top |

## Placement Strategies

### Signal Flow

```
┌─────────────────────────────────────────┐
│                                         │
│  INPUT ──► FILTER ──► AMPLIFIER ──► OUT │
│    │                                    │
│    ▼                                    │
│  POWER                                  │
│                                         │
└─────────────────────────────────────────┘
```

### Thermal Considerations

```rust
ThermalPlacement {
    // Hot components
    hot_components: vec!["U1", "Q1", "D1"],
    
    // Spacing from hot components
    min_spacing: 5.0,
    
    // Thermal via placement
    thermal_vias: true,
    
    // Heat sink clearance
    heatsink_clearance: 10.0,
}
```

### EMI/EMC Guidelines

1. **Keep high-speed signals short**
2. **Separate analog and digital**
3. **Ground plane under sensitive circuits**
4. **Shield noisy components**

## Placement Verification

### DRC for Placement

```rust
PlacementDrc {
    min_component_spacing: 0.2,
    courtyard_overlap: false,
    silkscreen_overlap: false,
    height_restrictions: true,
}
```

### Ratsnest Analysis

```
Ratsnest Statistics:
  Total connections: 245
  Average length: 12.3mm
  Longest: 45.2mm (NET_CLK)
  Crossings: 23
```

## Board Flip & Mirror

One-click board flip with automatic silkscreen mirroring and pin-1 indicator adjustment.

### Flip Board View

```rust
// Flip entire board view (bottom-up perspective)
pcb.flip_view(FlipOptions {
    // Mirror silkscreen text
    mirror_silkscreen: true,
    
    // Adjust pin-1 indicators
    adjust_pin1_markers: true,
    
    // Layer visibility
    swap_top_bottom_visibility: true,
    
    // Maintain orientation markers
    preserve_orientation_text: true,
});
```

### Flip UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Flip Board View                                      [Ctrl+F]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Current View: [● Top-Down] [○ Bottom-Up (Mirrored)]            │
│                                                                 │
│ When flipping:                                                  │
│   [☑] Mirror silkscreen text (keep readable)                   │
│   [☑] Adjust pin-1 dot indicators                              │
│   [☑] Swap layer visibility (F.Cu ↔ B.Cu)                      │
│   [☑] Mirror component outlines                                 │
│                                                                 │
│ [Flip View] [Cancel]                                            │
└─────────────────────────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+F` | Toggle board flip |
| `V` | Cycle layer visibility |
| `Shift+V` | Show bottom layer only |

## Nudge Cluster

Move multiple components together while preserving internal routing.

### Cluster Selection

```rust
// Select cluster
let cluster = pcb.select_cluster(&["U1", "C1", "C2", "C3", "R1", "R2"]);

// Nudge cluster
cluster.nudge(NudgeOptions {
    delta: (5.0, 0.0),         // mm
    
    // Preserve internal routing
    preserve_routing: true,
    
    // Move connected traces
    move_traces: true,
    
    // Adjust external connections
    reroute_external: true,
});
```

### Cluster Operations

```
┌─────────────────────────────────────────────────────────────────┐
│ Nudge Cluster                                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Selected: 6 components, 12 traces, 4 vias                       │
│                                                                 │
│ Move: X [5.0   ] mm   Y [0.0   ] mm                            │
│                                                                 │
│ Options:                                                        │
│   [☑] Preserve internal routing                                 │
│   [☑] Move connected traces                                     │
│   [☑] Reroute external connections                              │
│   [☐] Maintain grid alignment                                   │
│                                                                 │
│ [Nudge] [Preview] [Cancel]                                      │
└─────────────────────────────────────────────────────────────────┘
```

### Relative Constraints

```rust
// Define relative constraints between components
ComponentGroup::new("power_section")
    .add("U1")
    .add("C1").relative_to("U1", (2.0, 0.0))   // 2mm right of U1
    .add("C2").relative_to("U1", (-2.0, 0.0))  // 2mm left of U1
    .add("R1").relative_to("U1", (0.0, 3.0))   // 3mm below U1
    .lock_relative_positions(true);

// When U1 moves, C1, C2, R1 move with it
```

## Freeze Routing Zones

Protect already-approved routing while autorouting the rest.

### Define Freeze Zone

```rust
FreezeZone {
    name: "approved_power",
    
    // Zone boundary
    region: Rect::new(10.0, 10.0, 50.0, 40.0),
    
    // What to freeze
    freeze: FreezeOptions {
        traces: true,
        vias: true,
        zones: true,
        components: false,  // Allow component edits
    },
    
    // Protection level
    protection: Protection::Full,  // or Warn, or Soft
}
```

### Freeze Zone UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Freeze Zones                                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Active Zones:                                                   │
│   [☑] approved_power     (10,10)-(50,40)    [Edit] [Delete]    │
│   [☑] clock_routing      (60,20)-(80,60)    [Edit] [Delete]    │
│   [☐] analog_section     (0,50)-(30,80)     [Edit] [Delete]    │
│                                                                 │
│ [New Zone] [Draw Zone] [Clear All]                              │
│                                                                 │
│ Autorouter behavior:                                            │
│   [●] Route around frozen zones                                 │
│   [○] Allow crossing (no modifications)                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Freeze by Net

```rust
// Freeze specific nets
pcb.freeze_nets(&["VCC", "GND", "CLK"]);

// Freeze net class
pcb.freeze_net_class("power");

// Freeze all routed nets
pcb.freeze_routed();
```

## Related Topics

- [Footprints & Libraries](./footprints-libraries.md)
- [Interactive Routing](./interactive-routing.md)
- [Auto-Routing](./auto-routing.md)
