# Gerber RS-274X Export

## Overview

Gerber RS-274X is the industry-standard format for PCB manufacturing. Hardware Tool exports all necessary layers including copper, solder mask, silkscreen, drill files (Excellon), and board outline for fabrication.

## Gerber Format

### RS-274X Extended Gerber

Hardware Tool uses RS-274X (Extended Gerber) format with embedded apertures:

```gerber
G04 Hardware Tool Gerber Export*
G04 Layer: F.Cu*
%FSLAX46Y46*%
%MOIN*%
%ADD10C,0.010*%
%ADD11R,0.060X0.040*%
D10*
X100000Y100000D03*
X200000Y100000D01*
...
M02*
```

### Format Features

- Embedded aperture definitions
- Extended commands (attributes)
- High precision coordinates
- Self-contained files

## Layer Export

### Standard Layers

| Layer | Filename | Description |
|-------|----------|-------------|
| F.Cu | *-F_Cu.gbr | Top copper |
| B.Cu | *-B_Cu.gbr | Bottom copper |
| In1.Cu | *-In1_Cu.gbr | Inner layer 1 |
| In2.Cu | *-In2_Cu.gbr | Inner layer 2 |
| F.Mask | *-F_Mask.gbr | Top solder mask |
| B.Mask | *-B_Mask.gbr | Bottom solder mask |
| F.Paste | *-F_Paste.gbr | Top paste stencil |
| B.Paste | *-B_Paste.gbr | Bottom paste stencil |
| F.SilkS | *-F_SilkS.gbr | Top silkscreen |
| B.SilkS | *-B_SilkS.gbr | Bottom silkscreen |
| Edge.Cuts | *-Edge_Cuts.gbr | Board outline |

### Export Configuration

```rust
GerberExport::new(&pcb)
    .output_dir("./gerber")
    .prefix("my_board")
    
    // Layer selection
    .layers(&[
        Layer::FCu,
        Layer::BCu,
        Layer::FMask,
        Layer::BMask,
        Layer::FSilkS,
        Layer::BSilkS,
        Layer::EdgeCuts,
    ])
    
    // Options
    .use_protel_extensions(true)
    .use_aux_origin(false)
    .subtract_mask_from_silk(true)
    
    .export()?;
```

## Drill Files (Excellon)

### Drill Export

```rust
DrillExport::new(&pcb)
    .output_dir("./gerber")
    .prefix("my_board")
    
    // Format
    .format(DrillFormat::Excellon2)
    .units(Units::Millimeters)
    .zeros(ZeroFormat::SuppressLeading)
    
    // Options
    .merge_pth_npth(false)  // Separate files
    .minimal_header(false)
    .mirror_y(false)
    
    .export()?;
```

### Drill File Types

| File | Description |
|------|-------------|
| *-PTH.drl | Plated through holes |
| *-NPTH.drl | Non-plated holes |
| *-PTH-drl_map.gbr | Drill map (Gerber) |

### Excellon Format

```excellon
M48
; Hardware Tool Drill Export
; Date: 2026-01-19
METRIC,TZ
T1C0.300
T2C0.800
T3C1.000
%
T1
X10000Y15000
X20000Y15000
X30000Y15000
T2
X50000Y40000
T3
X5000Y5000
X95000Y5000
M30
```

## Export Options

### Coordinate Format

```rust
CoordinateFormat {
    // Integer + decimal digits
    integer_digits: 4,
    decimal_digits: 6,
    
    // Units
    units: Units::Millimeters,
    
    // Zero suppression
    zeros: ZeroFormat::SuppressLeading,
}
```

### Origin Options

```rust
pub enum GerberOrigin {
    Absolute,       // Design origin (0,0)
    AuxOrigin,      // User-defined aux origin
    GridOrigin,     // Grid origin
    BoardCenter,    // Center of board
    BoardCornerLL,  // Lower-left corner
}
```

### Protel Extensions

Standard Protel-compatible filenames:

| Layer | Extension |
|-------|-----------|
| Top Copper | .GTL |
| Bottom Copper | .GBL |
| Top Mask | .GTS |
| Bottom Mask | .GBS |
| Top Silk | .GTO |
| Bottom Silk | .GBO |
| Drill | .DRL |
| Outline | .GKO |

## Aperture Management

### Aperture Types

```rust
pub enum Aperture {
    Circle { diameter: f64 },
    Rectangle { width: f64, height: f64 },
    Obround { width: f64, height: f64 },
    Polygon { diameter: f64, vertices: u32, rotation: f64 },
    Macro { name: String, params: Vec<f64> },
}
```

### Aperture Optimization

```rust
ApertureConfig {
    // Merge similar apertures
    merge_tolerance: 0.001,  // mm
    
    // Limit aperture count
    max_apertures: 999,
    
    // Use macros for complex shapes
    use_macros: true,
}
```

## Gerber X2 Attributes

### File Attributes

```gerber
%TF.GenerationSoftware,HardwareTool,1.0.0*%
%TF.CreationDate,2026-01-19T16:30:00+00:00*%
%TF.ProjectId,my_board,abc123,1.0*%
%TF.FileFunction,Copper,L1,Top*%
%TF.FilePolarity,Positive*%
```

### Aperture Attributes

```gerber
%TA.AperFunction,ComponentPad*%
D11*
X100000Y100000D03*
%TD*%
```

### Supported Attributes

| Attribute | Description |
|-----------|-------------|
| .FileFunction | Layer type and position |
| .FilePolarity | Positive or negative |
| .Part | Single or array |
| .AperFunction | Pad, conductor, via, etc. |
| .DrillTolerance | Hole tolerance |
| .NetName | Net assignment |

## Output Verification

### Gerber Viewer

```rust
// Built-in Gerber preview
let preview = GerberPreview::load("./gerber")?;
preview.show_layer(Layer::FCu);
preview.overlay_drill();
```

### Export Report

```
Gerber Export Report
════════════════════

Output directory: ./gerber/
Files generated: 12

Layers:
  ✓ my_board-F_Cu.gbr (245 KB)
  ✓ my_board-B_Cu.gbr (198 KB)
  ✓ my_board-F_Mask.gbr (156 KB)
  ✓ my_board-B_Mask.gbr (142 KB)
  ✓ my_board-F_SilkS.gbr (45 KB)
  ✓ my_board-B_SilkS.gbr (12 KB)
  ✓ my_board-Edge_Cuts.gbr (2 KB)

Drill:
  ✓ my_board-PTH.drl (8 KB)
  ✓ my_board-NPTH.drl (1 KB)

Statistics:
  Apertures used: 45
  Drill tools: 5
  Board size: 100.0 x 80.0 mm
```

## Fabrication Notes

### Stackup Information

```rust
// Generate stackup file
StackupExport::new(&pcb)
    .output_path("./gerber/stackup.txt")
    .format(StackupFormat::Text)
    .export()?;
```

### README Generation

```rust
// Generate fabrication readme
FabReadme::new(&pcb)
    .output_path("./gerber/README.txt")
    .include_stackup(true)
    .include_drill_table(true)
    .include_notes(true)
    .export()?;
```

## Related Topics

- [IPC-2581 Export](./ipc2581-export.md)
- [ODB++ Export](./odbpp-export.md)
- [Design Rule Check](../pcb-layout/drc.md)
