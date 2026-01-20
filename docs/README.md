# Hardware Tool Documentation

**A pure Rust KiCAD/TSCircuit replacement**

Rust + Slint + PCB Editor + Bevy 2D/3D  
Gerber RS-274X (gerber-rs) + IPC-2581 + ODB++ Formats

A modern, pure-Rust EDA suite (KiCad + TSCircuit reimagined) built with Slint + Bevy. Fluid schematic/PCB editing, real-time 2D/3D canvas, programmatic code-first design, powerful DRC/ERC/DFM, and full export to Gerber RS-274X, IPC-2581 & ODB++. Fast, native, open-source hardware design that actually feels good to use. 🚀

*The next-generation open-source EDA experience — delightful, fast, fluid, and modern.*

---

## Documentation Index

### Core Architecture & Fundamentals

- [Project Structure & Management](./core-architecture/project-structure-management.md) - Unified project file format, directory structure
- [Unified Project File Format](./core-architecture/unified-project-file-format.md) - .hwt format specification, KiCAD compatibility
- [Circuit JSON as Intermediate Representation](./core-architecture/circuit-json-ir.md) - Universal data model for schematics, layouts, nets
- [Schematic Capture Workflow](./core-architecture/schematic-capture-workflow.md) - From abstract circuit to netlist generation
- [PCB Layout Workflow](./core-architecture/pcb-layout-workflow.md) - From netlist to physical board realization
- [Programmatic / Code-First Design](./core-architecture/programmatic-design.md) - Defining circuits via Rust code/macros

### Schematic Editor Concepts

- [Symbols & Libraries](./schematic-editor/symbols-libraries.md) - Custom + official symbols with pins, properties
- [Hierarchical & Multi-Sheet Schematics](./schematic-editor/hierarchical-schematics.md) - Sub-sheets, sheet pins, labels
- [Wiring & Connectivity](./schematic-editor/wiring-connectivity.md) - Wires, buses, labels, power symbols, net classes
- [Electrical Rules Check (ERC)](./schematic-editor/erc.md) - Pin connection validation, unconnected pins
- [Annotation & Reference Designators](./schematic-editor/annotation-reference-designators.md) - Automatic numbering, back-annotation
- [SPICE Simulation Integration](./schematic-editor/spice-simulation.md) - Netlist export and ngspice simulation

### PCB Layout Concepts

- [Footprints & Libraries](./pcb-layout/footprints-libraries.md) - Footprint assignment, pad definitions, 3D models
- [Component Placement](./pcb-layout/component-placement.md) - Manual drag/drop, auto-placement hints
- [Interactive Routing](./pcb-layout/interactive-routing.md) - Manual + push-and-shove router, differential pairs
- [Automatic Routing & Layout Engines](./pcb-layout/auto-routing.md) - pcbFlex / pcbGrid / pcbPack auto-layout
- [Copper Zones / Pours](./pcb-layout/copper-zones.md) - Filled regions, priority, thermals, clearance
- [Design Rule Check (DRC)](./pcb-layout/drc.md) - Clearance, connection, size, via violations
- [Multi-Layer Support](./pcb-layout/multi-layer.md) - Inner layers, blind/buried vias, stackup
- [Via & Via Stitching](./pcb-layout/via-stitching.md) - Plated, tented, microvias, ground stitching
- [Net Inspection & Highlighting](./pcb-layout/net-inspection.md) - Real-time net tracing, ratsnest display

### 3D Visualization & Mechanical Integration

- [3D PCB Viewer](./3d-visualization/3d-pcb-viewer.md) - Real-time 3D rendering with raytracing
- [STEP & 3D Model Export/Import](./3d-visualization/step-3d-models.md) - Mechanical fit checks, enclosure integration

### Manufacturing & Output Formats

- [Gerber RS-274X Export](./manufacturing-output/gerber-export.md) - Copper, mask, silkscreen, drill layers
- [IPC-2581 Export](./manufacturing-output/ipc2581-export.md) - Single-file intelligent format
- [ODB++ Export](./manufacturing-output/odbpp-export.md) - Folder-based format for professional fabs
- [BOM & Pick-and-Place Generation](./manufacturing-output/bom-pick-place.md) - Centroid files, component lists
- [Fabrication & Assembly Drawings](./manufacturing-output/fabrication-drawings.md) - Drawing sheets, title blocks

### Advanced & Productivity Features

- [Library Conventions & Quality Control](./advanced-features/library-conventions.md) - KLC-style guidelines
- [Design for Manufacturability (DFM) Checks](./advanced-features/dfm-checks.md) - Yield prediction, acute angles
- [Undo/Redo & Versioning](./advanced-features/undo-redo-versioning.md) - Command-based history, crash recovery
- [Command-Line Interface (CLI)](./advanced-features/cli.md) - Batch processing, export, automation
- [Calculator Tools](./advanced-features/calculator-tools.md) - Track width, via current, impedance, thermal overlay, test points
- [Thermal Simulation](./advanced-features/thermal-simulation.md) - Physics-based Joule heating, FEM solver, heat maps, radiation
- [Signal & Power Integrity](./advanced-features/signal-power-integrity.md) - S-parameters, eye diagrams, PDN impedance, crosstalk
- [Electromagnetic Simulation](./advanced-features/electromagnetic-simulation.md) - EMC/EMI, near-field, radiated emissions, shielding
- [Layout Properties & Constraints](./advanced-features/layout-properties-constraints.md) - Manual nudges, keep-out zones
- [Real-Time Preview & Iteration](./advanced-features/realtime-preview.md) - Live schematic-to-PCB sync
- [Real-Time Collaboration](./advanced-features/realtime-collaboration.md) - Multi-user editing, CRDT sync, design review
- [Variant Manager](./advanced-features/variant-manager.md) - What-if design exploration, cost comparison, variant BOMs
- [Digital Twin & AR Debug](./advanced-features/digital-twin-ar.md) - Live simulation, current flow, AR overlay, fault injection

### UX/UI Design

- [Main Window Layout](./ux-ui-design/main-window-layout.md) - Adaptive interface, canvas-first experience
- [Innovative Interaction Patterns](./ux-ui-design/innovative-interaction-patterns.md) - Magnet cursor, gesture router, shadow nudge
- [Visual Style Guidelines](./ux-ui-design/visual-style-guidelines.md) - Color palette, typography, motion design
- [Accessibility & Theming](./ux-ui-design/accessibility-and-theming.md) - Dark/light modes, color-blind support
- [Keyboard Shortcuts Reference](./ux-ui-design/keyboard-shortcuts-reference.md) - Complete shortcut catalog
- [Onboarding & First Experience](./ux-ui-design/onboarding-and-first-experience.md) - Tutorials, migration guides

### AI Integration

- [API Keys Configuration](./ai-integration/api-keys-configuration.md) - Secure AI provider setup, multi-provider support
- [Native AI Design Assistant](./ai-integration/native-ai-assistant.md) - First-party AI with direct tool access
- [AI-Powered Routing & Optimization](./ai-integration/ai-routing-optimization.md) - First-principles optimization, 5-10x efficiency
- [Generative AI Design](./advanced-features/generative-ai-design.md) - Spec-to-design, component selection, layout generation
- [Benchmarking Simulator](./ai-integration/benchmarking-simulator.md) - Real-time physics-based validation

### Appendices

- [Performance Targets](./appendices/performance-targets.md) - Response times, memory usage, benchmarks
- [Roadmap & Priorities](./appendices/roadmap-priorities.md) - Development phases, milestones
- [Comparison with KiCAD and TSCircuit](./appendices/comparison-with-kicad-and-tscircuit.md) - Feature matrix, philosophy

---

## Quick Start

```bash
# Install Hardware Tool
cargo install hardware-tool

# Create new project
hwt new my_project

# Open GUI
hwt open my_project.hwt

# Export Gerbers (CLI)
hwt export gerber my_board.hwt_pcb --output ./gerber/

# AI-assisted routing
hwt ai route --optimize signal-integrity
```

## Technology Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| UI Framework | Slint |
| 3D Rendering | Bevy |
| Gerber Export | gerber-rs |
| File Formats | Gerber RS-274X, IPC-2581, ODB++ |
| AI Integration | OpenAI, Anthropic, Google, Local (Ollama) |

## Core UX Philosophy

1. **Fluid Canvas-First Experience** — One continuous, zoomable, pannable workspace
2. **Context-Aware Adaptive Interface** — UI elements appear when and where needed
3. **Dual-Paradigm Mastery** — GUI and code-first workflows, deeply synchronized
4. **Visual Language of Feedback** — Rich, non-intrusive visual feedback everywhere
5. **Performance as a Feature** — Instant response even on very large designs
6. **Accessibility by Default** — Dark/light, high-contrast, color-blind friendly

## License

[License information here]
