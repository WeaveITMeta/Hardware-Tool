# Footprints & Libraries

## Overview

Footprints define the physical representation of components on a PCB, including pad geometry, silkscreen outlines, courtyard boundaries, and 3D model references. Hardware Tool provides comprehensive footprint libraries and tools for creating custom footprints.

## Footprint Anatomy

### Layer Structure

```
┌─────────────────────────────────────────┐
│           F.Courtyard                   │  ← Assembly boundary
│  ┌─────────────────────────────────┐    │
│  │        F.SilkS                  │    │  ← Component outline
│  │  ┌─────────────────────────┐    │    │
│  │  │      F.Fab              │    │    │  ← Fabrication layer
│  │  │  ┌─────────────────┐    │    │    │
│  │  │  │    F.Paste      │    │    │    │  ← Solder paste
│  │  │  │  ┌───────────┐  │    │    │    │
│  │  │  │  │  F.Mask   │  │    │    │    │  ← Solder mask opening
│  │  │  │  │  ┌─────┐  │  │    │    │    │
│  │  │  │  │  │F.Cu │  │  │    │    │    │  ← Copper pad
│  │  │  │  │  └─────┘  │  │    │    │    │
│  │  │  │  └───────────┘  │    │    │    │
│  │  │  └─────────────────┘    │    │    │
│  │  └─────────────────────────┘    │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

### Footprint Elements

| Element | Layer | Purpose |
|---------|-------|---------|
| **Pads** | F.Cu/B.Cu | Electrical connection |
| **Solder Mask** | F.Mask/B.Mask | Mask openings |
| **Solder Paste** | F.Paste/B.Paste | Stencil apertures |
| **Silkscreen** | F.SilkS/B.SilkS | Component marking |
| **Courtyard** | F.CrtYd/B.CrtYd | Placement boundary |
| **Fabrication** | F.Fab/B.Fab | Assembly reference |

## Pad Definitions

### Pad Types

```rust
pub enum PadType {
    Smd,           // Surface mount
    Through,       // Through-hole
    NpthHole,      // Non-plated through hole
    Connector,     // Edge connector
}

pub enum PadShape {
    Circle,
    Rectangle,
    RoundRect { ratio: f64 },
    Oval,
    Trapezoid,
    Custom { polygon: Vec<Point> },
}
```

### Pad Properties

```rust
Pad {
    number: "1",
    type_: PadType::Smd,
    shape: PadShape::RoundRect { ratio: 0.25 },
    position: Point { x: 0.0, y: 0.0 },
    size: Size { width: 1.0, height: 1.5 },
    layers: vec![Layer::FCu, Layer::FPaste, Layer::FMask],
    
    // Thermal relief
    thermal_gap: 0.5,
    thermal_spokes: 4,
    
    // Solder mask/paste
    mask_margin: 0.05,
    paste_margin: -0.05,
    paste_ratio: 0.9,
    
    // Drill (for through-hole)
    drill: None,
}
```

### Through-Hole Pads

```rust
Pad::through_hole("1")
    .position(0.0, 0.0)
    .size(1.8, 1.8)
    .drill(1.0)
    .shape(PadShape::Circle)
```

### SMD Pads

```rust
Pad::smd("1")
    .position(-0.75, 0.0)
    .size(0.9, 1.2)
    .shape(PadShape::RoundRect { ratio: 0.25 })
    .layers(&[Layer::FCu, Layer::FPaste, Layer::FMask])
```

## Footprint File Format

### `.hwt_fp` Format

```toml
[footprint]
name = "R_0603"
description = "Resistor SMD 0603 (1608 Metric)"
keywords = ["resistor", "0603", "smd"]
tags = ["smd", "passive"]

[properties]
reference = { value = "REF**", position = [0, -1.5], layer = "F.SilkS" }
value = { value = "VAL**", position = [0, 1.5], layer = "F.Fab" }

[[pads]]
number = "1"
type = "smd"
shape = "roundrect"
position = [-0.75, 0]
size = [0.9, 0.95]
roundrect_ratio = 0.25
layers = ["F.Cu", "F.Paste", "F.Mask"]

[[pads]]
number = "2"
type = "smd"
shape = "roundrect"
position = [0.75, 0]
size = [0.9, 0.95]
roundrect_ratio = 0.25
layers = ["F.Cu", "F.Paste", "F.Mask"]

[[graphics]]
type = "rectangle"
layer = "F.SilkS"
start = [-0.8, -0.4]
end = [0.8, 0.4]
width = 0.12

[[graphics]]
type = "rectangle"
layer = "F.CrtYd"
start = [-1.5, -0.75]
end = [1.5, 0.75]
width = 0.05

[3d_model]
path = "Package_SMD.3dshapes/R_0603.step"
offset = [0, 0, 0]
rotation = [0, 0, 0]
scale = [1, 1, 1]
```

## Library Organization

### Directory Structure

```
libraries/
├── footprints/
│   ├── Package_SMD.hwt_fplib/
│   │   ├── R_0402.hwt_fp
│   │   ├── R_0603.hwt_fp
│   │   ├── C_0402.hwt_fp
│   │   └── ...
│   ├── Package_QFP.hwt_fplib/
│   ├── Package_BGA.hwt_fplib/
│   └── Connector_USB.hwt_fplib/
└── 3dmodels/
    ├── Package_SMD.3dshapes/
    └── ...
```

### Library Table

```toml
# fp-lib-table
[[libraries]]
name = "Package_SMD"
type = "HwtLib"
uri = "${HWTOOLS_LIBS}/footprints/Package_SMD.hwt_fplib"

[[libraries]]
name = "MyCustom"
type = "HwtLib"
uri = "${PROJECT}/libraries/custom.hwt_fplib"
```

## Creating Custom Footprints

### Using the GUI Editor

1. **File → New Footprint**
2. Set grid and origin
3. Add pads with properties
4. Draw silkscreen outline
5. Add courtyard
6. Assign 3D model
7. Save to library

### Programmatic Definition

```rust
use hardware_tool::footprint::*;

let footprint = Footprint::builder("SOT-23-3")
    .description("SOT-23, 3 pin")
    .tags(&["smd", "transistor", "sot23"])
    
    // Pads
    .pad(Pad::smd("1").at(-0.95, 1.0).size(0.9, 0.8))
    .pad(Pad::smd("2").at(-0.95, -1.0).size(0.9, 0.8))
    .pad(Pad::smd("3").at(0.95, 0.0).size(0.9, 0.8))
    
    // Silkscreen
    .silkscreen(|s| {
        s.line((-0.35, 1.45), (0.35, 1.45));
        s.line((-0.35, -1.45), (0.35, -1.45));
    })
    
    // Courtyard
    .courtyard(Rect::new(-1.7, -1.75, 1.7, 1.75))
    
    // 3D model
    .model_3d("Package_TO_SOT_SMD.3dshapes/SOT-23.step")
    
    .build();
```

## Footprint Assignment

### From Schematic

```rust
// In schematic symbol
Component {
    reference: "R1",
    value: "10k",
    footprint: "Package_SMD:R_0603",
}
```

### Footprint Filters

```rust
Symbol {
    footprint_filters: vec![
        "R_0402*",
        "R_0603*",
        "R_0805*",
        "R_1206*",
    ],
}
```

### Assignment Table

| Reference | Value | Footprint |
|-----------|-------|-----------|
| R1-R10 | 10k | Package_SMD:R_0603 |
| C1-C5 | 100nF | Package_SMD:C_0402 |
| U1 | STM32F4 | Package_QFP:LQFP-64 |

## 3D Models

### Supported Formats

| Format | Extension | Use Case |
|--------|-----------|----------|
| STEP | .step, .stp | Mechanical CAD |
| VRML | .wrl | Colored visualization |
| IDF | .idf | Board exchange |

### Model Properties

```rust
Model3D {
    path: "models/capacitor.step",
    offset: Vector3::new(0.0, 0.0, 0.0),
    rotation: Vector3::new(0.0, 0.0, 0.0),
    scale: Vector3::new(1.0, 1.0, 1.0),
    opacity: 1.0,
}
```

## Import/Export

### Supported Formats

| Format | Import | Export |
|--------|--------|--------|
| KiCAD `.kicad_mod` | ✓ | ✓ |
| Eagle `.lbr` | ✓ | - |
| Altium `.PcbLib` | ✓ | - |
| IPC-7351 | ✓ | ✓ |

## Related Topics

- [Component Placement](./component-placement.md)
- [Library Conventions](../advanced-features/library-conventions.md)
- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md)
