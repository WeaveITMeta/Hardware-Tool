# Microstrip & Coplanar Routing

## Overview

Hardware Tool provides specialized routing for RF transmission lines including microstrip, coplanar waveguide (CPW), stripline, and coplanar stripline. Routing maintains controlled impedance and minimizes discontinuities.

## Transmission Line Types

```rust
TransmissionLines {
    // Microstrip
    microstrip: MicrostripConfig {
        impedance: 50.0,              // Ω
        width: 3.0e-3,                // Calculated from substrate
        substrate: Substrate {
            er: 4.4,
            height: 1.6e-3,
            loss_tangent: 0.02,
        },
    },
    
    // Coplanar Waveguide
    cpw: CPWConfig {
        impedance: 50.0,
        center_width: 1.5e-3,
        gap: 0.3e-3,
        ground_width: 5.0e-3,
        with_ground_plane: true,      // CPWG
    },
    
    // Stripline
    stripline: StriplineConfig {
        impedance: 50.0,
        width: 0.5e-3,
        substrate_height: 0.8e-3,     // Each side
    },
}
```

## Routing Editor

```
┌─────────────────────────────────────────────────────────────────┐
│ RF Routing: 50Ω Microstrip                                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Line Type: [Microstrip ▼]  Impedance: [50    ] Ω               │
│                                                                 │
│ Substrate: Rogers RO4003C                                       │
│   εr = 3.55, h = 0.508 mm, tan δ = 0.0027                      │
│                                                                 │
│ Calculated Dimensions:                                          │
│   Width: 1.12 mm (for 50Ω)                                     │
│   Effective εr: 2.68                                           │
│   λ/4 @ 2.4 GHz: 19.1 mm                                       │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │   ┌─────┐                                      ┌─────┐     │ │
│ │   │ U1  │══════════════════════════════════════│ U2  │     │ │
│ │   │ LNA │        50Ω Microstrip                │ PA  │     │ │
│ │   └─────┘                                      └─────┘     │ │
│ │                                                             │ │
│ │   ════════════════════════════════════════════════════     │ │
│ │                    Ground Plane                            │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Routing Options:                                                │
│ ☑ Maintain constant width                                      │
│ ☑ Mitered corners (optimal angle)                              │
│ ☑ Taper at component pads                                      │
│ ☐ Add via fences                                               │
│                                                                 │
│ [Route] [Check Impedance] [EM Simulate] [Close]                 │
└─────────────────────────────────────────────────────────────────┘
```

## Corner Mitigation

```rust
CornerMitigation {
    // Corner types
    types: vec![
        Corner::Mitered {
            miter_ratio: 0.6,         // Optimal for microstrip
        },
        Corner::Curved {
            radius: 3.0,              // × line width
        },
        Corner::Chamfered {
            chamfer: 0.5,             // × line width
        },
    ],
    
    // Auto-mitigation
    auto_miter: AutoMiter {
        enabled: true,
        min_angle: 45.0,              // Degrees
        optimize_for: OptimizeFor::ReturnLoss,
    },
}
```

## Rust API

```rust
// Create RF route
let route = project.create_rf_route("rf_signal")?;

// Set transmission line type
route.set_line_type(LineType::Microstrip {
    impedance: 50.0,
    substrate: project.get_substrate()?,
})?;

// Route between components
route.route_between("U1.RF_OUT", "U2.RF_IN", RoutingConfig {
    maintain_impedance: true,
    corner_type: Corner::Mitered { miter_ratio: 0.6 },
    via_fences: false,
})?;

// Verify impedance
let impedance = route.calculate_impedance()?;
println!("Impedance: {} Ω", impedance);
```

## Related Topics

- [Component Placement for EM](./component-placement-for-em.md)
- [Via Fences & Stitching](./via-fences-and-stitching.md)
- [RF Design Rule Check](./rf-design-rule-check.md)
