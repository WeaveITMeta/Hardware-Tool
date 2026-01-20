# RDL & Interposer Routing

## Overview

Hardware Tool provides specialized routing for Redistribution Layers (RDL) and silicon interposers, supporting high-density interconnects between dies, TSVs, and package substrates. Routing handles micro-bump pitch, signal integrity, and power delivery requirements.

## RDL Configuration

```rust
RDLRouting {
    // Layer stack
    layers: RDLLayerStack {
        count: 4,
        metal_thickness: 2e-6,        // 2 µm
        dielectric: "Polyimide",
        dielectric_thickness: 5e-6,   // 5 µm
    },
    
    // Design rules
    design_rules: RDLDesignRules {
        min_line_width: 2e-6,         // 2 µm
        min_spacing: 2e-6,            // 2 µm
        min_via_size: 3e-6,           // 3 µm
        min_via_pitch: 6e-6,          // 6 µm
    },
    
    // Routing constraints
    constraints: RDLConstraints {
        max_current_density: 2e10,    // A/m² (2 MA/cm²)
        max_ir_drop: 0.05,            // 50 mV
        impedance_control: true,
    },
}
```

## Interposer Routing

```
┌─────────────────────────────────────────────────────────────────┐
│ Interposer Routing: Die-to-Die Connection                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   Die 1 µ-bumps              Die 2 µ-bumps                 │ │
│ │   ● ● ● ● ● ● ● ●            ● ● ● ● ● ● ● ●               │ │
│ │   │ │ │ │ │ │ │ │            │ │ │ │ │ │ │ │               │ │
│ │ ══╪═╪═╪═╪═╪═╪═╪═╪════════════╪═╪═╪═╪═╪═╪═╪═╪══ RDL M4     │ │
│ │   │ │ │ │ │ │ │ │            │ │ │ │ │ │ │ │               │ │
│ │   ○ │ │ │ │ │ │ ○────────────○ │ │ │ │ │ │ ○  Vias        │ │
│ │   │ │ │ │ │ │ │ │            │ │ │ │ │ │ │ │               │ │
│ │ ══╪═╧═╪═╪═╪═╪═╧═╪════════════╪═╧═╪═╪═╪═╪═╧═╪══ RDL M3     │ │
│ │   │   │ │ │ │   │            │   │ │ │ │   │               │ │
│ │   ○   ○ │ │ │   ○────────────○   ○ │ │ │   ○  Vias        │ │
│ │   │   │ │ │ │   │            │   │ │ │ │   │               │ │
│ │ ══╧═══╧═╪═╪═╧═══╧════════════╧═══╧═╪═╪═╧═══╧══ RDL M2     │ │
│ │         │ │                        │ │                      │ │
│ │         ◯ ◯                        ◯ ◯       TSVs          │ │
│ │         │ │                        │ │                      │ │
│ │ ════════╪═╪════════════════════════╪═╪════════ RDL M1     │ │
│ │         │ │                        │ │                      │ │
│ │         ● ●                        ● ●       C4 Bumps      │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Routing Statistics:                                             │
│   Nets routed: 2,048 / 2,048 (100%)                           │
│   Total wire length: 1.2 m                                     │
│   Via count: 8,192                                             │
│   Max current density: 1.8 MA/cm² ✓                           │
│                                                                 │
│ [Auto-Route] [Check DRC] [SI Analysis] [Export]                 │
└─────────────────────────────────────────────────────────────────┘
```

## TSV Integration

```rust
TSVRouting {
    // TSV properties
    tsv: TSVProperties {
        diameter: 10e-6,              // 10 µm
        pitch: 50e-6,                 // 50 µm
        depth: 100e-6,                // 100 µm (interposer thickness)
        resistance: 0.05,             // Ω
        capacitance: 50e-15,          // 50 fF
    },
    
    // Keep-out zones
    keep_out: TSVKeepOut {
        radius: 25e-6,                // Around each TSV
        no_active: true,              // No active devices
    },
    
    // Routing to TSV
    routing: TSVRoutingConfig {
        landing_pad_size: 15e-6,
        via_stack: true,              // Stack vias to TSV
    },
}
```

## Rust API

```rust
// Create interposer routing
let interposer = project.get_interposer()?;

// Route die-to-die connections
interposer.route_die_to_die("compute", "hbm1", RoutingConfig {
    layer_preference: vec!["RDL_M4", "RDL_M3"],
    use_tsv: true,
    impedance_control: true,
})?;

// Route power distribution
interposer.route_power("VDD", PowerRoutingConfig {
    width: 20e-6,
    max_ir_drop: 0.05,
    via_array: true,
})?;

// Check routing
let drc = interposer.run_drc()?;
let si = interposer.analyze_si()?;

// Export
interposer.export_gdsii("interposer.gds")?;
```

## Related Topics

- [Die Placement & Stacking](./die-placement-and-stacking.md)
- [TSV & Bump Stitching](./tsv-and-bump-stitching.md)
- [Packaging Design Rule Check](./packaging-design-rule-check.md)
