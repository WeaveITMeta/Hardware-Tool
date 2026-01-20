# Design Rule Check (DRC)

## Overview

Design Rule Check (DRC) validates that the PCB layout meets manufacturing constraints and design requirements. Hardware Tool performs comprehensive checks for clearance, connectivity, size, and other critical parameters.

## DRC Categories

### Clearance Violations

Minimum spacing between copper features:

```rust
ClearanceRules {
    // Global defaults
    track_to_track: 0.15,
    track_to_pad: 0.15,
    track_to_via: 0.15,
    pad_to_pad: 0.2,
    via_to_via: 0.2,
    
    // Zone clearances
    zone_to_track: 0.2,
    zone_to_pad: 0.25,
    zone_to_zone: 0.3,
    
    // Board edge
    copper_to_edge: 0.25,
}
```

### Connection Violations

```rust
ConnectionRules {
    // Unconnected items
    check_unconnected_pads: true,
    check_unconnected_vias: true,
    
    // Broken nets
    check_broken_nets: true,
    
    // Stub traces
    min_stub_length: 0.5,
}
```

### Size Violations

```rust
SizeRules {
    // Track width
    min_track_width: 0.1,
    
    // Via dimensions
    min_via_diameter: 0.4,
    min_via_drill: 0.2,
    min_via_annular_ring: 0.1,
    
    // Microvia
    min_microvia_diameter: 0.2,
    min_microvia_drill: 0.1,
    
    // Hole sizes
    min_hole_diameter: 0.2,
    max_hole_diameter: 6.0,
}
```

## Rule Configuration

### Design Rules File

```toml
[design_rules]
version = "1.0"

[clearance]
default = 0.15

[clearance.net_class.power]
track_to_track = 0.3
track_to_pad = 0.3

[clearance.net.VCC]
to_any = 0.4

[size]
min_track_width = 0.1
min_via_diameter = 0.4
min_via_drill = 0.2

[size.net_class.high_speed]
min_track_width = 0.12
max_track_width = 0.15
```

### Net Class Rules

```rust
NetClassRules::new("differential")
    .track_width(0.1, 0.15)  // min, max
    .clearance(0.2)
    .via_diameter(0.5)
    .via_drill(0.25)
    .diff_pair_gap(0.1, 0.15);
```

### Area-Specific Rules

```rust
AreaRule::new("bga_region")
    .region(Rect::new(20.0, 20.0, 60.0, 60.0))
    .min_track_width(0.075)
    .min_clearance(0.075)
    .min_via_drill(0.15);
```

## DRC Checks

### Copper Checks

| Check | Description |
|-------|-------------|
| **Track clearance** | Spacing between traces |
| **Pad clearance** | Spacing around pads |
| **Via clearance** | Spacing around vias |
| **Zone clearance** | Spacing in/around zones |
| **Annular ring** | Copper around drill holes |
| **Track width** | Minimum/maximum width |

### Drill Checks

| Check | Description |
|-------|-------------|
| **Hole size** | Min/max drill diameter |
| **Hole spacing** | Distance between holes |
| **Hole to edge** | Distance from board edge |
| **Via drill** | Via hole requirements |

### Zone Checks

| Check | Description |
|-------|-------------|
| **Zone isolation** | Clearance to other nets |
| **Min width** | Minimum copper width in zone |
| **Islands** | Unconnected copper islands |
| **Thermal relief** | Proper thermal connections |

### Courtyard Checks

| Check | Description |
|-------|-------------|
| **Overlap** | Components overlapping |
| **Clearance** | Minimum spacing |
| **Missing** | Components without courtyard |

## Running DRC

### Full Check

```rust
let report = pcb.run_drc(DrcConfig::default())?;

println!("Errors: {}", report.errors.len());
println!("Warnings: {}", report.warnings.len());
```

### Selective Check

```rust
let report = pcb.run_drc(DrcConfig {
    check_clearance: true,
    check_connectivity: true,
    check_size: false,  // Skip size checks
    check_courtyard: true,
    
    // Scope
    layers: Some(vec![Layer::FCu, Layer::BCu]),
    region: Some(Rect::new(0.0, 0.0, 50.0, 50.0)),
});
```

### Incremental Check

```rust
// Check only modified items
let report = pcb.run_incremental_drc(&modified_items);
```

## DRC Report

### Report Format

```
═══════════════════════════════════════════════════════════
DRC Report - my_board.hwt_pcb
Generated: 2026-01-19 16:30:00
═══════════════════════════════════════════════════════════

Summary:
  Errors:   5
  Warnings: 12
  Excluded: 2

───────────────────────────────────────────────────────────
ERRORS (5)
───────────────────────────────────────────────────────────

[E001] Clearance violation: track to track
  Location: (45.2, 32.1) on F.Cu
  Net 1: DATA_0
  Net 2: DATA_1
  Actual: 0.12mm
  Required: 0.15mm

[E002] Unconnected pad
  Component: U1 (STM32F4)
  Pad: 45 (PA10)
  Net: UART_RX
  
[E003] Via annular ring too small
  Location: (60.0, 40.0)
  Net: VCC
  Actual: 0.08mm
  Required: 0.10mm

[E004] Track width below minimum
  Location: (25.0, 55.0) to (30.0, 55.0) on F.Cu
  Net: CLK
  Actual: 0.08mm
  Required: 0.10mm

[E005] Courtyard overlap
  Components: U1 and C5
  Overlap area: 0.5mm²

───────────────────────────────────────────────────────────
WARNINGS (12)
───────────────────────────────────────────────────────────

[W001] Silk over pad
  Component: R1
  Pad: 1
  
[W002] Track near board edge
  Location: (1.2, 30.0) on F.Cu
  Distance: 0.2mm (recommended: 0.25mm)
  
...
```

### Report Export

```rust
// Export formats
report.export_json("drc_report.json")?;
report.export_csv("drc_report.csv")?;
report.export_html("drc_report.html")?;
```

## DRC Markers

### Visual Indicators

```
┌─────────────────────────────────────┐
│                                     │
│    ●────────────────●               │
│         ⚠ 0.12mm                    │  ← Clearance warning
│    ●────────────────●               │
│                                     │
│         ✗                           │  ← Error marker
│    ●────╳────●                      │  ← Unconnected
│                                     │
└─────────────────────────────────────┘
```

### Marker Navigation

| Action | Shortcut |
|--------|----------|
| Next error | `N` |
| Previous error | `P` |
| Jump to error | Click in report |
| Clear markers | `Ctrl+Shift+C` |

## Exclusions

### Excluding Violations

```rust
// Exclude specific violation
pcb.drc_exclude(DrcExclusion {
    type_: "clearance",
    location: Point::new(45.2, 32.1),
    layer: Layer::FCu,
    reason: "Intentional tight spacing for impedance matching",
});

// Exclude by rule
pcb.drc_exclude_rule("silk_over_pad", "R*");
```

### Exclusion Management

```
DRC Exclusions
══════════════

1. Clearance at (45.2, 32.1) - F.Cu
   Reason: Impedance matching
   Created: 2026-01-15
   
2. Courtyard overlap: U1/C5
   Reason: Verified mechanical fit
   Created: 2026-01-18

[Add] [Remove] [Edit] [Clear All]
```

## Severity Levels

```rust
pub enum DrcSeverity {
    Error,      // Must fix
    Warning,    // Should review
    Info,       // Informational
    Ignore,     // Suppressed
}
```

### Configuring Severity

```rust
DrcSeverityConfig {
    clearance_violation: Severity::Error,
    silk_over_pad: Severity::Warning,
    missing_courtyard: Severity::Info,
    track_near_edge: Severity::Warning,
}
```

## Real-Time DRC

### Interactive Checking

```rust
RealtimeDrc {
    enabled: true,
    
    // Check during routing
    check_while_routing: true,
    
    // Visual feedback
    highlight_violations: true,
    block_violations: false,  // Allow but warn
}
```

### Performance Settings

```rust
DrcPerformance {
    // Incremental updates
    incremental: true,
    
    // Threading
    parallel: true,
    threads: 4,
    
    // Debounce
    delay_ms: 100,
}
```

## Related Topics

- [Interactive Routing](./interactive-routing.md)
- [Copper Zones](./copper-zones.md)
- [Manufacturing Output](../manufacturing-output/gerber-export.md)
