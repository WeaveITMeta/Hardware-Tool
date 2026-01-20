# Cells & Libraries

## Overview

Hardware Tool provides comprehensive standard cell and custom cell library support for digital IC design. Libraries integrate with PDKs and support timing, power, and physical characterization.

## Standard Cell Libraries

```rust
StandardCellLibrary {
    // Library identification
    info: LibraryInfo {
        name: "sky130_fd_sc_hd",
        description: "SkyWater 130nm high-density standard cells",
        pdk: "sky130",
        voltage: 1.8,
        temperature: 25.0,
    },
    
    // Cell categories
    categories: vec![
        CellCategory::Combinational,   // AND, OR, XOR, MUX, etc.
        CellCategory::Sequential,       // DFF, latch, etc.
        CellCategory::Buffer,           // BUF, INV, CLKBUF
        CellCategory::Special,          // Tie cells, filler, tap
    ],
    
    // Drive strengths
    drive_strengths: vec![1, 2, 4, 6, 8, 12, 16],
    
    // Characterization
    characterization: CellCharacterization {
        timing: true,                   // Liberty (.lib)
        power: true,                    // Power tables
        noise: true,                    // Noise immunity
        physical: true,                 // LEF
    },
}
```

## Cell Browser

```
┌─────────────────────────────────────────────────────────────────┐
│ Standard Cell Browser: sky130_fd_sc_hd                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Search: [and2                          ] [🔍]                   │
│                                                                 │
│ Category: [All ▼]  Drive: [All ▼]  Vt: [All ▼]                 │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Cell Name          │ Function │ Drive │ Area   │ Delay     │ │
│ │ ───────────────────┼──────────┼───────┼────────┼────────── │ │
│ │ sky130_fd_sc_hd__  │          │       │        │           │ │
│ │   and2_1           │ A & B    │   1   │ 3.75   │ 0.12 ns   │ │
│ │   and2_2           │ A & B    │   2   │ 5.00   │ 0.09 ns   │ │
│ │   and2_4           │ A & B    │   4   │ 7.50   │ 0.07 ns   │ │
│ │   and2b_1          │ A & !B   │   1   │ 5.00   │ 0.14 ns   │ │
│ │   and2b_2          │ A & !B   │   2   │ 6.25   │ 0.11 ns   │ │
│ │   and3_1           │ A&B&C    │   1   │ 5.00   │ 0.15 ns   │ │
│ │   and3_2           │ A&B&C    │   2   │ 6.25   │ 0.12 ns   │ │
│ │   and4_1           │ A&B&C&D  │   1   │ 6.25   │ 0.18 ns   │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Selected: and2_2                                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Symbol:     ┌───┐                                           │ │
│ │         A ──┤   │                                           │ │
│ │             │ & ├── X                                       │ │
│ │         B ──┤   │                                           │ │
│ │             └───┘                                           │ │
│ │                                                             │ │
│ │ Timing: Rise=0.09ns, Fall=0.08ns @ CL=10fF                 │ │
│ │ Power: Leakage=0.5nW, Dynamic=2.1µW @ 100MHz               │ │
│ │ Area: 5.00 µm²                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [View LEF] [View Liberty] [Insert] [Close]                      │
└─────────────────────────────────────────────────────────────────┘
```

## Liberty Timing Model

```rust
LibertyModel {
    // Cell definition
    cell: CellDefinition {
        name: "and2_2",
        area: 5.0,
        cell_leakage_power: 0.5e-9,
    },
    
    // Pin definitions
    pins: vec![
        Pin {
            name: "A",
            direction: Direction::Input,
            capacitance: 2.5e-15,
        },
        Pin {
            name: "B",
            direction: Direction::Input,
            capacitance: 2.5e-15,
        },
        Pin {
            name: "X",
            direction: Direction::Output,
            function: "A & B",
            timing: vec![
                TimingArc {
                    related_pin: "A",
                    timing_type: TimingType::Combinational,
                    cell_rise: LUT2D { /* NLDM table */ },
                    cell_fall: LUT2D { /* NLDM table */ },
                    rise_transition: LUT2D { /* NLDM table */ },
                    fall_transition: LUT2D { /* NLDM table */ },
                },
            ],
        },
    ],
}
```

## Rust API

```rust
// Load standard cell library
let lib = StandardCellLibrary::load("sky130_fd_sc_hd")?;

// Browse cells
let and_cells = lib.find_cells("and*")?;
for cell in and_cells {
    println!("{}: area={}, delay={}", cell.name, cell.area, cell.delay);
}

// Get cell details
let and2 = lib.get_cell("and2_2")?;
println!("Leakage: {} nW", and2.leakage_power * 1e9);

// Use in synthesis
synthesis.set_target_library(&lib)?;
```

## Related Topics

- [Hierarchical Modules & Instances](./hierarchical-modules-and-instances.md)
- [RTL Simulation Integration](./rtl-simulation-integration.md)
