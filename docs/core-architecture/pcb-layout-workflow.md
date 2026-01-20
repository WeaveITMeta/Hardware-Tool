# PCB Layout Workflow

## Overview

The PCB layout workflow transforms a validated netlist into a physical board design ready for manufacturing. Hardware Tool provides both interactive and automated approaches to component placement and routing.

## Workflow Stages

```
┌─────────────────┐
│  Netlist Import │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Board Setup    │
└────────┬────────┘
         ▼
┌─────────────────┐
│   Placement     │
└────────┬────────┘
         ▼
┌─────────────────┐
│    Routing      │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Copper Zones   │
└────────┬────────┘
         ▼
┌─────────────────┐
│      DRC        │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Manufacturing  │
└────────┴────────┘
```

## Stage 1: Netlist Import

### From Schematic

- Automatic footprint assignment from schematic
- Component grouping preservation
- Net class inheritance

### Synchronization

- Forward annotation (schematic → PCB)
- Back annotation (PCB → schematic)
- Change detection and conflict resolution

## Stage 2: Board Setup

### Board Outline

```rust
BoardOutline::rectangle(100.0, 80.0, Unit::Mm)
    .with_corner_radius(3.0)
    .with_mounting_holes(vec![
        MountingHole::new(5.0, 5.0, 3.2),
        MountingHole::new(95.0, 5.0, 3.2),
        MountingHole::new(5.0, 75.0, 3.2),
        MountingHole::new(95.0, 75.0, 3.2),
    ])
```

### Layer Stackup

| Layer | Type | Thickness |
|-------|------|-----------|
| F.Cu | Signal | 35µm |
| Prepreg | Dielectric | 0.2mm |
| In1.Cu | Ground | 35µm |
| Core | Dielectric | 1.0mm |
| In2.Cu | Power | 35µm |
| Prepreg | Dielectric | 0.2mm |
| B.Cu | Signal | 35µm |

### Design Rules

- Minimum trace width
- Minimum clearance
- Via sizes and types
- Differential pair constraints

## Stage 3: Component Placement

### Manual Placement

- Drag and drop with grid snapping
- Rotation (90°, 45°, arbitrary)
- Flip to bottom layer
- Alignment tools

### Auto-Placement Hints

TSCircuit-inspired placement engines:

| Engine | Description |
|--------|-------------|
| `pcbFlex` | Flexible constraint-based placement |
| `pcbGrid` | Grid-aligned regular placement |
| `pcbPack` | Density-optimized packing |

### Placement Strategies

1. **Functional grouping**: Keep related components together
2. **Signal flow**: Arrange by signal path
3. **Thermal**: Separate heat-generating components
4. **EMI**: Shield sensitive circuits

## Stage 4: Routing

### Interactive Routing

- Click-to-route with obstacle avoidance
- Push-and-shove for tight spaces
- Via insertion (through, blind, buried)
- Layer switching

### Differential Pairs

```rust
DifferentialPair::new("USB_D+", "USB_D-")
    .impedance(90.0)
    .spacing(0.15)
    .length_matching(0.1)
```

### Length Tuning

- Serpentine/meander patterns
- Target length specification
- Real-time length display

### Auto-Routing

- Full board auto-route
- Selected nets only
- Fanout routing for BGAs

## Stage 5: Copper Zones

### Zone Types

- Ground planes
- Power planes
- Thermal relief pads
- Keep-out areas

### Zone Properties

```rust
CopperZone::new("GND")
    .layer(Layer::In1Cu)
    .priority(1)
    .clearance(0.3)
    .thermal_relief(ThermalRelief::Spokes(4, 0.25))
    .fill_mode(FillMode::Solid)
```

## Stage 6: Design Rule Check (DRC)

### Check Categories

| Category | Examples |
|----------|----------|
| Clearance | Trace-to-trace, trace-to-pad |
| Connection | Unconnected pads, broken nets |
| Size | Min trace width, min drill |
| Via | Annular ring, drill size |
| Zone | Isolation, minimum width |
| Courtyard | Component overlap |

### DRC Report

```
DRC Report - 2026-01-19
========================
Errors: 0
Warnings: 3

[W] Trace width below recommended (0.12mm < 0.15mm)
    Location: (45.2, 32.1) on F.Cu
    Net: DATA_0

[W] Via near board edge (1.8mm < 2.0mm)
    Location: (98.2, 40.0)
```

## Stage 7: Manufacturing Output

### Output Generation

- Gerber RS-274X
- Excellon drill files
- IPC-2581
- ODB++
- Pick and place
- BOM

## Related Topics

- [Footprints & Libraries](../pcb-layout/footprints-libraries.md)
- [Interactive Routing](../pcb-layout/interactive-routing.md)
- [Design Rule Check](../pcb-layout/drc.md)
- [Gerber Export](../manufacturing-output/gerber-export.md)
