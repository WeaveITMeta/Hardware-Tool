# Circuit JSON as Intermediate Representation

## Overview

Circuit JSON serves as the universal data model for Hardware Tool, providing a standardized intermediate representation (IR) for schematics, layouts, nets, and constraints. This format is inspired by TSCircuit's approach, enabling seamless data exchange between different stages of the design flow.

## Purpose

The Circuit JSON IR acts as:

- **Bridge** between schematic capture and PCB layout
- **Exchange format** for external tools and automation
- **Serialization layer** for undo/redo and versioning
- **API interface** for programmatic circuit manipulation

## Schema Structure

### Root Document

```json
{
  "version": "1.0",
  "metadata": {
    "name": "Example Circuit",
    "created": "2026-01-19T16:00:00Z",
    "tool_version": "0.1.0"
  },
  "components": [],
  "nets": [],
  "constraints": [],
  "layout": {}
}
```

### Components

```json
{
  "components": [
    {
      "id": "R1",
      "type": "resistor",
      "value": "10k",
      "footprint": "0603",
      "properties": {
        "tolerance": "1%",
        "power_rating": "0.1W"
      },
      "pins": [
        { "id": "1", "name": "1", "net": "VCC" },
        { "id": "2", "name": "2", "net": "NODE_A" }
      ],
      "position": { "x": 100.0, "y": 50.0 },
      "rotation": 0
    }
  ]
}
```

### Nets

```json
{
  "nets": [
    {
      "id": "VCC",
      "class": "power",
      "properties": {
        "voltage": "3.3V",
        "max_current": "500mA"
      },
      "connections": [
        { "component": "U1", "pin": "VDD" },
        { "component": "C1", "pin": "1" },
        { "component": "R1", "pin": "1" }
      ]
    }
  ]
}
```

### Constraints

```json
{
  "constraints": [
    {
      "type": "clearance",
      "scope": "net:VCC",
      "value": 0.2,
      "unit": "mm"
    },
    {
      "type": "trace_width",
      "scope": "class:power",
      "min": 0.3,
      "max": 1.0,
      "unit": "mm"
    },
    {
      "type": "differential_pair",
      "nets": ["USB_D+", "USB_D-"],
      "impedance": 90,
      "tolerance": 10
    }
  ]
}
```

### Layout Data

```json
{
  "layout": {
    "board_outline": {
      "type": "polygon",
      "points": [[0,0], [100,0], [100,80], [0,80]]
    },
    "layers": ["F.Cu", "In1.Cu", "In2.Cu", "B.Cu"],
    "traces": [],
    "vias": [],
    "zones": []
  }
}
```

## Rust Integration

### Serde Serialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CircuitJson {
    pub version: String,
    pub metadata: Metadata,
    pub components: Vec<Component>,
    pub nets: Vec<Net>,
    pub constraints: Vec<Constraint>,
    pub layout: Option<Layout>,
}
```

### Type Safety

The Rust implementation provides:

- Compile-time validation of circuit structures
- Strong typing for electrical properties
- Unit-aware measurements (mm, mil, ohm, etc.)

## Use Cases

1. **Schematic → Netlist**: Extract connectivity from schematic
2. **Netlist → PCB**: Initialize layout with component placement
3. **Design Sync**: Bidirectional updates between schematic and PCB
4. **Automation**: Script-based circuit generation and modification
5. **Export**: Convert to manufacturer-specific formats

## Related Topics

- [Project Structure & Management](./project-structure-management.md)
- [Schematic Capture Workflow](./schematic-capture-workflow.md)
- [PCB Layout Workflow](./pcb-layout-workflow.md)
