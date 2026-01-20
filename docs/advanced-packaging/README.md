# Advanced Packaging & Chiplet Integration Documentation

**Multi-Die Integration, 2.5D/3D Packaging, and Heterogeneous Systems**

This module supports multi-die integration, 2.5D/3D packaging, and heterogeneous systems.

---

## Documentation Index

### Core Architecture & Fundamentals

- [Chiplet Project Structure & Management](./core-architecture/chiplet-project-structure-and-management.md) - Unified packaging project format
- [Unified Packaging File Format](./core-architecture/unified-packaging-file-format.md) - .hwt_pkg format specification
- [Die JSON as Intermediate Representation](./core-architecture/die-json-as-intermediate-representation.md) - Universal die/package model
- [Interposer Capture Workflow](./core-architecture/interposer-capture-workflow.md) - 2.5D design flow
- [Heterogeneous Integration Workflow](./core-architecture/heterogeneous-integration-workflow.md) - Multi-technology integration
- [Programmatic Die Stacking Design](./core-architecture/programmatic-die-stacking-design.md) - Code-first packaging

### Die Editor Concepts

- [Die IP & Libraries](./die-editor/die-ip-and-libraries.md) - Chiplet IP blocks
- [Hierarchical Multi-Die Assemblies](./die-editor/hierarchical-multi-die-assemblies.md) - System organization
- [TSV & Microbump Connectivity](./die-editor/tsv-and-microbump-connectivity.md) - Die-to-die connections
- [Thermal Management Rules Check](./die-editor/thermal-management-rules-check.md) - Heat dissipation validation
- [Die Alignment & Bonding Annotation](./die-editor/die-alignment-and-bonding-annotation.md) - Assembly specifications
- [Packaging Simulation Integration](./die-editor/packaging-simulation-integration.md) - Thermal, mechanical, SI

### Layout Concepts

- [Bump Maps & Libraries](./layout-concepts/bump-maps-and-libraries.md) - Micro-bump, C4 patterns
- [Die Placement & Stacking](./layout-concepts/die-placement-and-stacking.md) - 2.5D/3D arrangement
- [RDL & Interposer Routing](./layout-concepts/rdl-and-interposer-routing.md) - Redistribution layers
- [Automatic Bond Wire Generators](./layout-concepts/automatic-bond-wire-generators.md) - Wire bond patterns
- [Heat Spreader Zones](./layout-concepts/heat-spreader-zones.md) - Thermal management regions
- [Packaging Design Rule Check](./layout-concepts/packaging-design-rule-check.md) - Assembly DRC
- [Multi-Level Stacking Support](./layout-concepts/multi-level-stacking-support.md) - 3D die stacks
- [TSV & Bump Stitching](./layout-concepts/tsv-and-bump-stitching.md) - Vertical connections
- [Stress-Strain Highlighting](./layout-concepts/stress-strain-highlighting.md) - Mechanical visualization

### 3D Visualization & Thermal

- [3D Package Viewer](./3d-visualization-thermal/3d-package-viewer.md) - Cross-section, exploded view
- [Thermal-Mechanical Export/Import](./3d-visualization-thermal/thermal-mechanical-export-import.md) - FEA data interchange

### Manufacturing & Output Formats

- [ODB++ Packaging Export](./manufacturing-output/odb-packaging-export.md) - Substrate data
- [APD Export](./manufacturing-output/apd-export.md) - Advanced Package Description
- [Assembly Data Generation](./manufacturing-output/assembly-data-generation.md) - Pick-and-place, bonding
- [Fabrication Drawings with Stackups](./manufacturing-output/fabrication-drawings-with-stackups.md) - Package documentation

### Advanced & Productivity Features

- [Foundry Library Conventions](./advanced-features/foundry-library-conventions.md) - OSAT standards
- [Design for Warpage Checks](./advanced-features/design-for-warpage-checks.md) - Thermal cycling validation
- [Undo/Redo & Versioning](./advanced-features/undo-redo-and-versioning.md) - Packaging-specific history
- [Command-Line Packaging Flows](./advanced-features/command-line-packaging-flows.md) - Batch assembly
- [Power Integrity Calculators](./advanced-features/power-integrity-calculators.md) - PDN analysis
- [Alignment Constraints](./advanced-features/alignment-constraints.md) - Die placement rules
- [Real-Time Die-to-Package Sync](./advanced-features/real-time-die-to-package-sync.md) - Live preview

---

## Quick Start

```rust
// Create chiplet system
let system = ChipletSystem::new("hpc_accelerator")?;

// Add dies
let compute = system.add_die(Die::new("compute", Technology::N5)?)?;
let hbm1 = system.add_die(Die::new("hbm3", Technology::HBM3)?)?;
let hbm2 = system.add_die(Die::new("hbm3", Technology::HBM3)?)?;
let io = system.add_die(Die::new("io", Technology::N12)?)?;

// Create silicon interposer
let interposer = system.create_interposer(InterposerConfig {
    size: (55e-3, 55e-3),
    rdl_layers: 4,
    tsv_pitch: 50e-6,
})?;

// Place dies on interposer
interposer.place_die(&compute, Position::center())?;
interposer.place_die(&hbm1, Position::left_of(&compute, 2e-3))?;
interposer.place_die(&hbm2, Position::right_of(&compute, 2e-3))?;
interposer.place_die(&io, Position::below(&compute, 2e-3))?;

// Connect dies with UCIe
system.connect(&compute, &hbm1, UCIe::new(64))?;
system.connect(&compute, &hbm2, UCIe::new(64))?;
system.connect(&compute, &io, UCIe::new(32))?;

// Run analyses
let thermal = system.analyze_thermal()?;
let si = system.analyze_si()?;

// Export
system.export_gdsii("interposer.gds")?;
system.export_substrate("substrate.brd")?;
```

## Supported Packaging Technologies

| Technology | Description | Pitch | Status |
|------------|-------------|-------|--------|
| 2.5D Interposer | Silicon/organic interposer | 50-100 µm | ✓ Full support |
| 3D Stacking | Die-on-die with TSV | 10-50 µm | ✓ Full support |
| Fan-Out WLP | RDL-based redistribution | 200-400 µm | ✓ Full support |
| EMIB | Embedded bridge die | 55 µm | ✓ Full support |
| CoWoS | Chip-on-Wafer-on-Substrate | 40 µm | ✓ Full support |
| Foveros | Face-to-face stacking | 10 µm | ✓ Full support |
| UCIe | Universal Chiplet Interconnect | 25-100 µm | ✓ Full support |

## Supported Interconnects

| Interconnect | Data Rate | Lanes | Status |
|--------------|-----------|-------|--------|
| UCIe Standard | 4-32 GT/s | 16-64 | ✓ Full support |
| UCIe Advanced | 32 GT/s | 16-64 | ✓ Full support |
| HBM3 | 6.4 Gbps/pin | 1024 | ✓ Full support |
| BoW (Bunch of Wires) | 16 GT/s | 16-256 | ✓ Full support |
| AIB (Advanced Interface Bus) | 2 Gbps | 20-320 | ✓ Full support |

## Related Topics

- [IC Design](../ic-design/README.md) - Die design
- [PCB Layout](../pcb-layout/component-placement.md) - Board integration
- [Thermal Simulation](../advanced-features/thermal-simulation.md) - Thermal analysis
- [Signal Integrity](../advanced-features/signal-power-integrity.md) - SI/PI analysis
