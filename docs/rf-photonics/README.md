# RF, Microwave & Photonics Design Documentation

**Radio Frequency, Microwave, and Photonic Integrated Circuit Design**

This module handles radio frequency, microwave, and photonic designs, focusing on high-frequency and optical components.

---

## Documentation Index

### Core Architecture & Fundamentals

- [RF Project Structure & Management](./core-architecture/rf-project-structure-and-management.md) - Unified RF project format
- [Unified RF Design File Format](./core-architecture/unified-rf-design-file-format.md) - .hwt_rf format specification
- [S-Parameter JSON as Intermediate Representation](./core-architecture/s-parameter-json-as-intermediate-representation.md) - Universal RF data model
- [RF Schematic Capture Workflow](./core-architecture/rf-schematic-capture-workflow.md) - From concept to simulation
- [Waveguide Layout Workflow](./core-architecture/waveguide-layout-workflow.md) - Physical RF/photonic layout
- [Programmatic EM Design](./core-architecture/programmatic-em-design.md) - Code-first RF structures

### Schematic Editor Concepts

- [RF Components & Libraries](./schematic-editor/rf-components-and-libraries.md) - Transistors, passives, models
- [Hierarchical RF Blocks](./schematic-editor/hierarchical-rf-blocks.md) - Amplifier stages, filters
- [Transmission Lines & Connectivity](./schematic-editor/transmission-lines-and-connectivity.md) - Microstrip, CPW, stripline
- [Impedance Matching Rules Check](./schematic-editor/impedance-matching-rules-check.md) - VSWR, return loss validation
- [Port Annotation & S-Parameters](./schematic-editor/port-annotation-and-s-parameters.md) - Network analysis setup
- [RF Simulation Integration](./schematic-editor/rf-simulation-integration.md) - SPICE, harmonic balance

### Layout Concepts

- [RF Footprints & Libraries](./layout-concepts/rf-footprints-and-libraries.md) - Component land patterns
- [Component Placement for EM](./layout-concepts/component-placement-for-em.md) - Coupling-aware placement
- [Microstrip/Coplanar Routing](./layout-concepts/microstrip-coplanar-routing.md) - Controlled impedance
- [Automatic EM Optimizations](./layout-concepts/automatic-em-optimizations.md) - Tuning, matching
- [Ground Planes & Shielding](./layout-concepts/ground-planes-and-shielding.md) - EMI mitigation
- [RF Design Rule Check](./layout-concepts/rf-design-rule-check.md) - High-frequency DRC
- [Multi-Layer Dielectric Support](./layout-concepts/multi-layer-dielectric-support.md) - Substrate stacks
- [Via Fences & Stitching](./layout-concepts/via-fences-and-stitching.md) - Isolation, grounding
- [Field Distribution Highlighting](./layout-concepts/field-distribution-highlighting.md) - E/H field visualization

### 3D Visualization & EM

- [3D RF Viewer](./3d-visualization-em/3d-rf-viewer.md) - Structure visualization
- [EM Field Export/Import](./3d-visualization-em/em-field-export-import.md) - Field data interchange

### Manufacturing & Output Formats

- [Gerber RF Export](./manufacturing-output/gerber-rf-export.md) - PCB-based RF
- [DXF Photonics Export](./manufacturing-output/dxf-photonics-export.md) - Waveguide masks
- [S-Parameter Data Generation](./manufacturing-output/s-parameter-data-generation.md) - Touchstone files
- [Fabrication Drawings with Tolerances](./manufacturing-output/fabrication-drawings-with-tolerances.md) - RF specifications

### Advanced & Productivity Features

- [Substrate Library Conventions](./advanced-features/substrate-library-conventions.md) - Material standards
- [Design for Signal Integrity Checks](./advanced-features/design-for-signal-integrity-checks.md) - High-speed validation
- [Undo/Redo & Versioning](./advanced-features/undo-redo-and-versioning.md) - RF-specific history
- [Command-Line EM Solvers](./advanced-features/command-line-em-solvers.md) - Batch simulation
- [Insertion Loss & VSWR Calculators](./advanced-features/insertion-loss-and-vswr-calculators.md) - Performance tools
- [Coupling Constraints](./advanced-features/coupling-constraints.md) - Isolation requirements
- [Real-Time Schematic-to-EM Sync](./advanced-features/real-time-schematic-to-em-sync.md) - Live preview

---

## Quick Start

```rust
// Create RF amplifier design
let lna = LNA::new("wifi_lna")?;

// Set specifications
lna.set_frequency(2.4e9, 2.5e9)?;  // 2.4-2.5 GHz
lna.set_gain(15.0)?;                // 15 dB
lna.set_noise_figure(1.5)?;         // 1.5 dB
lna.set_iip3(-5.0)?;                // -5 dBm

// Select transistor
lna.set_transistor(Transistor::from_model("BFU730F"))?;

// Design matching networks
let input_match = lna.design_input_match(MatchType::LNetwork)?;
let output_match = lna.design_output_match(MatchType::PiNetwork)?;

// Simulate
let result = lna.simulate(SimConfig {
    frequency: FrequencySweep::linear(2e9, 3e9, 101),
    analyses: vec![Analysis::SParameters, Analysis::NoiseFigure],
})?;

// Create filter
let filter = BandpassFilter::new("10ghz_bpf")?;
filter.set_center_frequency(10e9)?;
filter.set_bandwidth(500e6)?;
filter.synthesize(FilterType::Chebyshev { ripple: 0.1 })?;

// Export
lna.export_gerber("lna_gerber/")?;
filter.export_gerber("filter_gerber/")?;
```

## Supported RF/Photonic Components

| Category | Components | Frequency Range | Status |
|----------|------------|-----------------|--------|
| Amplifiers | LNA, PA, VGA | DC - 100 GHz | ✓ Full support |
| Filters | LP, HP, BP, BS, notch | DC - 100 GHz | ✓ Full support |
| Mixers | Gilbert cell, passive | DC - 60 GHz | ✓ Full support |
| Oscillators | VCO, crystal, MEMS | 1 MHz - 100 GHz | ✓ Full support |
| Antennas | Patch, dipole, array | 100 MHz - 100 GHz | ✓ Full support |
| Waveguides | Microstrip, CPW, stripline | DC - 100 GHz | ✓ Full support |
| Photonics | Waveguide, ring, MZI | 1300-1600 nm | ✓ Full support |

## Supported Substrates

| Substrate | εr | tan δ | Status |
|-----------|-----|-------|--------|
| FR4 | 4.4 | 0.02 | ✓ Full support |
| Rogers RO4003C | 3.55 | 0.0027 | ✓ Full support |
| Rogers RT/duroid 5880 | 2.2 | 0.0009 | ✓ Full support |
| Alumina (Al₂O₃) | 9.8 | 0.0001 | ✓ Full support |
| Silicon (high-res) | 11.9 | 0.001 | ✓ Full support |
| Silicon Nitride | 2.0 | 0.0001 | ✓ Full support (photonics) |

## Related Topics

- [Signal Integrity](../advanced-features/signal-power-integrity.md) - SI/PI analysis
- [EMC Simulation](../advanced-features/electromagnetic-simulation.md) - EMC/EMI
- [IC Design](../ic-design/README.md) - RF IC design
- [Quantum Hardware](../quantum-hardware/README.md) - Quantum control
