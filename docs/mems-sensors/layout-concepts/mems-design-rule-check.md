# MEMS Design Rule Check

## Overview

Hardware Tool provides comprehensive MEMS-specific design rule checking that validates mechanical structures, etch patterns, and process constraints. DRC rules are foundry-specific and cover release hole spacing, anchor dimensions, and structural integrity.

## MEMS DRC Categories

```rust
MEMSDRCRules {
    // Structural rules
    structural: StructuralRules {
        min_beam_width: 2e-6,         // 2 µm
        min_beam_length: 10e-6,       // 10 µm
        max_aspect_ratio: 50.0,       // Length/width
        min_anchor_size: 4e-6,        // 4 µm
    },
    
    // Etch hole rules
    etch_holes: EtchHoleRules {
        max_spacing: 30e-6,           // Maximum hole spacing
        min_size: 3e-6,               // Minimum hole size
        min_spacing: 5e-6,            // Minimum between holes
        edge_clearance: 5e-6,         // From structure edge
    },
    
    // Release rules
    release: ReleaseRules {
        max_undercut: 50e-6,          // Maximum etch distance
        min_gap: 2e-6,                // Minimum air gap
        anti_stiction: true,          // Require dimples
    },
    
    // Layer-specific
    layer_rules: vec![
        LayerRule { layer: "Poly1", min_width: 2e-6, min_space: 2e-6 },
        LayerRule { layer: "Poly2", min_width: 2e-6, min_space: 2e-6 },
        LayerRule { layer: "Metal", min_width: 3e-6, min_space: 3e-6 },
    ],
}
```

## DRC Results UI

```
┌─────────────────────────────────────────────────────────────────┐
│ MEMS DRC Results: accelerometer                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Summary: 2 errors, 3 warnings                                   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Rule              │ Count │ Severity │ Description          │ │
│ │ ──────────────────┼───────┼──────────┼───────────────────── │ │
│ │ etch_hole.1       │   1   │ Error    │ Hole spacing > 30µm  │ │
│ │ beam.2            │   1   │ Error    │ Beam width < 2µm     │ │
│ │ release.1         │   1   │ Warning  │ Undercut > 40µm      │ │
│ │ anchor.1          │   1   │ Warning  │ Anchor near edge     │ │
│ │ stiction.1        │   1   │ Warning  │ No dimples in gap    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Error Details:                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ✗ etch_hole.1: Etch hole spacing exceeds maximum           │ │
│ │   Location: Proof mass region (100, 200) - (150, 250)      │ │
│ │   Actual: 45 µm                                            │ │
│ │   Maximum: 30 µm                                           │ │
│ │   Impact: Incomplete release, stiction risk                │ │
│ │   [Zoom To] [Add Holes] [Waive]                            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Fix All] [Export Report] [Re-run DRC] [Close]                  │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Run MEMS DRC
let layout = project.get_layout("accelerometer")?;
let drc_result = layout.run_mems_drc(MEMSDRCConfig {
    foundry: Foundry::MEMSCAP_PolyMUMPs,
    check_etch_holes: true,
    check_release: true,
    check_stiction: true,
})?;

if !drc_result.is_clean() {
    for error in drc_result.errors() {
        println!("{}: {}", error.rule, error.message);
    }
}

// Auto-fix etch holes
layout.auto_add_etch_holes(EtchHoleConfig {
    max_spacing: 25e-6,
    hole_size: 4e-6,
})?;
```

## Related Topics

- [Etch Hole Routing](./etch-hole-routing.md)
- [Release Zones](./release-zones.md)
- [Modal Analysis Highlighting](./modal-analysis-highlighting.md)
