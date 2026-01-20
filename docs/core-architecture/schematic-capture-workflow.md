# Schematic Capture Workflow

## Overview

The schematic capture workflow in Hardware Tool transforms abstract circuit concepts into a complete, validated netlist ready for PCB layout. This document outlines the end-to-end process from initial design to netlist generation.

## Workflow Stages

```
┌─────────────────┐
│  Circuit Idea   │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Symbol Placement│
└────────┬────────┘
         ▼
┌─────────────────┐
│    Wiring       │
└────────┬────────┘
         ▼
┌─────────────────┐
│   Annotation    │
└────────┬────────┘
         ▼
┌─────────────────┐
│      ERC        │
└────────┬────────┘
         ▼
┌─────────────────┐
│ Netlist Export  │
└────────┴────────┘
```

## Stage 1: Symbol Placement

### Adding Components

1. **Library Browser**: Search and select symbols from libraries
2. **Quick Add**: Keyboard shortcuts for common components (R, C, L, etc.)
3. **Programmatic**: Define components via Rust code

### Component Properties

- Reference designator (auto-assigned or manual)
- Value (resistance, capacitance, part number)
- Footprint assignment
- Custom fields (manufacturer, cost, etc.)

## Stage 2: Wiring & Connectivity

### Connection Methods

| Method | Use Case |
|--------|----------|
| Wire | Direct point-to-point connection |
| Bus | Grouped signals (data buses) |
| Label | Named net connections across sheets |
| Power Symbol | VCC, GND, and power rails |
| Hierarchical Pin | Inter-sheet connections |

### Net Classes

Define electrical characteristics for groups of nets:

```rust
NetClass {
    name: "high_speed",
    trace_width: 0.15,
    clearance: 0.2,
    diff_pair: true,
}
```

## Stage 3: Annotation

### Automatic Annotation

- Sequential numbering by component type
- Sheet-based prefixes for hierarchical designs
- Configurable starting numbers and increments

### Back-Annotation

Synchronize changes from PCB layout back to schematic:

- Reference designator swaps
- Pin/gate swaps
- Component value updates

## Stage 4: Electrical Rules Check (ERC)

### Validation Categories

1. **Pin Conflicts**: Output driving output, unconnected inputs
2. **Power Issues**: Missing power connections, voltage conflicts
3. **Label Errors**: Duplicate labels, orphaned labels
4. **Hierarchy**: Unconnected sheet pins, missing sub-sheets

### Error Severity

- **Error**: Must fix before proceeding
- **Warning**: Review recommended
- **Info**: Informational only

## Stage 5: Netlist Generation

### Output Formats

| Format | Purpose |
|--------|---------|
| Circuit JSON | Internal IR for PCB layout |
| SPICE | Simulation (ngspice compatible) |
| KiCAD | Interoperability |
| Custom | User-defined templates |

### Netlist Contents

```json
{
  "components": [
    {
      "ref": "R1",
      "value": "10k",
      "footprint": "Resistor_SMD:R_0603"
    }
  ],
  "nets": [
    {
      "name": "VCC",
      "nodes": ["U1.14", "C1.1", "R1.1"]
    }
  ]
}
```

## Best Practices

1. **Organize hierarchically** for complex designs
2. **Use net labels** instead of long wires
3. **Run ERC frequently** during design
4. **Document** with text notes and frames
5. **Version control** schematic files

## Related Topics

- [Symbols & Libraries](../schematic-editor/symbols-libraries.md)
- [Hierarchical Schematics](../schematic-editor/hierarchical-schematics.md)
- [Electrical Rules Check](../schematic-editor/erc.md)
- [PCB Layout Workflow](./pcb-layout-workflow.md)
