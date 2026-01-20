# Comparison with KiCAD and TSCircuit

## Overview

Hardware Tool combines the best aspects of KiCAD's mature GUI-based workflow with TSCircuit's innovative code-first approach, while adding unique capabilities enabled by Rust and modern rendering technology.

## Feature Comparison Matrix

### Core Capabilities

| Feature | Hardware Tool | KiCAD | TSCircuit |
|---------|---------------|-------|-----------|
| GUI Editor | ✅ Full | ✅ Full | ⚠️ Limited |
| Code-First Design | ✅ Native | ❌ No | ✅ Native |
| Schematic Capture | ✅ Yes | ✅ Yes | ✅ Yes |
| PCB Layout | ✅ Yes | ✅ Yes | ✅ Yes |
| 3D Visualization | ✅ Bevy (GPU) | ✅ OpenGL | ⚠️ Basic |
| Real-time Sync | ✅ Yes | ⚠️ Manual | ✅ Yes |
| Cross-platform | ✅ Yes | ✅ Yes | ✅ Yes |

### File Formats

| Format | Hardware Tool | KiCAD | TSCircuit |
|--------|---------------|-------|-----------|
| Native Format | .hwt (TOML/JSON) | .kicad_* | TypeScript |
| Gerber RS-274X | ✅ Yes | ✅ Yes | ✅ Yes |
| IPC-2581 | ✅ Yes | ⚠️ Plugin | ⚠️ Limited |
| ODB++ | ✅ Yes | ⚠️ Plugin | ❌ No |
| STEP Export | ✅ Yes | ✅ Yes | ⚠️ Limited |
| Circuit JSON | ✅ Native | ❌ No | ✅ Native |

### Routing & Layout

| Feature | Hardware Tool | KiCAD | TSCircuit |
|---------|---------------|-------|-----------|
| Interactive Router | ✅ Push-and-shove | ✅ Push-and-shove | ❌ No |
| Auto-router | ✅ AI-powered | ⚠️ External | ✅ Built-in |
| Differential Pairs | ✅ Yes | ✅ Yes | ✅ Yes |
| Length Matching | ✅ Yes | ✅ Yes | ✅ Yes |
| Via Stitching | ✅ Yes | ✅ Yes | ⚠️ Limited |
| Copper Zones | ✅ Live preview | ✅ Yes | ✅ Yes |

### Design Verification

| Feature | Hardware Tool | KiCAD | TSCircuit |
|---------|---------------|-------|-----------|
| DRC | ✅ Real-time | ✅ Batch | ✅ Yes |
| ERC | ✅ Real-time | ✅ Batch | ✅ Yes |
| DFM Checks | ✅ Built-in | ⚠️ Plugin | ❌ No |
| SPICE Simulation | ✅ Integrated | ✅ Integrated | ❌ No |

### AI & Automation

| Feature | Hardware Tool | KiCAD | TSCircuit |
|---------|---------------|-------|-----------|
| AI Integration | ✅ Native | ❌ No | ❌ No |
| Natural Language | ✅ Yes | ❌ No | ❌ No |
| AI Placement | ✅ Yes | ❌ No | ⚠️ Algorithm |
| AI Routing | ✅ Yes | ❌ No | ✅ Algorithm |
| Design Optimization | ✅ First-principles | ❌ No | ⚠️ Limited |

## Philosophy Comparison

### KiCAD Philosophy

```
Traditional GUI-first approach:
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Eeschema   │ ──► │   Pcbnew    │ ──► │   Gerber    │
│ (Schematic) │     │    (PCB)    │     │  (Output)   │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │
       └───── Manual ──────┘
           Annotation
```

**Strengths:**
- Mature, battle-tested
- Huge community
- Extensive libraries
- Free and open source

**Limitations:**
- Separate applications
- Manual sync required
- Python scripting (slower)
- Traditional UX patterns

### TSCircuit Philosophy

```
Code-first approach:
┌─────────────────────────────────────────────────────┐
│                  TypeScript Code                     │
│  const circuit = new Circuit()                       │
│  circuit.add(resistor({ value: "10k" }))            │
└─────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Schematic  │     │     PCB     │     │   Output    │
│   (Auto)    │     │   (Auto)    │     │   (Auto)    │
└─────────────┘     └─────────────┘     └─────────────┘
```

**Strengths:**
- Programmatic control
- Version control friendly
- Reusable modules
- Fast iteration

**Limitations:**
- Limited GUI
- Learning curve
- Smaller community
- Less visual feedback

### Hardware Tool Philosophy

```
Unified dual-paradigm approach:
┌─────────────────────────────────────────────────────┐
│              Unified Workspace (Slint + Bevy)        │
│  ┌─────────────────────────────────────────────┐    │
│  │     GUI Editor  ◄──── Real-time ────►  Code │    │
│  │    (Visual)      │      Sync        (Rust)  │    │
│  └─────────────────────────────────────────────┘    │
│                         │                            │
│                    Circuit JSON                      │
│                    (Universal IR)                    │
│                         │                            │
│  ┌──────────┬──────────┬──────────┬──────────┐     │
│  │Schematic │   PCB    │    3D    │    AI    │     │
│  └──────────┴──────────┴──────────┴──────────┘     │
└─────────────────────────────────────────────────────┘
```

**Strengths:**
- Best of both worlds
- Native AI integration
- Modern UX patterns
- Rust performance
- Real-time everything

## Performance Comparison

| Metric | Hardware Tool | KiCAD | TSCircuit |
|--------|---------------|-------|-----------|
| Language | Rust | C++ | TypeScript |
| Rendering | Bevy (GPU) | OpenGL | Canvas/WebGL |
| Startup Time | < 2s | 3-5s | < 1s (web) |
| Large Design FPS | 60+ | 30-60 | 30-60 |
| Memory (1000 comp) | ~500MB | ~800MB | ~400MB |
| DRC Speed | Real-time | Batch | Real-time |

## Migration Paths

### From KiCAD

```rust
// Import KiCAD project
hwt import kicad ./my_project.kicad_pro

// Automatic conversion:
// - .kicad_sch → .hwt_sch
// - .kicad_pcb → .hwt_pcb
// - Libraries → .hwt_lib
// - Settings preserved
```

### From TSCircuit

```rust
// Import TSCircuit project
hwt import tscircuit ./circuit.ts

// Conversion:
// - TypeScript → Rust code (optional)
// - Circuit JSON preserved
// - Layout data imported
```

### To KiCAD (Export)

```rust
// Export for KiCAD users
hwt export kicad ./output/

// Generates:
// - .kicad_pro
// - .kicad_sch
// - .kicad_pcb
```

## Use Case Recommendations

### Choose Hardware Tool When:

- You want both GUI and code-first workflows
- AI-assisted design is important
- Real-time feedback is critical
- Modern UX matters to you
- You need first-principles optimization

### Choose KiCAD When:

- You need maximum stability
- Large existing library investment
- Team already trained on KiCAD
- Plugin ecosystem is important
- Conservative, proven approach preferred

### Choose TSCircuit When:

- Pure code-first is preferred
- Web-based workflow needed
- TypeScript ecosystem integration
- Rapid prototyping focus
- Minimal GUI acceptable

## Interoperability

### Hardware Tool ↔ KiCAD

| Direction | Support |
|-----------|---------|
| Import .kicad_pro | ✅ Full |
| Import .kicad_sch | ✅ Full |
| Import .kicad_pcb | ✅ Full |
| Import libraries | ✅ Full |
| Export to KiCAD | ✅ Full |

### Hardware Tool ↔ TSCircuit

| Direction | Support |
|-----------|---------|
| Import Circuit JSON | ✅ Native |
| Export Circuit JSON | ✅ Native |
| Import TypeScript | ⚠️ Partial |
| Code generation | ✅ Rust |

## Related Topics

- [Project Structure](../core-architecture/project-structure-management.md)
- [Circuit JSON IR](../core-architecture/circuit-json-ir.md)
- [Programmatic Design](../core-architecture/programmatic-design.md)
