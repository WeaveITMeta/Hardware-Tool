# Wiring & Connectivity

## Overview

Wiring defines the electrical connections between components in a schematic. Hardware Tool provides multiple connectivity methods including wires, buses, labels, and power symbols, all organized through net classes.

## Connection Methods

### Wires

Direct point-to-point connections between pins:

```
Component A          Component B
    Pin 1 ●──────────● Pin 1
```

**Wire Properties:**
- Automatic junction creation at intersections
- Orthogonal routing (90° angles)
- Optional diagonal segments

### Wire Drawing

| Action | Result |
|--------|--------|
| Click-click | Orthogonal wire segment |
| Click-drag | Freeform wire |
| Double-click | End wire |
| `Esc` | Cancel wire |
| `/` | Toggle wire mode |

## Buses

### Bus Definition

Buses group related signals for cleaner schematics:

```
┌─────────┐                    ┌─────────┐
│         │   DATA[0..7]       │         │
│   MCU   ├════════════════════┤  Memory │
│         │                    │         │
└─────────┘                    └─────────┘
```

### Bus Syntax

```
DATA[0..7]      // 8-bit bus: DATA0, DATA1, ... DATA7
ADDR[0..15]     // 16-bit address bus
{SDA, SCL}      // Named signal group
{CLK, DATA, CS} // SPI-like bus
```

### Bus Entry

Connect individual signals to/from buses:

```
        DATA[0..7]
═══════════╦═══════════
           ║
    ┌──────╫──────┐
    │      ║      │
    │  D0 ─╫─     │
    │  D1 ─╫─     │
    │  D2 ─╫─     │
    └──────╫──────┘
           ║
```

## Labels

### Local Labels

Connect nets within the same sheet:

```
┌─────┐                      ┌─────┐
│ U1  ├──● CLK_OUT     CLK ●─┤ U2  │
└─────┘                      └─────┘
```

### Global Labels

Connect nets across all sheets:

```rust
GlobalLabel::new("RESET")
    .shape(LabelShape::Input)
    .position(100, 50);
```

**Label Shapes:**

| Shape | Meaning |
|-------|---------|
| `──►` | Input |
| `◄──` | Output |
| `◄─►` | Bidirectional |
| `───` | Tri-state |
| `○──` | Passive |

### Hierarchical Labels

Connect to parent sheet through sheet pins:

```rust
HierarchicalLabel::new("SPI_MOSI")
    .direction(Direction::Output)
    .position(0, 100);
```

## Power Symbols

### Standard Power Symbols

```
    VCC          GND          +5V         +3V3
     │            │            │            │
     ▼            ▼            ▼            ▼
    ─┴─          ─┬─          ─┴─          ─┴─
                 ═╧═         +5V          +3V3
```

### Power Symbol Properties

```rust
PowerSymbol {
    name: "VCC",
    net: "VCC",
    voltage: Some(3.3),
    visible: true,
    style: PowerStyle::Bar,
}
```

### Custom Power Rails

```rust
PowerSymbol::custom("+1V8_ANALOG")
    .style(PowerStyle::Arrow)
    .color(Color::Blue);
```

## Net Classes

### Definition

Group nets with common electrical requirements:

```rust
NetClass {
    name: "high_speed",
    description: "High-speed digital signals",
    
    // PCB constraints
    trace_width: 0.15,
    clearance: 0.2,
    via_diameter: 0.6,
    via_drill: 0.3,
    
    // Differential pair settings
    diff_pair_width: 0.1,
    diff_pair_gap: 0.15,
}
```

### Assigning Nets to Classes

```rust
// In schematic
net!("USB_D+").class("differential_usb");
net!("USB_D-").class("differential_usb");

// Or via net class rules
NetClassRule::new("high_speed")
    .match_pattern("CLK*")
    .match_pattern("DATA*");
```

### Default Net Classes

| Class | Use Case | Typical Width |
|-------|----------|---------------|
| `default` | General signals | 0.2mm |
| `power` | Power distribution | 0.5mm |
| `high_speed` | Fast digital | 0.15mm |
| `analog` | Sensitive analog | 0.25mm |

## Junction Management

### Automatic Junctions

Junctions are created when:
- Wire crosses another wire with connection
- Wire ends on existing wire
- Three or more wires meet

### Junction Symbols

```
    │           │           │
────●────   ────┼────   ────┤
    │           │           │
 Connected   Crossing    T-junction
              (no conn)
```

## Connectivity Validation

### Real-Time Checks

- Unconnected pins highlighted
- Floating nets marked
- Short circuits detected
- Bus width mismatches flagged

### Net Navigator

```rust
// Find all connections to a net
let connections = schematic.net("VCC").connections();
for conn in connections {
    println!("{}.{}", conn.component, conn.pin);
}
```

## Related Topics

- [Symbols & Libraries](./symbols-libraries.md)
- [Hierarchical Schematics](./hierarchical-schematics.md)
- [Electrical Rules Check](./erc.md)
