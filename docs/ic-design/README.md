# IC Design Module Documentation

**Integrated Circuit Design for Digital ASIC, Analog, and Mixed-Signal**

This module extends Hardware Tool for integrated circuit design, supporting digital ASIC flows, analog circuits, and mixed-signal integration. Documentation mirrors the core EDA structure for consistency.

---

## Documentation Index

### Core Architecture & Fundamentals

- [IC Project Structure & Management](./core-architecture/ic-project-structure-and-management.md) - Unified IC project format, PDK integration
- [Unified IC Design File Format](./core-architecture/unified-ic-design-file-format.md) - .hwt_ic format specification
- [RTL JSON as Intermediate Representation](./core-architecture/rtl-json-as-intermediate-representation.md) - Universal RTL data model
- [RTL Capture Workflow](./core-architecture/rtl-capture-workflow.md) - From HDL to synthesized netlist
- [Analog Schematic Workflow](./core-architecture/analog-schematic-workflow.md) - Transistor-level design flow
- [Mixed-Signal Integration Workflow](./core-architecture/mixed-signal-integration-workflow.md) - Combining digital and analog
- [Programmatic Verilog/SystemVerilog Design](./core-architecture/programmatic-verilog-systemverilog-design.md) - Code-first HDL

### RTL & Logic Design Concepts

- [Cells & Libraries](./rtl-logic-design/cells-and-libraries.md) - Standard cells, custom cells, PDK libraries
- [Hierarchical Modules & Instances](./rtl-logic-design/hierarchical-modules-and-instances.md) - Module hierarchy, instantiation
- [Wiring & Connectivity](./rtl-logic-design/wiring-and-connectivity.md) - Nets, buses, clock trees
- [Functional Verification Rules Check](./rtl-logic-design/functional-verification-rules-check.md) - Linting, CDC, formal
- [Module Instantiation & Parameters](./rtl-logic-design/module-instantiation-and-parameters.md) - Parameterized modules
- [RTL Simulation Integration](./rtl-logic-design/rtl-simulation-integration.md) - Verilator, Icarus, commercial

### Analog & Mixed-Signal Concepts

- [Device Models & Libraries](./analog-mixed-signal/device-models-and-libraries.md) - SPICE models, PDK devices
- [Component Placement in Layout](./analog-mixed-signal/component-placement-in-layout.md) - Matching, symmetry
- [Interactive Routing for Analog](./analog-mixed-signal/interactive-routing-for-analog.md) - Shielding, matching
- [Automatic Layout Generators](./analog-mixed-signal/automatic-layout-generators.md) - PCell, generators
- [Guard Rings & Substrate Noise](./analog-mixed-signal/guard-rings-and-substrate-noise.md) - Isolation techniques
- [Physical Verification (DRC/LVS)](./analog-mixed-signal/physical-verification-drc-lvs.md) - Design rule checks
- [Multi-Layer Metal Stack Support](./analog-mixed-signal/multi-layer-metal-stack-support.md) - Backend layers
- [Via Customization & Stitching](./analog-mixed-signal/via-customization-and-stitching.md) - Via arrays, stacking
- [Signal Integrity Analysis](./analog-mixed-signal/signal-integrity-analysis.md) - Crosstalk, IR drop

### 3D Visualization & Parasitics

- [3D IC Viewer](./3d-visualization-parasitics/3d-ic-viewer.md) - Layer-by-layer visualization
- [Parasitic Extraction Export/Import](./3d-visualization-parasitics/parasitic-extraction-export-import.md) - SPEF, DSPF, StarRC

### Manufacturing & Output Formats

- [GDSII Export](./manufacturing-output/gdsii-export.md) - Stream format for foundries
- [OASIS Export](./manufacturing-output/oasis-export.md) - Compressed mask format
- [LEF/DEF Export](./manufacturing-output/lef-def-export.md) - Library/Design exchange
- [SPICE Netlist & PDK Integration](./manufacturing-output/spice-netlist-and-pdk-integration.md) - Simulation netlists
- [Tapeout Checklist Generation](./manufacturing-output/tapeout-checklist-generation.md) - Signoff verification

### Advanced & Productivity Features

- [PDK Conventions & Quality Control](./advanced-features/pdk-conventions-and-quality-control.md) - PDK validation
- [Design for Test (DFT) Checks](./advanced-features/design-for-test-dft-checks.md) - Scan, BIST, JTAG
- [Undo/Redo & Versioning](./advanced-features/undo-redo-and-versioning.md) - IC-specific history
- [Command-Line EDA Flows](./advanced-features/command-line-eda-flows.md) - Batch synthesis, P&R
- [Timing & Power Calculators](./advanced-features/timing-and-power-calculators.md) - STA, power estimation
- [Layout Constraints & Floorplanning](./advanced-features/layout-constraints-and-floorplanning.md) - Placement guides
- [Real-Time RTL-to-Layout Sync](./advanced-features/real-time-rtl-to-layout-sync.md) - Live preview

---

## Quick Start

```rust
// Create IC project with PDK
let project = ICProject::new("my_asic", PDK::SkyWater130)?;

// Import RTL
project.import_verilog("rtl/top.v")?;

// Synthesize
let netlist = project.synthesize(SynthesisConfig {
    target: "sky130_fd_sc_hd",
    optimization: Optimization::Area,
    clock_period: 10.0,  // ns
})?;

// Place and route
let layout = project.place_and_route(PnRConfig::default())?;

// Verify
layout.run_drc()?;
layout.run_lvs(&netlist)?;

// Export
layout.export_gdsii("my_asic.gds")?;
```

## Supported PDKs

| PDK | Node | Type | Status |
|-----|------|------|--------|
| SkyWater SKY130 | 130nm | Digital + Analog | ✓ Full support |
| GlobalFoundries GF180MCU | 180nm | Digital + Analog + HV | ✓ Full support |
| IHP SG13G2 | 130nm SiGe | RF + Analog | ✓ Full support |
| ASAP7 | 7nm (predictive) | Digital | ✓ Full support |
| Commercial (TSMC, Samsung, Intel) | Various | All | Import via OpenAccess |

## Related Topics

- [PCB Design](../core-architecture/pcb-layout-workflow.md) - Board-level design
- [Quantum Hardware](../quantum-hardware/README.md) - Quantum IC design
- [Advanced Packaging](../advanced-packaging/README.md) - Multi-die integration
