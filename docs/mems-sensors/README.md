# MEMS & Sensors Design Documentation

**Micro-Electro-Mechanical Systems and Sensor Design**

This module covers micro-electro-mechanical systems and sensor design, including mechanical, electrical, and fabrication aspects.

---

## Documentation Index

### Core Architecture & Fundamentals

- [MEMS Project Structure & Management](./core-architecture/mems-project-structure-and-management.md) - Unified MEMS project format
- [Unified MEMS Design File Format](./core-architecture/unified-mems-design-file-format.md) - .hwt_mems format specification
- [GDS JSON as Intermediate Representation](./core-architecture/gds-json-as-intermediate-representation.md) - Universal MEMS data model
- [Mechanical Capture Workflow](./core-architecture/mechanical-capture-workflow.md) - From concept to mask
- [Sensor Integration Workflow](./core-architecture/sensor-integration-workflow.md) - MEMS + ASIC integration
- [Programmatic Scripted Design](./core-architecture/programmatic-scripted-design.md) - Code-first MEMS

### Device Editor Concepts

- [Structures & Libraries](./device-editor/structures-and-libraries.md) - Proof masses, springs, electrodes
- [Hierarchical Assemblies](./device-editor/hierarchical-assemblies.md) - Multi-device systems
- [Anchors & Suspensions](./device-editor/anchors-and-suspensions.md) - Mechanical supports
- [Stress-Strain Rules Check](./device-editor/stress-strain-rules-check.md) - Mechanical validation
- [Actuation & Sensing Annotation](./device-editor/actuation-and-sensing-annotation.md) - Functional labeling
- [MEMS Simulation Integration](./device-editor/mems-simulation-integration.md) - FEA, modal, electrostatic

### Layout Concepts

- [Mask Layers & Libraries](./layout-concepts/mask-layers-and-libraries.md) - Process layer definitions
- [Structure Placement](./layout-concepts/structure-placement.md) - Component positioning
- [Etch Hole Routing](./layout-concepts/etch-hole-routing.md) - Release hole patterns
- [Automatic Mask Generators](./layout-concepts/automatic-mask-generators.md) - Parametric structures
- [Release Zones](./layout-concepts/release-zones.md) - Sacrificial layer regions
- [MEMS Design Rule Check](./layout-concepts/mems-design-rule-check.md) - Process-specific DRC
- [Multi-Layer Silicon Support](./layout-concepts/multi-layer-silicon-support.md) - SOI, PolySi stacks
- [Via & Anchor Stitching](./layout-concepts/via-and-anchor-stitching.md) - Mechanical connections
- [Modal Analysis Highlighting](./layout-concepts/modal-analysis-highlighting.md) - Resonance visualization

### 3D Visualization & Packaging

- [3D MEMS Viewer](./3d-visualization-packaging/3d-mems-viewer.md) - Cross-section, animation
- [Wafer Bonding Export/Import](./3d-visualization-packaging/wafer-bonding-export-import.md) - Cap wafer integration

### Manufacturing & Output Formats

- [GDSII MEMS Export](./manufacturing-output/gdsii-mems-export.md) - Mask data for foundries
- [DXF Export](./manufacturing-output/dxf-export.md) - CAD interchange format
- [Process Flow Generation](./manufacturing-output/process-flow-generation.md) - Fabrication steps
- [Fabrication Drawings with Cross-Sections](./manufacturing-output/fabrication-drawings-with-cross-sections.md) - Process documentation

### Advanced & Productivity Features

- [Process Library Conventions](./advanced-features/process-library-conventions.md) - Foundry standards
- [Design for Reliability Checks](./advanced-features/design-for-reliability-checks.md) - Fatigue, stiction
- [Undo/Redo & Versioning](./advanced-features/undo-redo-and-versioning.md) - MEMS-specific history
- [Command-Line MEMS Synthesis](./advanced-features/command-line-mems-synthesis.md) - Batch generation
- [Resonance & Sensitivity Calculators](./advanced-features/resonance-and-sensitivity-calculators.md) - Performance estimation
- [Etch Constraints](./advanced-features/etch-constraints.md) - Process limitations
- [Real-Time Mechanical-to-Electrical Sync](./advanced-features/real-time-mechanical-to-electrical-sync.md) - Live preview

---

## Quick Start

```rust
// Create MEMS accelerometer
let accel = Accelerometer::new("3axis_accel")?;

// Configure sensing parameters
accel.set_range(16.0)?;           // ±16 g
accel.set_bandwidth(1000.0)?;     // 1 kHz
accel.set_noise_target(100e-6)?;  // 100 µg/√Hz

// Design proof mass
accel.design_proof_mass(ProofMassConfig {
    target_mass: 30e-9,           // 30 µg
    aspect_ratio: 1.0,
    perforation: true,
})?;

// Design springs
accel.design_springs(SpringConfig {
    target_frequency: 3000.0,     // 3 kHz
    type_: SpringType::FoldedBeam,
    beams_per_side: 4,
})?;

// Design sense electrodes
accel.design_electrodes(ElectrodeConfig {
    target_sensitivity: 1.0,      // pF/g
    type_: ElectrodeType::CombFinger,
    fingers: 50,
})?;

// Run simulations
let modal = accel.simulate_modal(10)?;
let stress = accel.simulate_stress(16.0)?;

// Export for fabrication
accel.export_gdsii("accelerometer.gds", Foundry::MEMSCAP_PolyMUMPs)?;
```

## Supported MEMS Types

| Type | Description | Typical Sensitivity | Status |
|------|-------------|---------------------|--------|
| Accelerometer | Capacitive, piezoresistive | 100 µg/√Hz | ✓ Full support |
| Gyroscope | Vibratory, tuning fork | 0.01 °/s/√Hz | ✓ Full support |
| Pressure Sensor | Piezoresistive, capacitive | 0.01% FS | ✓ Full support |
| Microphone | Capacitive MEMS | -38 dBV/Pa | ✓ Full support |
| RF Switch | Electrostatic, piezo | <0.5 dB IL | ✓ Full support |
| Micromirror | Electrostatic, thermal | ±15° tilt | ✓ Full support |

## Supported Foundries

| Foundry | Process | Device Layer | Status |
|---------|---------|--------------|--------|
| MEMSCAP PolyMUMPs | Surface | 2 µm poly | ✓ Full support |
| MEMSCAP SOIMUMPs | SOI bulk | 25 µm Si | ✓ Full support |
| STMicro ThELMA | Epitaxial | 22 µm poly | ✓ Full support |
| X-FAB XMB10 | Bulk | 400 µm Si | ✓ Full support |

## Related Topics

- [IC Design](../ic-design/README.md) - ASIC integration
- [Advanced Packaging](../advanced-packaging/README.md) - MEMS packaging
- [Thermal Simulation](../advanced-features/thermal-simulation.md) - Thermal analysis
