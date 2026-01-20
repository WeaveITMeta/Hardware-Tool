# Qubit Placement

## Overview

Hardware Tool provides intelligent qubit placement for quantum processor layouts, supporting various topologies including linear chains, 2D grids, heavy-hex, and custom arrangements. Placement considers frequency collision avoidance, coupling requirements, and control line routing.

## Placement Topologies

```rust
QubitTopology {
    // Predefined topologies
    predefined: vec![
        Topology::Linear {
            qubits: 5,
            spacing: 500e-6,          // 500 µm
        },
        Topology::Grid {
            rows: 3,
            cols: 3,
            spacing: 500e-6,
        },
        Topology::HeavyHex {
            size: 27,                 // IBM-style
            spacing: 500e-6,
        },
        Topology::Custom {
            positions: vec![/* custom coordinates */],
        },
    ],
    
    // Placement constraints
    constraints: PlacementConstraints {
        min_qubit_spacing: 300e-6,    // Minimum separation
        max_coupling_distance: 600e-6, // Maximum for coupling
        frequency_collision: FrequencyCollision {
            min_detuning: 100e6,      // 100 MHz minimum
            check_neighbors: 2,        // Check 2-hop neighbors
        },
    },
}
```

## Placement Editor

```
┌─────────────────────────────────────────────────────────────────┐
│ Qubit Placement: 5-Qubit Linear Chain                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Topology: [Linear ▼]  Qubits: [5    ]  Spacing: [500 µm]       │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │                                                             │ │
│ │    ┌───┐      ┌───┐      ┌───┐      ┌───┐      ┌───┐      │ │
│ │    │Q0 │──────│Q1 │──────│Q2 │──────│Q3 │──────│Q4 │      │ │
│ │    │5.0│      │5.15│     │5.3│      │5.45│     │5.6│      │ │
│ │    │GHz│      │GHz│      │GHz│      │GHz│      │GHz│      │ │
│ │    └───┘      └───┘      └───┘      └───┘      └───┘      │ │
│ │      │          │          │          │          │         │ │
│ │    ═════      ═════      ═════      ═════      ═════      │ │
│ │     R0         R1         R2         R3         R4        │ │
│ │    7.0GHz    7.2GHz     7.4GHz     7.6GHz     7.8GHz      │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Frequency Assignment:                                           │
│   Strategy: [Staggered ▼]  Detuning: [150 MHz]                 │
│                                                                 │
│ Coupling Analysis:                                              │
│   Q0-Q1: 30 MHz ✓    Q1-Q2: 30 MHz ✓    Q2-Q3: 30 MHz ✓       │
│   Q3-Q4: 30 MHz ✓                                              │
│                                                                 │
│ Frequency Collisions: None ✓                                   │
│                                                                 │
│ [Auto-Assign Frequencies] [Check Collisions] [Generate Layout]  │
└─────────────────────────────────────────────────────────────────┘
```

## Frequency Assignment

```rust
FrequencyAssignment {
    // Assignment strategy
    strategy: FrequencyStrategy::Staggered {
        base_frequency: 5.0e9,
        step: 150e6,
    },
    
    // Collision avoidance
    collision_avoidance: CollisionAvoidance {
        // Direct neighbors
        neighbor_detuning: 150e6,
        
        // Two-qubit gate frequencies
        gate_frequencies: vec![
            GateFrequency::SUM,       // f1 + f2
            GateFrequency::DIFF,      // |f1 - f2|
        ],
        
        // Readout resonators
        resonator_detuning: 1.5e9,    // From qubit
    },
}
```

## Rust API

```rust
// Create processor with topology
let processor = QuantumProcessor::new("5q_linear")?;

// Set topology
processor.set_topology(Topology::Linear {
    qubits: 5,
    spacing: 500e-6,
})?;

// Auto-assign frequencies
processor.assign_frequencies(FrequencyStrategy::Staggered {
    base_frequency: 5.0e9,
    step: 150e6,
})?;

// Check for collisions
let collisions = processor.check_frequency_collisions()?;
if collisions.is_empty() {
    println!("No frequency collisions!");
}

// Add readout resonators
processor.add_readout_resonators(ResonatorConfig {
    base_frequency: 7.0e9,
    step: 200e6,
})?;

// Generate physical layout
processor.generate_layout()?;
```

## Related Topics

- [Control Line Routing](./control-line-routing.md)
- [Automatic Topology Generators](./automatic-topology-generators.md)
- [Coherence Time Analysis](./coherence-time-analysis.md)
