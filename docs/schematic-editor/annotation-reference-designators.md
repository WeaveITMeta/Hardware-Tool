# Annotation & Reference Designators

## Overview

Annotation assigns unique reference designators to components, enabling unambiguous identification throughout the design process. Hardware Tool supports automatic numbering, custom schemes, and bidirectional synchronization with PCB layout.

## Reference Designator Conventions

### Standard Prefixes

| Prefix | Component Type |
|--------|----------------|
| R | Resistor |
| C | Capacitor |
| L | Inductor |
| D | Diode |
| Q | Transistor |
| U | Integrated Circuit |
| J | Connector |
| P | Plug |
| SW | Switch |
| F | Fuse |
| Y | Crystal/Oscillator |
| T | Transformer |
| K | Relay |
| TP | Test Point |

### Designator Format

```
[Prefix][Number][Suffix]

Examples:
  R1, R2, R3          - Simple sequential
  R101, R102          - Sheet-based (1xx = sheet 1)
  U1A, U1B            - Multi-unit components
  C1_PWR, C2_PWR      - Functional suffix
```

## Automatic Annotation

### Annotation Modes

```rust
pub enum AnnotationMode {
    Sequential,           // R1, R2, R3...
    SheetBased,          // R101, R201, R301...
    PositionalX,         // Left to right
    PositionalY,         // Top to bottom
    PositionalXY,        // Left-right, then top-bottom
}
```

### Configuration

```rust
AnnotationConfig {
    mode: AnnotationMode::SheetBased,
    start_number: 1,
    increment: 1,
    sheet_multiplier: 100,  // Sheet 1 = 1xx, Sheet 2 = 2xx
    
    // Per-type overrides
    overrides: hashmap! {
        "R" => AnnotationOverride { start: 1, increment: 1 },
        "U" => AnnotationOverride { start: 1, increment: 1 },
    },
}
```

### Running Annotation

```rust
// Annotate entire schematic
schematic.annotate(AnnotationConfig::default());

// Annotate selection only
schematic.annotate_selection(&selected_components, config);

// Re-annotate (reset and renumber)
schematic.reannotate(config);
```

## Annotation Scope

### Sheet-Based Numbering

```
Sheet 1 (main.hwt_sch):
  R101, R102, R103
  C101, C102
  U101

Sheet 2 (power.hwt_sch):
  R201, R202
  C201, C202, C203
  U201, U202

Sheet 3 (interface.hwt_sch):
  R301, R302, R303, R304
  C301
  U301
```

### Hierarchical Instance Numbering

For reused sub-sheets:

```
Filter Instance 1:
  R1_1, R1_2, C1_1, C1_2

Filter Instance 2:
  R2_1, R2_2, C2_1, C2_2
```

## Multi-Unit Components

### Unit Designation

```rust
// Quad op-amp with 4 units
Component {
    reference: "U1",
    units: vec![
        Unit { id: "A", pins: ["1", "2", "3"] },
        Unit { id: "B", pins: ["4", "5", "6"] },
        Unit { id: "C", pins: ["7", "8", "9"] },
        Unit { id: "D", pins: ["10", "11", "12"] },
    ],
    common_pins: vec!["VCC", "GND"],
}
```

### Unit Assignment

```
U1A - Sheet 1 (audio input)
U1B - Sheet 1 (audio output)
U1C - Sheet 2 (sensor buffer)
U1D - Unassigned
```

## Back-Annotation

### PCB → Schematic Sync

Changes made in PCB layout propagate back to schematic:

```rust
BackAnnotation {
    // Reference swaps
    swaps: vec![
        RefSwap { from: "R5", to: "R12" },
        RefSwap { from: "R12", to: "R5" },
    ],
    
    // Pin/gate swaps
    pin_swaps: vec![
        PinSwap { component: "U1", unit_from: "A", unit_to: "B" },
    ],
    
    // Value updates
    value_changes: vec![
        ValueChange { ref: "R3", old: "10k", new: "12k" },
    ],
}
```

### Applying Back-Annotation

```
Back-Annotation Preview
═══════════════════════

Reference Swaps:
  R5 ↔ R12 (layout optimization)
  
Pin Swaps:
  U1: Unit A ↔ Unit B (routing improvement)
  
Value Changes:
  R3: 10k → 12k (tuning adjustment)

[Apply All] [Apply Selected] [Cancel]
```

## Annotation Conflicts

### Duplicate Detection

```
Annotation Conflict Detected
────────────────────────────
Reference "R15" is used by multiple components:

  1. Sheet 1, (100, 50): Resistor 10k
  2. Sheet 2, (200, 75): Resistor 4.7k

Resolution:
  [Renumber Sheet 2] [Renumber Sheet 1] [Manual Fix]
```

### Reserved Designators

```rust
AnnotationConfig {
    reserved: vec![
        "R1".."R10",    // Reserved for power section
        "U50".."U99",   // Reserved for expansion
    ],
}
```

## Annotation Report

### Summary

```
Annotation Report
═════════════════

Components Annotated: 156
  Resistors (R):     45
  Capacitors (C):    32
  ICs (U):           12
  Connectors (J):     8
  Other:             59

Sheets Processed: 5
Conflicts Resolved: 2
Warnings: 1

Designator Ranges:
  R: 1-45
  C: 1-32
  U: 1-12
  J: 1-8
```

### Export Formats

| Format | Use Case |
|--------|----------|
| CSV | Spreadsheet analysis |
| JSON | Automation/scripting |
| PDF | Documentation |

## Best Practices

1. **Annotate early** - Assign designators before detailed wiring
2. **Use sheet-based numbering** - Easier to locate components
3. **Reserve ranges** - For future expansion or specific sections
4. **Document swaps** - Track back-annotation changes
5. **Verify before layout** - Ensure consistent designators

## Silkscreen Reference Optimizer

Automatically reposition reference designators on the PCB silkscreen for optimal readability and manufacturing.

### Optimization Goals

| Goal | Description |
|------|-------------|
| **Avoid pads** | Keep text clear of solder joints |
| **Avoid vias** | Don't overlap via holes |
| **Stay on component** | Position near associated part |
| **Readable orientation** | Prefer 0° or 90° rotation |
| **Consistent size** | Uniform text height across board |

### Optimizer UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Silkscreen Reference Optimizer                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Scope: [● All Components] [○ Selected] [○ By Type]             │
│                                                                 │
│ Optimization Rules:                                             │
│   [☑] Avoid solder pads (clearance: [0.2  ] mm)                │
│   [☑] Avoid vias (clearance: [0.15 ] mm)                       │
│   [☑] Avoid copper (clearance: [0.1  ] mm)                     │
│   [☑] Stay within courtyard                                     │
│   [☑] Prefer horizontal orientation                             │
│   [☐] Allow 90° rotation only                                   │
│   [☑] Uniform text size ([0.8  ] mm)                           │
│                                                                 │
│ Placement Priority:                                             │
│   1. [Top of component    ▼]                                   │
│   2. [Right of component  ▼]                                   │
│   3. [Bottom of component ▼]                                   │
│   4. [Left of component   ▼]                                   │
│                                                                 │
│ Preview: 42 designators will be repositioned                    │
│          3 designators have no valid position (manual needed)   │
│                                                                 │
│ [Optimize] [Preview] [Reset to Default] [Cancel]                │
└─────────────────────────────────────────────────────────────────┘
```

### Optimizer Configuration

```rust
SilkscreenOptimizer {
    // Clearance rules
    clearance: SilkClearance {
        to_pad: 0.2,           // mm
        to_via: 0.15,
        to_copper: 0.1,
        to_board_edge: 0.5,
        to_other_silk: 0.1,
    },
    
    // Placement preferences
    placement: PlacementPrefs {
        prefer_positions: vec![
            Position::Top,
            Position::Right,
            Position::Bottom,
            Position::Left,
        ],
        stay_in_courtyard: true,
        max_distance: 2.0,     // mm from component center
    },
    
    // Orientation
    orientation: OrientationPrefs {
        prefer_horizontal: true,
        allow_angles: vec![0, 90, 180, 270],
        match_component_rotation: false,
    },
    
    // Text properties
    text: TextProps {
        height: 0.8,           // mm
        width: 0.8,
        thickness: 0.15,
        font: Font::Default,
    },
}
```

### Optimization API

```rust
// Optimize all designators
pcb.optimize_silkscreen_refs(SilkscreenOptimizer::default())?;

// Optimize selected components
pcb.optimize_silkscreen_refs_for(&["U1", "U2", "R1".."R20"])?;

// Optimize by component type
pcb.optimize_silkscreen_refs_by_type(ComponentType::SMD)?;

// Preview changes
let preview = pcb.preview_silkscreen_optimization()?;
for change in preview.changes {
    println!("{}: {:?} -> {:?}", change.ref, change.old_pos, change.new_pos);
}
```

### Collision Detection

```rust
SilkCollisionCheck {
    // Check for overlaps
    check_pad_overlap: true,
    check_via_overlap: true,
    check_silk_overlap: true,
    
    // Report
    report_unplaceable: true,
    suggest_alternatives: true,
}
```

### Manual Override

```rust
// Lock specific designators (won't be moved by optimizer)
pcb.lock_silk_position("U1");
pcb.lock_silk_position("J1");

// Set custom position
pcb.set_silk_position("R5", SilkPosition {
    offset: (1.0, 0.5),        // mm from component center
    rotation: 0,
    layer: Layer::FSilkS,
});
```

### Optimization Report

```
Silkscreen Optimization Report
══════════════════════════════

Processed: 156 designators
Repositioned: 42
Unchanged: 111
Manual needed: 3

Repositioned:
  R1:  (0.0, -1.2) → (1.5, 0.0)  [was overlapping pad]
  U3:  (0.0, 2.0) → (0.0, -2.5)  [was outside courtyard]
  C15: rotated 90° [better fit]

Manual attention needed:
  U1:  No valid position (dense area)
  J5:  Conflicts with mounting hole
  Q2:  Overlaps with adjacent component

[Export Report] [Show on PCB] [Close]
```

## Related Topics

- [Symbols & Libraries](./symbols-libraries.md)
- [Hierarchical Schematics](./hierarchical-schematics.md)
- [Schematic Capture Workflow](../core-architecture/schematic-capture-workflow.md)
