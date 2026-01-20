# Hardware Tool Documentation

**The Universal Hardware Design Platform**

*One unified environment for ALL hardware: PCBs, Integrated Circuits, Quantum Processors, MEMS/Sensors, RF/Photonics, and Advanced Packaging.*

---

## Platform Overview

Hardware Tool is a revolutionary, pure-Rust EDA suite that transcends traditional boundaries between hardware domains. Whether you're designing a simple PCB, a complex ASIC, a quantum processor, a MEMS accelerometer, an RF front-end, or a multi-die chiplet system — Hardware Tool provides a unified platform with consistent workflows, shared simulation engines, and seamless integration.

**Core Technologies:** Rust + Slint + Bevy 2D/3D + Multi-Physics Simulation  
**Output Formats:** Gerber RS-274X, IPC-2581, ODB++, GDSII, OASIS, LEF/DEF, Touchstone

---

## Hardware Domains

Hardware Tool treats all hardware types as first-class citizens. Each domain shares the same core architecture while providing specialized tools for domain-specific needs.

| Domain | Description | Key Outputs |
|--------|-------------|-------------|
| **PCB** | Printed circuit boards, from simple to HDI | Gerber, IPC-2581, ODB++ |
| **IC** | Digital ASIC, analog, mixed-signal ICs | GDSII, OASIS, LEF/DEF |
| **Quantum** | Superconducting, photonic, trapped ion qubits | GDSII, Qiskit Pulse |
| **MEMS** | Accelerometers, gyroscopes, pressure sensors | GDSII, DXF |
| **RF/Photonics** | LNAs, filters, antennas, waveguides | Gerber, GDSII, Touchstone |
| **Packaging** | Chiplets, 2.5D/3D, interposers, TSV | ODB++, GDSII |

---

## Documentation Index

### 1. Unified Platform Foundation

These modules are shared across ALL hardware domains, providing consistent project management, design capture, and simulation infrastructure.

#### 1.1 Core Architecture (All Domains)

- [Project Structure & Management](./core-architecture/project-structure-management.md) - Unified project format for any hardware type
- [Unified Project File Format](./core-architecture/unified-project-file-format.md) - .hwt format specification
- [Circuit JSON as Intermediate Representation](./core-architecture/circuit-json-ir.md) - Universal data model
- [Programmatic / Code-First Design](./core-architecture/programmatic-design.md) - Rust-native hardware description

#### 1.2 Shared Design Capture

- [Symbols & Libraries](./schematic-editor/symbols-libraries.md) - Component symbols across all domains
- [Hierarchical Design](./schematic-editor/hierarchical-schematics.md) - Multi-level hierarchy for any hardware
- [Wiring & Connectivity](./schematic-editor/wiring-connectivity.md) - Nets, buses, power distribution
- [Annotation & Designators](./schematic-editor/annotation-reference-designators.md) - Consistent naming conventions

#### 1.3 Shared Layout Infrastructure

- [Component/Device Placement](./pcb-layout/component-placement.md) - Placement algorithms for all domains
- [Interactive Routing](./pcb-layout/interactive-routing.md) - Push-and-shove, differential pairs
- [Design Rule Check (DRC)](./pcb-layout/drc.md) - Extensible rule engine
- [Multi-Layer Support](./pcb-layout/multi-layer.md) - Layer stack management

#### 1.4 Shared 3D Visualization

- [3D Viewer](./3d-visualization/3d-pcb-viewer.md) - Real-time rendering for all hardware types
- [STEP & 3D Model Integration](./3d-visualization/step-3d-models.md) - Mechanical fit checks

#### 1.5 Shared Simulation Engines

- [Thermal Simulation](./advanced-features/thermal-simulation.md) - FEM-based heat analysis
- [Signal & Power Integrity](./advanced-features/signal-power-integrity.md) - S-parameters, PDN analysis
- [Electromagnetic Simulation](./advanced-features/electromagnetic-simulation.md) - EMC/EMI, field solvers
- [SPICE Simulation](./schematic-editor/spice-simulation.md) - Circuit-level simulation

#### 1.6 Shared Productivity Tools

- [Undo/Redo & Versioning](./advanced-features/undo-redo-versioning.md) - Command-based history
- [Command-Line Interface](./advanced-features/cli.md) - Batch processing, automation
- [Calculator Tools](./advanced-features/calculator-tools.md) - Domain-specific calculators
- [Real-Time Collaboration](./advanced-features/realtime-collaboration.md) - Multi-user editing
- [Variant Manager](./advanced-features/variant-manager.md) - Design exploration
- [Digital Twin & AR Debug](./advanced-features/digital-twin-ar.md) - Live simulation overlay

#### 1.7 AI Integration (All Domains)

- [API Keys Configuration](./ai-integration/api-keys-configuration.md) - Multi-provider setup
- [Native AI Design Assistant](./ai-integration/native-ai-assistant.md) - First-party AI
- [AI-Powered Optimization](./ai-integration/ai-routing-optimization.md) - Routing, placement
- [Generative AI Design](./advanced-features/generative-ai-design.md) - Spec-to-design

#### 1.8 UX/UI Design (All Domains)

- [Main Window Layout](./ux-ui-design/main-window-layout.md) - Adaptive interface
- [Interaction Patterns](./ux-ui-design/innovative-interaction-patterns.md) - Magnet cursor, gestures
- [Visual Style Guidelines](./ux-ui-design/visual-style-guidelines.md) - Consistent aesthetics
- [Accessibility & Theming](./ux-ui-design/accessibility-and-theming.md) - Inclusive design
- [Keyboard Shortcuts](./ux-ui-design/keyboard-shortcuts-reference.md) - Efficiency
- [Onboarding](./ux-ui-design/onboarding-and-first-experience.md) - Getting started

---

### 2. Domain-Specific Modules

Each hardware domain extends the unified platform with specialized workflows, libraries, verification rules, and manufacturing outputs.

#### 2.1 PCB Design

*Board-level electronic design from schematic to manufacturing.*

| Category | Documentation |
|----------|---------------|
| **Workflow** | [Schematic Capture](./core-architecture/schematic-capture-workflow.md), [PCB Layout](./core-architecture/pcb-layout-workflow.md) |
| **Libraries** | [Footprints & 3D Models](./pcb-layout/footprints-libraries.md) |
| **Layout** | [Auto-Routing](./pcb-layout/auto-routing.md), [Copper Zones](./pcb-layout/copper-zones.md), [Via Stitching](./pcb-layout/via-stitching.md) |
| **Verification** | [ERC](./schematic-editor/erc.md), [DRC](./pcb-layout/drc.md), [DFM](./advanced-features/dfm-checks.md) |
| **Output** | [Gerber](./manufacturing-output/gerber-export.md), [IPC-2581](./manufacturing-output/ipc2581-export.md), [ODB++](./manufacturing-output/odbpp-export.md), [BOM](./manufacturing-output/bom-pick-place.md) |

#### 2.2 Integrated Circuit Design

*Transistor-level design for digital ASIC, analog, and mixed-signal ICs.*

| Category | Documentation |
|----------|---------------|
| **Module Index** | [IC Design Module](./ic-design/README.md) |
| **Overview** | [IC Design Overview](./ic-design/integrated-circuit-design.md) |
| **Project** | [IC Project Structure](./ic-design/core-architecture/ic-project-structure-and-management.md) |
| **Libraries** | [Cells & Libraries](./ic-design/rtl-logic-design/cells-and-libraries.md) |
| **Verification** | [Physical Verification (DRC/LVS)](./ic-design/analog-mixed-signal/physical-verification-drc-lvs.md) |
| **Output** | [GDSII Export](./ic-design/manufacturing-output/gdsii-export.md) |
| **Analysis** | [Timing & Power Calculators](./ic-design/advanced-features/timing-and-power-calculators.md) |

#### 2.3 Quantum Hardware Design

*Superconducting qubits, photonic circuits, trapped ions, and control systems.*

| Category | Documentation |
|----------|---------------|
| **Module Index** | [Quantum Design Module](./quantum-hardware/README.md) |
| **Overview** | [Quantum Circuit Design](./quantum-hardware/quantum-circuit-design.md) |
| **Project** | [Quantum Project Structure](./quantum-hardware/core-architecture/quantum-project-structure-and-management.md) |
| **Layout** | [Qubit Placement](./quantum-hardware/layout-concepts/qubit-placement.md) |
| **3D/Cryo** | [3D Qubit Viewer](./quantum-hardware/3d-visualization-cryogenics/3d-qubit-viewer.md), [Cryogenic Integration](./quantum-hardware/3d-visualization-cryogenics/cryogenic-integration-export-import.md) |
| **Output** | [GDSII Quantum](./quantum-hardware/manufacturing-output/gdsii-quantum-export.md), [Qiskit Pulse](./quantum-hardware/manufacturing-output/qiskit-pulse-export.md) |
| **Analysis** | [Decoherence & Fidelity](./quantum-hardware/advanced-features/decoherence-and-fidelity-calculators.md) |

#### 2.4 MEMS & Sensor Design

*Accelerometers, gyroscopes, pressure sensors, and micro-actuators.*

| Category | Documentation |
|----------|---------------|
| **Module Index** | [MEMS Design Module](./mems-sensors/README.md) |
| **Overview** | [MEMS Sensor Design](./mems-sensors/mems-sensor-design.md) |
| **Project** | [MEMS Project Structure](./mems-sensors/core-architecture/mems-project-structure-and-management.md) |
| **Verification** | [MEMS Design Rule Check](./mems-sensors/layout-concepts/mems-design-rule-check.md) |
| **Analysis** | [Resonance & Sensitivity](./mems-sensors/advanced-features/resonance-and-sensitivity-calculators.md) |

#### 2.5 RF, Microwave & Photonics Design

*LNAs, PAs, filters, antennas, waveguides, and photonic integrated circuits.*

| Category | Documentation |
|----------|---------------|
| **Module Index** | [RF Design Module](./rf-photonics/README.md) |
| **Overview** | [RF/Microwave Design](./rf-photonics/rf-microwave-design.md) |
| **Project** | [RF Project Structure](./rf-photonics/core-architecture/rf-project-structure-and-management.md) |
| **Schematic** | [RF Components](./rf-photonics/schematic-editor/rf-components-and-libraries.md), [Impedance Matching](./rf-photonics/schematic-editor/impedance-matching-rules-check.md) |
| **Layout** | [Microstrip & CPW Routing](./rf-photonics/layout-concepts/microstrip-coplanar-routing.md) |
| **Output** | [RF Gerber](./rf-photonics/manufacturing-output/gerber-rf-export.md), [S-Parameters](./rf-photonics/manufacturing-output/s-parameter-data-generation.md) |
| **Analysis** | [Insertion Loss & VSWR](./rf-photonics/advanced-features/insertion-loss-and-vswr-calculators.md) |

#### 2.6 Advanced Packaging & Chiplets

*2.5D/3D integration, interposers, TSV, UCIe, and heterogeneous systems.*

| Category | Documentation |
|----------|---------------|
| **Module Index** | [Chiplet Design Module](./advanced-packaging/README.md) |
| **Overview** | [Chiplet Integration](./advanced-packaging/chiplet-integration.md) |
| **Project** | [Chiplet Project Structure](./advanced-packaging/core-architecture/chiplet-project-structure-and-management.md) |
| **Die Editor** | [Die IP & Libraries](./advanced-packaging/die-editor/die-ip-and-libraries.md), [TSV & Microbump](./advanced-packaging/die-editor/tsv-and-microbump-connectivity.md) |
| **Layout** | [RDL & Interposer Routing](./advanced-packaging/layout-concepts/rdl-and-interposer-routing.md) |
| **3D/Thermal** | [3D Package Viewer](./advanced-packaging/3d-visualization-thermal/3d-package-viewer.md), [Thermal-Mechanical](./advanced-packaging/3d-visualization-thermal/thermal-mechanical-export-import.md) |
| **Output** | [ODB++ Packaging](./advanced-packaging/manufacturing-output/odb-packaging-export.md), [Assembly Data](./advanced-packaging/manufacturing-output/assembly-data-generation.md) |
| **Analysis** | [Power Integrity](./advanced-packaging/advanced-features/power-integrity-calculators.md) |

---

### 3. Appendices

- [Performance Targets](./appendices/performance-targets.md) - Response times, memory usage, benchmarks
- [Roadmap & Priorities](./appendices/roadmap-priorities.md) - Development phases, milestones
- [Comparison with KiCAD and TSCircuit](./appendices/comparison-with-kicad-and-tscircuit.md) - Feature matrix, philosophy

---

## Quick Start

```bash
# Install Hardware Tool
cargo install hardware-tool

# Create new project (specify hardware type)
hwt new my_pcb --type pcb
hwt new my_asic --type ic
hwt new my_quantum --type quantum
hwt new my_mems --type mems
hwt new my_rf --type rf
hwt new my_chiplet --type packaging

# Open GUI
hwt open my_project.hwt

# Export (format auto-detected by hardware type)
hwt export my_board.hwt       # → Gerber for PCB
hwt export my_chip.hwt_ic     # → GDSII for IC
hwt export my_processor.hwt_q # → GDSII + Qiskit for Quantum

# AI-assisted design (works across all domains)
hwt ai optimize --target performance
```

## Technology Stack

| Component | Technology | Used By |
|-----------|------------|---------|
| **Language** | Rust | All domains |
| **UI Framework** | Slint | All domains |
| **3D Rendering** | Bevy | All domains |
| **Circuit Simulation** | ngspice, custom | PCB, IC, RF |
| **EM Simulation** | FDTD, BEM | RF, Quantum, IC |
| **FEA Simulation** | Custom FEM | MEMS, Packaging, Thermal |
| **AI Integration** | OpenAI, Anthropic, Ollama | All domains |

## Core UX Philosophy

1. **Unified Experience** — Same interface patterns across all hardware domains
2. **Fluid Canvas-First** — One continuous, zoomable, pannable workspace
3. **Context-Aware Interface** — UI adapts to current hardware type and task
4. **Dual-Paradigm Mastery** — GUI and code-first workflows, deeply synchronized
5. **Performance as a Feature** — Instant response even on very large designs
6. **Accessibility by Default** — Dark/light, high-contrast, color-blind friendly

## License

[License information here]
