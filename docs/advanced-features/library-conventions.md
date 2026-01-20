# Library Conventions & Quality Control

## Overview

Hardware Tool follows KLC-style (KiCAD Library Convention) guidelines for symbols and footprints to ensure consistency, quality, and interoperability. This document defines the standards for creating and maintaining component libraries.

## Symbol Conventions

### General Rules

1. **Origin**: Symbol origin at center of component body
2. **Grid**: All elements on 50mil (1.27mm) grid
3. **Pin length**: Standard 100mil (2.54mm)
4. **Text size**: Reference 50mil, Value 50mil
5. **Line width**: 10mil (0.254mm) for body outline

### Pin Placement

```
         INPUTS                    OUTPUTS
            │                         │
            ▼                         ▼
    ┌───────────────────────────────────┐
    │                                   │
  ──┤ Pin 1                      Pin 8 ├──
    │                                   │
  ──┤ Pin 2                      Pin 7 ├──
    │                                   │
  ──┤ Pin 3                      Pin 6 ├──
    │                                   │
  ──┤ Pin 4                      Pin 5 ├──
    │                                   │
    └───────────────────────────────────┘
            ▲                         ▲
            │                         │
         ACTIVE LOW              ACTIVE LOW
         (with bar)              (optional)
```

### Pin Placement Rules

| Pin Type | Placement |
|----------|-----------|
| Inputs | Left side |
| Outputs | Right side |
| Bidirectional | Either side (consistent) |
| Power (VCC) | Top |
| Ground (GND) | Bottom |
| No Connect | Any (marked NC) |

### Naming Conventions

```rust
SymbolNaming {
    // Format: Manufacturer_Family_Variant_Package
    // Example: STM_STM32F4_05RGT6_LQFP64
    
    manufacturer: "STM",
    family: "STM32F4",
    variant: "05RGT6",
    package: "LQFP64",
}
```

### Symbol Validation

```rust
let validation = SymbolValidator::validate(&symbol);

// Checks performed:
// - Pin grid alignment
// - Pin length consistency
// - Text positioning
// - Electrical type assignments
// - Duplicate pin numbers
// - Missing properties
```

## Footprint Conventions

### General Rules

1. **Origin**: At component center (or pin 1 for connectors)
2. **Orientation**: Pin 1 at top-left or left
3. **Grid**: Pads on 0.05mm grid minimum
4. **Courtyard**: 0.25mm clearance from pads/body
5. **Silkscreen**: Outside pads, 0.12mm line width

### Layer Usage

| Layer | Content |
|-------|---------|
| F.Cu | Copper pads |
| F.Paste | Paste apertures |
| F.Mask | Mask openings |
| F.SilkS | Component outline, pin 1 marker |
| F.Fab | Fabrication outline, reference |
| F.CrtYd | Courtyard boundary |

### Pad Naming

```rust
PadNaming {
    // Through-hole: 1, 2, 3, ...
    // SMD: 1, 2, 3, ... (same as schematic)
    // BGA: A1, A2, B1, B2, ...
    // Thermal/mounting: MP, TH, or numbered
}
```

### Courtyard Rules

```rust
CourtyardRules {
    // Minimum clearance from component body
    clearance: 0.25,  // mm
    
    // Grid
    grid: 0.05,  // mm
    
    // Line width
    line_width: 0.05,  // mm
    
    // Must be closed polygon
    closed: true,
}
```

### Silkscreen Rules

```rust
SilkscreenRules {
    // Line width
    line_width: 0.12,  // mm
    
    // Clearance from pads
    pad_clearance: 0.2,  // mm
    
    // Pin 1 indicator required
    pin1_indicator: true,
    
    // Reference designator
    reference_visible: true,
    reference_size: 1.0,  // mm
}
```

## Footprint Validation

### Automated Checks

```rust
let validation = FootprintValidator::validate(&footprint);

// Checks:
validation.check_pad_overlap();
validation.check_courtyard_clearance();
validation.check_silkscreen_overlap();
validation.check_fab_layer();
validation.check_3d_model();
validation.check_documentation();
```

### Validation Report

```
Footprint Validation: R_0603
════════════════════════════

✓ Pad geometry correct
✓ Courtyard present and valid
✓ Silkscreen clear of pads
✓ Pin 1 indicator present
✓ Fabrication layer present
⚠ 3D model missing
✓ Properties complete

Result: PASS (1 warning)
```

## Library Organization

### Directory Structure

```
libraries/
├── symbols/
│   ├── Device.hwt_lib/
│   │   ├── library.toml
│   │   └── symbols/
│   │       ├── R.hwt_sym
│   │       ├── C.hwt_sym
│   │       └── ...
│   ├── MCU_ST.hwt_lib/
│   └── ...
├── footprints/
│   ├── Package_SMD.hwt_fplib/
│   │   ├── library.toml
│   │   └── footprints/
│   │       ├── R_0402.hwt_fp
│   │       ├── R_0603.hwt_fp
│   │       └── ...
│   └── ...
└── 3dmodels/
    └── ...
```

### Library Metadata

```toml
# library.toml
[library]
name = "Device"
version = "1.0.0"
description = "Basic electronic components"
author = "Hardware Tool Team"
license = "CC-BY-SA-4.0"

[compatibility]
min_version = "0.1.0"

[categories]
symbols = ["passive", "discrete"]
```

## Quality Control

### Review Checklist

#### Symbols

- [ ] Origin centered
- [ ] Pins on grid
- [ ] Pin types correct
- [ ] Reference/value positioned
- [ ] Description filled
- [ ] Keywords added
- [ ] Footprint filters set
- [ ] Datasheet linked

#### Footprints

- [ ] Origin correct
- [ ] Pads match datasheet
- [ ] Courtyard present
- [ ] Silkscreen clear
- [ ] Pin 1 marked
- [ ] 3D model assigned
- [ ] Properties complete

### Batch Validation

```rust
// Validate entire library
let report = LibraryValidator::validate_all("./libraries")?;

println!("Symbols: {} passed, {} failed", 
         report.symbols_passed, 
         report.symbols_failed);
println!("Footprints: {} passed, {} failed",
         report.footprints_passed,
         report.footprints_failed);
```

## Contributing Guidelines

### Submission Process

1. Fork library repository
2. Create feature branch
3. Add/modify components
4. Run validation
5. Submit pull request
6. Review and merge

### Style Guide

```rust
StyleGuide {
    // Naming
    use_manufacturer_names: true,
    use_standard_prefixes: true,
    
    // Documentation
    require_description: true,
    require_datasheet: true,
    require_keywords: true,
    
    // Quality
    require_3d_model: false,  // Recommended
    require_simulation_model: false,
}
```

## Related Topics

- [Symbols & Libraries](../schematic-editor/symbols-libraries.md)
- [Footprints & Libraries](../pcb-layout/footprints-libraries.md)
- [Programmatic Design](../core-architecture/programmatic-design.md)
