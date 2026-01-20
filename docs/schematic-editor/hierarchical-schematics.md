# Hierarchical & Multi-Sheet Schematics

## Overview

Hierarchical schematics enable complex designs to be organized into manageable, reusable blocks. Hardware Tool supports multi-sheet designs with full synchronization between hierarchy levels.

## Hierarchy Concepts

### Flat vs Hierarchical

| Approach | Use Case | Pros | Cons |
|----------|----------|------|------|
| **Flat** | Simple circuits | Easy to navigate | Cluttered for large designs |
| **Hierarchical** | Complex systems | Organized, reusable | More setup required |

### Hierarchy Structure

```
Root Sheet (main.hwt_sch)
├── Power Supply (power.hwt_sch)
│   ├── 5V Regulator
│   └── 3.3V Regulator
├── MCU Section (mcu.hwt_sch)
│   ├── Processor
│   └── Crystal/Reset
├── Interface (interface.hwt_sch)
│   ├── USB
│   └── UART
└── Sensors (sensors.hwt_sch)
    ├── Temperature
    └── Humidity
```

## Sheet Symbols

### Creating Sheet Symbols

Sheet symbols represent sub-schematics on the parent sheet:

```rust
let power_block = SheetSymbol::new("Power Supply")
    .file("power_supply.hwt_sch")
    .size(150, 100)
    .position(200, 100);
```

### Sheet Symbol Properties

| Property | Description |
|----------|-------------|
| **Name** | Display name on parent sheet |
| **File** | Path to sub-schematic file |
| **Size** | Visual dimensions |
| **Sheet Number** | Position in sheet sequence |

## Hierarchical Pins

### Pin Types

```rust
pub enum HierarchicalPinType {
    Input,       // Signal enters sub-sheet
    Output,      // Signal exits sub-sheet
    Bidirectional, // Signal flows both ways
    TriState,    // High-impedance capable
    Passive,     // No direction (power, ground)
}
```

### Defining Hierarchical Pins

On the **sheet symbol** (parent):

```rust
sheet_symbol
    .add_pin("VIN", HierarchicalPinType::Input, Side::Left)
    .add_pin("VOUT_5V", HierarchicalPinType::Output, Side::Right)
    .add_pin("VOUT_3V3", HierarchicalPinType::Output, Side::Right)
    .add_pin("GND", HierarchicalPinType::Passive, Side::Left);
```

On the **sub-sheet** (child):

```rust
// Hierarchical labels that connect to parent pins
HierarchicalLabel::new("VIN", HierarchicalPinType::Input);
HierarchicalLabel::new("VOUT_5V", HierarchicalPinType::Output);
```

## Labels & Connectivity

### Label Types

| Type | Scope | Symbol |
|------|-------|--------|
| **Local Label** | Current sheet only | `NET_NAME` |
| **Global Label** | Entire design | `>NET_NAME` |
| **Hierarchical Label** | Parent/child connection | `[NET_NAME]` |
| **Power Symbol** | Global power net | `VCC`, `GND` |

### Label Synchronization

```
Parent Sheet                    Child Sheet
─────────────                   ───────────
                               
    ┌──────────┐               ┌──────────┐
    │  Power   │               │          │
VIN─┤  Supply  ├─VOUT    VIN──►│ Regulator├──►VOUT
    │          │               │          │
    └──────────┘               └──────────┘
         │                           │
    Sheet Pin                  Hier. Label
    "VIN"                      "VIN"
```

## Multi-Sheet Navigation

### Sheet Index

```
Sheet Index
───────────
1/5  Main (main.hwt_sch)
2/5  Power Supply (power.hwt_sch)
3/5  MCU (mcu.hwt_sch)
4/5  Interface (interface.hwt_sch)
5/5  Sensors (sensors.hwt_sch)
```

### Navigation Commands

| Action | Shortcut |
|--------|----------|
| Go to sheet | `Ctrl+G` |
| Next sheet | `Page Down` |
| Previous sheet | `Page Up` |
| Enter sub-sheet | `Double-click` |
| Return to parent | `Backspace` |

## Reusable Blocks

### Instance Reuse

The same sub-schematic can be instantiated multiple times:

```rust
// Create two instances of the same filter design
let filter1 = SheetSymbol::new("Filter Ch1")
    .file("lowpass_filter.hwt_sch")
    .instance_id("FILTER1");

let filter2 = SheetSymbol::new("Filter Ch2")
    .file("lowpass_filter.hwt_sch")
    .instance_id("FILTER2");
```

### Instance-Specific Annotation

Each instance gets unique reference designators:

```
Filter Ch1: R101, R102, C101, C102
Filter Ch2: R201, R202, C201, C202
```

## Synchronization

### Forward Sync (Parent → Child)

- Pin additions/removals propagate to sub-sheet
- Name changes update hierarchical labels
- Type changes validated for compatibility

### Backward Sync (Child → Parent)

- New hierarchical labels create sheet pins
- Label deletions flag orphaned pins
- Property changes propagate up

### Conflict Resolution

```
Sync Conflict Detected
──────────────────────
Pin "DATA" exists on sheet symbol but 
no matching hierarchical label in sub-sheet.

Options:
[Create Label] [Remove Pin] [Ignore]
```

## Design Patterns

### Power Distribution

```
┌─────────────────────────────────────┐
│            Root Sheet               │
│  ┌─────────┐                        │
│  │  Power  │──VCC──┬──┬──┬──►       │
│  │  Supply │       │  │  │          │
│  │         │──GND──┼──┼──┼──►       │
│  └─────────┘       │  │  │          │
│                    │  │  │          │
│  ┌─────────┐       │  │  │          │
│  │   MCU   │◄──────┘  │  │          │
│  └─────────┘          │  │          │
│  ┌─────────┐          │  │          │
│  │  Sensor │◄─────────┘  │          │
│  └─────────┘             │          │
│  ┌─────────┐             │          │
│  │  Radio  │◄────────────┘          │
│  └─────────┘                        │
└─────────────────────────────────────┘
```

### Bus Distribution

```rust
// Define bus on parent
let data_bus = Bus::new("DATA[0..7]");

// Connect to multiple sub-sheets
sheet1.connect_bus(&data_bus);
sheet2.connect_bus(&data_bus);
```

## Related Topics

- [Symbols & Libraries](./symbols-libraries.md)
- [Wiring & Connectivity](./wiring-connectivity.md)
- [Annotation & Reference Designators](./annotation-reference-designators.md)
