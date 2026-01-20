# Design for Manufacturability (DFM) Checks

## Overview

Design for Manufacturability (DFM) analysis identifies potential manufacturing issues before fabrication. Hardware Tool performs comprehensive DFM checks including yield prediction, acute angle detection, solder mask slivers, and other common manufacturing problems.

## DFM Categories

### Copper Features

| Check | Description | Typical Limit |
|-------|-------------|---------------|
| Trace width | Minimum copper width | 0.1mm |
| Trace spacing | Minimum clearance | 0.1mm |
| Annular ring | Copper around holes | 0.1mm |
| Acid traps | Acute angles in copper | >90° |
| Copper slivers | Thin copper features | >0.1mm |
| Starved thermals | Insufficient thermal relief | 4 spokes |

### Drill Features

| Check | Description | Typical Limit |
|-------|-------------|---------------|
| Hole size | Minimum drill diameter | 0.2mm |
| Hole spacing | Minimum hole-to-hole | 0.3mm |
| Aspect ratio | Board thickness / hole | <10:1 |
| Hole to edge | Distance from board edge | 0.5mm |
| Hole to copper | Non-connected copper | 0.2mm |

### Solder Mask

| Check | Description | Typical Limit |
|-------|-------------|---------------|
| Mask slivers | Thin mask between features | >0.1mm |
| Mask dams | Mask between pads | >0.1mm |
| Mask expansion | Mask opening vs pad | 0.05mm |
| Mask bridges | Unintended connections | Check |

### Silkscreen

| Check | Description | Typical Limit |
|-------|-------------|---------------|
| Line width | Minimum silk line | 0.15mm |
| Text height | Minimum readable text | 0.8mm |
| Silk on pads | Silkscreen over copper | Avoid |
| Silk legibility | Text clarity | Check |

## DFM Configuration

### Rule Setup

```rust
DfmConfig {
    // Copper rules
    copper: CopperDfm {
        min_trace_width: 0.1,
        min_trace_spacing: 0.1,
        min_annular_ring: 0.1,
        min_copper_sliver: 0.1,
        max_acute_angle: 90.0,
    },
    
    // Drill rules
    drill: DrillDfm {
        min_hole_size: 0.2,
        min_hole_spacing: 0.3,
        max_aspect_ratio: 10.0,
        min_hole_to_edge: 0.5,
        min_hole_to_copper: 0.2,
    },
    
    // Solder mask rules
    mask: MaskDfm {
        min_mask_sliver: 0.1,
        min_mask_dam: 0.1,
        mask_expansion: 0.05,
    },
    
    // Silkscreen rules
    silk: SilkDfm {
        min_line_width: 0.15,
        min_text_height: 0.8,
        check_silk_on_pads: true,
    },
}
```

### Manufacturer Presets

```rust
// Load manufacturer-specific rules
let dfm = DfmConfig::from_preset(DfmPreset::JlcPcb);
let dfm = DfmConfig::from_preset(DfmPreset::Pcbway);
let dfm = DfmConfig::from_preset(DfmPreset::OshPark);
let dfm = DfmConfig::from_preset(DfmPreset::Advanced);  // Tight tolerances
```

## Acid Trap Detection

### What Are Acid Traps?

Acute angles in copper where etchant can pool:

```
Bad (acid trap):          Good (no trap):
    ╱                         ╱
   ╱                         ╱
  ╱◄── Acute angle          ╱
 ╱     traps acid          ╱
╱_____                    ╱_____
       ╲                        ╲
        ╲                        ╲
         ╲                        ╲
```

### Detection

```rust
let acid_traps = dfm.detect_acid_traps(&pcb, AcidTrapConfig {
    min_angle: 90.0,  // Flag angles less than this
    check_traces: true,
    check_zones: true,
});

for trap in acid_traps {
    println!("Acid trap at {:?}, angle: {:.1}°", 
             trap.location, trap.angle);
}
```

## Solder Mask Analysis

### Sliver Detection

```rust
let slivers = dfm.detect_mask_slivers(&pcb, MaskSliverConfig {
    min_width: 0.1,  // mm
    layers: vec![Layer::FMask, Layer::BMask],
});
```

### Dam Analysis

```
Pad 1        Pad 2
┌────┐      ┌────┐
│    │      │    │
│    │◄────►│    │
│    │ Dam  │    │
└────┘      └────┘

Dam width must be ≥ 0.1mm for reliable mask
```

## Copper Sliver Detection

### Thin Feature Analysis

```rust
let copper_slivers = dfm.detect_copper_slivers(&pcb, CopperSliverConfig {
    min_width: 0.1,
    check_traces: true,
    check_zones: true,
    check_pads: false,
});
```

### Visualization

```
┌─────────────────────────────────┐
│                                 │
│  ████████████████████████████   │
│  ████████████████████████████   │
│  ████████▌◄─ Sliver (0.08mm)    │
│  ████████████████████████████   │
│                                 │
└─────────────────────────────────┘
```

## Yield Prediction

### Yield Analysis

```rust
let yield_report = dfm.predict_yield(&pcb, YieldConfig {
    // Process capability
    process_sigma: 3.0,
    
    // Feature analysis
    analyze_traces: true,
    analyze_vias: true,
    analyze_pads: true,
});

println!("Predicted yield: {:.1}%", yield_report.yield_percent);
println!("Risk areas: {}", yield_report.risk_areas.len());
```

### Yield Report

```
Yield Prediction Report
═══════════════════════

Overall predicted yield: 94.2%

Risk Factors:
  Trace width margin: 98.5%
  Via reliability: 99.1%
  Pad registration: 96.8%
  Solder mask: 97.2%

High-Risk Areas:
  1. BGA U1 fanout (tight spacing)
  2. Fine-pitch connector J3
  3. Impedance-controlled traces

Recommendations:
  - Increase BGA trace spacing by 0.02mm
  - Add teardrops to fine-pitch pads
```

## Running DFM Analysis

### Full Analysis

```rust
let dfm_report = DfmAnalyzer::analyze(&pcb, DfmConfig::default())?;

println!("Errors: {}", dfm_report.errors.len());
println!("Warnings: {}", dfm_report.warnings.len());
println!("Info: {}", dfm_report.info.len());
```

### Selective Analysis

```rust
let dfm_report = DfmAnalyzer::analyze(&pcb, DfmConfig {
    // Only check specific categories
    check_copper: true,
    check_drill: true,
    check_mask: false,
    check_silk: false,
    
    // Region
    region: Some(Rect::new(20.0, 20.0, 80.0, 60.0)),
});
```

## DFM Report

### Report Format

```
═══════════════════════════════════════════════════════════
DFM Analysis Report - my_board.hwt_pcb
Generated: 2026-01-19 16:45:00
Manufacturer: JLCPCB Standard
═══════════════════════════════════════════════════════════

Summary:
  Errors:   3
  Warnings: 12
  Info:     5

───────────────────────────────────────────────────────────
ERRORS
───────────────────────────────────────────────────────────

[DFM-E001] Copper sliver too narrow
  Location: (45.2, 32.1) on F.Cu
  Width: 0.08mm (minimum: 0.1mm)
  Recommendation: Widen trace or adjust zone

[DFM-E002] Acid trap detected
  Location: (60.0, 40.0) on F.Cu
  Angle: 45°
  Recommendation: Add chamfer or adjust routing

[DFM-E003] Solder mask sliver
  Location: (25.0, 55.0) on F.Mask
  Width: 0.06mm (minimum: 0.1mm)
  Recommendation: Merge mask openings

───────────────────────────────────────────────────────────
WARNINGS
───────────────────────────────────────────────────────────

[DFM-W001] Trace width near minimum
  Location: (30.0, 45.0) on F.Cu
  Width: 0.12mm (minimum: 0.1mm, recommended: 0.15mm)
  
[DFM-W002] Small annular ring
  Location: Via at (55.0, 35.0)
  Ring: 0.12mm (minimum: 0.1mm, recommended: 0.15mm)

...
```

### Export Options

```rust
dfm_report.export_html("dfm_report.html")?;
dfm_report.export_pdf("dfm_report.pdf")?;
dfm_report.export_csv("dfm_report.csv")?;
```

## Related Topics

- [Design Rule Check](../pcb-layout/drc.md)
- [Gerber Export](../manufacturing-output/gerber-export.md)
- [Interactive Routing](../pcb-layout/interactive-routing.md)
