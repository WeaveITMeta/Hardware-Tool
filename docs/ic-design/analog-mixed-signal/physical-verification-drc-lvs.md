# Physical Verification (DRC/LVS)

## Overview

Hardware Tool provides comprehensive physical verification for IC designs, including Design Rule Check (DRC), Layout vs Schematic (LVS), and Electrical Rule Check (ERC). Verification integrates with PDK design rules and supports both interactive and batch modes.

## Design Rule Check (DRC)

```rust
DRCEngine {
    // Rule categories
    categories: vec![
        DRCCategory::Width,           // Minimum width
        DRCCategory::Spacing,         // Minimum spacing
        DRCCategory::Enclosure,       // Enclosure rules
        DRCCategory::Extension,       // Extension rules
        DRCCategory::Area,            // Minimum area
        DRCCategory::Density,         // Metal density
        DRCCategory::Antenna,         // Antenna rules
        DRCCategory::OffGrid,         // Grid alignment
    ],
    
    // Execution mode
    mode: DRCMode {
        incremental: true,            // Check only modified regions
        hierarchical: true,           // Preserve hierarchy
        parallel: true,               // Multi-threaded
    },
    
    // Reporting
    reporting: DRCReporting {
        max_errors: 10000,
        group_by_rule: true,
        include_coordinates: true,
        generate_markers: true,
    },
}
```

## DRC Results UI

```
┌─────────────────────────────────────────────────────────────────┐
│ DRC Results: opamp_layout                                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: 3 errors, 2 warnings                                   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Rule          │ Count │ Severity │ Description              │ │
│ │ ──────────────┼───────┼──────────┼───────────────────────── │ │
│ │ met1.5        │   1   │ Error    │ Metal1 spacing < 0.14µm  │ │
│ │ poly.2        │   1   │ Error    │ Poly width < 0.15µm      │ │
│ │ antenna.1     │   1   │ Error    │ Gate area ratio > 400    │ │
│ │ density.1     │   1   │ Warning  │ Metal1 density < 28%     │ │
│ │ offgrid.1     │   1   │ Warning  │ Vertex off 5nm grid      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Error Details:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ met1.5: Metal1 spacing violation                         │ │
│ │   Location: (23.450, 12.670) - (23.580, 12.670)            │ │
│ │   Actual: 0.130 µm                                         │ │
│ │   Required: 0.140 µm                                       │ │
│ │   [Zoom To] [Fix] [Waive]                                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix All] [Export Report] [Re-run DRC] [Close]                  │
└─────────────────────────────────────────────────────────────────┘
```

## Layout vs Schematic (LVS)

```rust
LVSEngine {
    // Extraction
    extraction: LVSExtraction {
        extract_devices: true,
        extract_resistors: true,
        extract_capacitors: false,    // For LVS only
        merge_parallel: true,
        merge_series: true,
    },
    
    // Comparison
    comparison: LVSComparison {
        match_by: MatchMethod::Name,  // or Topology
        device_tolerance: DeviceTolerance {
            width: 0.01,              // 1%
            length: 0.01,
            multiplier: 0,            // Exact
        },
        ignore_bulk: false,
        ignore_dummy: true,
    },
    
    // Reporting
    reporting: LVSReporting {
        generate_extracted_netlist: true,
        highlight_mismatches: true,
        cross_reference: true,
    },
}
```

## LVS Results UI

```
┌─────────────────────────────────────────────────────────────────┐
│ LVS Results: opamp                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ✓ LVS CLEAN                                                    │
│                                                                 │
│ Device Comparison:                                              │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Device    │ Schematic │ Layout │ Matched │ Errors           │ │
│ │ ──────────┼───────────┼────────┼─────────┼─────────         │ │
│ │ NMOS      │     5     │    5   │    5    │   0              │ │
│ │ PMOS      │     4     │    4   │    4    │   0              │ │
│ │ Resistor  │     2     │    2   │    2    │   0              │ │
│ │ Capacitor │     1     │    1   │    1    │   0              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Net Comparison:                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Category      │ Schematic │ Layout │ Matched               │ │
│ │ ──────────────┼───────────┼────────┼─────────              │ │
│ │ Total Nets    │    12     │   12   │   12                  │ │
│ │ Power Nets    │     2     │    2   │    2                  │ │
│ │ Signal Nets   │    10     │   10   │   10                  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View Netlist] [Cross-Probe] [Export] [Close]                   │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Run DRC
let layout = project.get_layout("opamp")?;
let drc_result = layout.run_drc(DRCConfig::from_pdk(&pdk))?;

if !drc_result.is_clean() {
    for error in drc_result.errors() {
        println!("{}: {} at {:?}", error.rule, error.message, error.location);
    }
}

// Run LVS
let schematic = project.get_schematic("opamp")?;
let lvs_result = layout.run_lvs(&schematic, LVSConfig::default())?;

if lvs_result.is_clean() {
    println!("LVS clean!");
} else {
    for mismatch in lvs_result.mismatches() {
        println!("Mismatch: {:?}", mismatch);
    }
}
```

## Related Topics

- [Guard Rings & Substrate Noise](./guard-rings-and-substrate-noise.md)
- [Signal Integrity Analysis](./signal-integrity-analysis.md)
