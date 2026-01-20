# Fabrication & Assembly Drawings

## Overview

Fabrication and assembly drawings provide visual documentation for PCB manufacturing and assembly. Hardware Tool generates professional drawing sheets with title blocks, dimensions, notes, and layer views.

## Drawing Types

### Fabrication Drawing

Manufacturing specifications for the bare PCB:

- Board dimensions and tolerances
- Layer stackup
- Drill chart
- Material specifications
- Surface finish requirements
- Impedance requirements

### Assembly Drawing

Component placement guide for assembly:

- Component locations
- Reference designators
- Orientation indicators
- Special instructions
- Polarity markings

## Drawing Sheet Setup

### Page Configuration

```rust
DrawingSheet {
    // Size
    size: PaperSize::A3,  // or A4, Letter, Custom
    orientation: Orientation::Landscape,
    
    // Margins
    margins: Margins {
        left: 20.0,
        right: 10.0,
        top: 10.0,
        bottom: 10.0,
    },
    
    // Scale
    scale: DrawingScale::FitToPage,
    // or Fixed(1.0), Custom(2.0)
}
```

### Paper Sizes

| Size | Dimensions (mm) |
|------|-----------------|
| A4 | 210 × 297 |
| A3 | 297 × 420 |
| A2 | 420 × 594 |
| A1 | 594 × 841 |
| Letter | 216 × 279 |
| Legal | 216 × 356 |
| Tabloid | 279 × 432 |

## Title Block

### Standard Title Block

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│                    [DRAWING AREA]                       │
│                                                         │
├─────────────────────────────────────────────────────────┤
│ Company: Acme Electronics    │ Title: Main Board        │
│ Project: Widget v2           │ Drawing: Fabrication     │
├──────────────────────────────┼──────────────────────────┤
│ Drawn: J.Smith  2026-01-15   │ Rev: A                   │
│ Checked: M.Jones 2026-01-18  │ Sheet: 1 of 3            │
├──────────────────────────────┼──────────────────────────┤
│ Scale: 1:1                   │ Units: mm                │
└─────────────────────────────────────────────────────────┘
```

### Title Block Configuration

```rust
TitleBlock {
    // Company info
    company: "Acme Electronics",
    logo: Some("logo.png"),
    
    // Project info
    project: "Widget v2",
    title: "Main Board",
    drawing_type: "Fabrication",
    
    // Revision
    revision: "A",
    date: "2026-01-19",
    
    // Personnel
    drawn_by: "J. Smith",
    checked_by: "M. Jones",
    approved_by: Some("Director"),
    
    // Sheet info
    sheet_number: 1,
    total_sheets: 3,
    
    // Drawing info
    scale: "1:1",
    units: "mm",
}
```

## Fabrication Drawing Content

### Board Outline View

```rust
FabDrawing::new(&pcb)
    .add_view(BoardOutlineView {
        position: Point::new(50.0, 150.0),
        scale: 1.0,
        
        // Dimensions
        show_dimensions: true,
        dimension_style: DimensionStyle::Standard,
        
        // Features
        show_mounting_holes: true,
        show_cutouts: true,
        show_slots: true,
    });
```

### Drill Chart

```
┌──────────────────────────────────────────┐
│              DRILL CHART                 │
├──────┬──────────┬───────┬────────┬───────┤
│ Tool │ Diameter │ Count │ Plated │ Tol   │
├──────┼──────────┼───────┼────────┼───────┤
│  T1  │  0.30mm  │  145  │  Yes   │ ±0.05 │
│  T2  │  0.80mm  │   23  │  Yes   │ ±0.05 │
│  T3  │  1.00mm  │   12  │  Yes   │ ±0.08 │
│  T4  │  3.20mm  │    4  │  No    │ ±0.10 │
└──────┴──────────┴───────┴────────┴───────┘
```

```rust
FabDrawing::new(&pcb)
    .add_view(DrillChart {
        position: Point::new(200.0, 50.0),
        
        // Content
        show_tool_number: true,
        show_diameter: true,
        show_count: true,
        show_plated: true,
        show_tolerance: true,
        
        // Sorting
        sort_by: DrillSort::Diameter,
    });
```

### Layer Stackup

```
┌─────────────────────────────────────┐
│          LAYER STACKUP              │
├─────────────────────────────────────┤
│  ══════════════════  F.Cu (35µm)    │
│  ░░░░░░░░░░░░░░░░░░  Prepreg (0.2mm)│
│  ══════════════════  In1.Cu (35µm)  │
│  ████████████████████ Core (1.0mm)  │
│  ══════════════════  In2.Cu (35µm)  │
│  ░░░░░░░░░░░░░░░░░░  Prepreg (0.2mm)│
│  ══════════════════  B.Cu (35µm)    │
├─────────────────────────────────────┤
│  Total thickness: 1.6mm ± 10%       │
└─────────────────────────────────────┘
```

### Fabrication Notes

```rust
FabDrawing::new(&pcb)
    .add_notes(FabNotes {
        position: Point::new(200.0, 150.0),
        
        notes: vec![
            "1. Material: FR-4 TG170",
            "2. Copper weight: 1oz (35µm)",
            "3. Surface finish: ENIG",
            "4. Solder mask: Green LPI",
            "5. Silkscreen: White",
            "6. Min trace/space: 0.15/0.15mm",
            "7. Min hole: 0.3mm",
            "8. Impedance: 50Ω ±10% (controlled)",
            "9. IPC Class: 2",
        ],
    });
```

## Assembly Drawing Content

### Component Placement View

```rust
AssemblyDrawing::new(&pcb)
    .add_view(PlacementView {
        position: Point::new(50.0, 150.0),
        scale: 1.0,
        side: BoardSide::Top,
        
        // Display options
        show_references: true,
        show_outlines: true,
        show_polarity: true,
        show_pin1: true,
        
        // Filtering
        highlight_polarized: true,
        dim_unpopulated: true,
    });
```

### Component Table

```
┌────────────────────────────────────────────────────┐
│              COMPONENT LIST (TOP)                  │
├──────┬──────────┬────────────┬─────────────────────┤
│ Ref  │ Value    │ Footprint  │ Notes               │
├──────┼──────────┼────────────┼─────────────────────┤
│ U1   │ STM32F4  │ LQFP-64    │ Pin 1 at corner     │
│ U2   │ LM1117   │ SOT-223    │ Tab is GND          │
│ D1   │ LED      │ 0603       │ Cathode marked      │
│ Q1   │ 2N7002   │ SOT-23     │ Gate at pin 1       │
└──────┴──────────┴────────────┴─────────────────────┘
```

### Assembly Notes

```rust
AssemblyDrawing::new(&pcb)
    .add_notes(AssemblyNotes {
        position: Point::new(200.0, 50.0),
        
        notes: vec![
            "1. All SMD components on top side unless noted",
            "2. Observe polarity for diodes and capacitors",
            "3. U1: Align pin 1 dot with PCB marking",
            "4. Apply thermal paste under U3",
            "5. DNP: R15, R16, C20",
            "6. Solder J1 last after mechanical fit check",
        ],
    });
```

## Dimensions

### Dimension Styles

```rust
pub enum DimensionStyle {
    Linear,         // Straight line dimension
    Aligned,        // Aligned with feature
    Angular,        // Angle measurement
    Radial,         // Radius dimension
    Diameter,       // Diameter dimension
    Leader,         // Leader with note
}
```

### Adding Dimensions

```rust
drawing.add_dimension(Dimension::linear(
    Point::new(0.0, 0.0),
    Point::new(100.0, 0.0),
    DimensionConfig {
        offset: 10.0,
        text_height: 2.5,
        arrow_size: 2.0,
        precision: 2,
        units: "mm",
    }
));
```

## Export Formats

### PDF Export

```rust
DrawingExport::pdf(&drawing)
    .output_path("fabrication_drawing.pdf")
    .color_mode(ColorMode::Color)
    .resolution(300)  // DPI
    .export()?;
```

### DXF Export

```rust
DrawingExport::dxf(&drawing)
    .output_path("fabrication_drawing.dxf")
    .version(DxfVersion::R2018)
    .export()?;
```

### SVG Export

```rust
DrawingExport::svg(&drawing)
    .output_path("fabrication_drawing.svg")
    .embed_fonts(true)
    .export()?;
```

## Drawing Templates

### Custom Templates

```rust
let template = DrawingTemplate::load("company_template.hwt_dwg")?;

FabDrawing::new(&pcb)
    .template(template)
    .export("drawing.pdf")?;
```

### Template Elements

- Title block layout
- Company logo
- Standard notes
- Revision table
- Approval signatures

## Related Topics

- [Gerber Export](./gerber-export.md)
- [BOM Generation](./bom-pick-place.md)
- [STEP Export](../3d-visualization/step-3d-models.md)
