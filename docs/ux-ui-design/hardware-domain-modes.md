# Hardware Domain Modes

## Overview

Hardware Tool uses **explicit domain modes** to provide a clean, focused UI for each hardware type. While the underlying architecture is unified ("One Hardware Tool That Does It All"), the UI adapts to show only relevant tools, panels, and options for the current domain.

**Default Mode:** PCB (most common use case)

---

## Mode Switcher UI

### Location

The mode switcher is prominently placed in the **title bar**, always visible regardless of current view.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ⚙ Hardware Tool    [Smart Sensor v1.2]                              [─][□][✕]│
├──────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────────────────┐  │
│ │ [🔲 PCB ▼] │ [📐 Schematic] [🔲 Layout] [🎲 3D] [💻 Code]              │  │
│ └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Domain: PCB ────────────────────────────────────────────────────────────────│
└──────────────────────────────────────────────────────────────────────────────┘
```

### Mode Dropdown

```
┌─────────────────────────────────────┐
│ Select Hardware Domain              │
├─────────────────────────────────────┤
│ ● 🔲 PCB Design          (Default) │
│ ○ 🔷 IC Design                     │
│ ○ ⚛️ Quantum Hardware              │
│ ○ 📡 MEMS & Sensors                │
│ ○ 📶 RF & Photonics                │
│ ○ 📦 Advanced Packaging            │
├─────────────────────────────────────┤
│ [+ Create New Domain Project]       │
└─────────────────────────────────────┘
```

---

## Domain Modes

### Mode Definitions

| Mode | Icon | Color | Description | Default View |
|------|------|-------|-------------|--------------|
| **PCB** | 🔲 | Copper (#B87333) | Printed circuit boards | Schematic |
| **IC** | 🔷 | Blue (#3498DB) | Integrated circuits | RTL Editor |
| **Quantum** | ⚛️ | Purple (#9B59B6) | Quantum processors | Circuit Editor |
| **MEMS** | 📡 | Teal (#1ABC9C) | Micro-electromechanical | Device Editor |
| **RF** | 📶 | Orange (#E67E22) | RF & Photonics | Schematic |
| **Packaging** | 📦 | Gray (#7F8C8D) | Advanced packaging | Die Editor |

### Mode Color Accents

Each mode has a subtle color accent applied to:
- Title bar border
- Active tab indicators
- Selection highlights
- Mode-specific icons

---

## Shared UI Elements (All Modes)

These UI elements appear in **every domain mode** because they use shared architecture:

### Title Bar (Always Visible)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Mode Dropdown ▼] │ [View Tabs] │ [Search] │ [Undo/Redo] │ [Save] │ [Menu]  │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Button | Icon | Shortcut | Shared Module |
|--------|------|----------|---------------|
| Mode Dropdown | 🔲/🔷/⚛️/📡/📶/📦 | `Ctrl+M` | — |
| Search | 🔍 | `Ctrl+K` | Command Palette |
| Undo | ↩️ | `Ctrl+Z` | Undo/Redo |
| Redo | ↪️ | `Ctrl+Y` | Undo/Redo |
| Save | 💾 | `Ctrl+S` | Project Management |
| Menu | ☰ | `Alt` | — |

### Left Sidebar - Shared Panels

```
┌─────────────────────┐
│ 🔍 Search...        │  ← Shared: Library Browser
├─────────────────────┤
│ ▼ Libraries         │  ← Shared: Library Architecture
│   📁 [Domain libs]  │
├─────────────────────┤
│ ▼ Project           │  ← Shared: Project Management
│   📄 Files          │
│   📊 Hierarchy      │
├─────────────────────┤
│ ▼ History           │  ← Shared: Undo/Redo
│   ↩️ Recent actions │
└─────────────────────┘
```

### Right Sidebar - Shared Panels

```
┌─────────────────────┐
│ Properties          │  ← Shared: Property Inspector
├─────────────────────┤
│ DRC Results         │  ← Shared: DRC Architecture
│ ✗ 3 Errors          │
│ ⚠ 5 Warnings        │
├─────────────────────┤
│ Sync Status         │  ← Shared: Real-Time Sync
│ 🟢 Synchronized     │
└─────────────────────┘
```

### Bottom Bar (Always Visible)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Grid: 0.1mm] [Snap: On] │ X: 45.2 Y: 32.1 │ Sel: 3 items │ 🟢 Sync │ 60 FPS│
└──────────────────────────────────────────────────────────────────────────────┘
```

| Element | Shared Module |
|---------|---------------|
| Grid/Snap | Layout Infrastructure |
| Coordinates | Layout Infrastructure |
| Selection | Core Architecture |
| Sync Status | Real-Time Sync |
| Performance | Core Architecture |

### Floating Panels (All Modes)

| Panel | Shortcut | Shared Module |
|-------|----------|---------------|
| 3D Preview | `F8` | 3D Viewer Architecture |
| Simulation Jobs | `F9` | Simulation Architecture |
| Command Palette | `Ctrl+K` | CLI |
| Calculator | `F2` | Calculator Tools |

---

## Domain-Specific UI Elements

### PCB Mode (Default)

#### View Tabs
```
[📐 Schematic] [🔲 Layout] [🎲 3D] [💻 Code]
```

#### Left Sidebar - PCB Specific
```
┌─────────────────────┐
│ ▼ Symbols           │  ← PCB: Schematic symbols
│ ▼ Footprints        │  ← PCB: Physical footprints
├─────────────────────┤
│ ▼ Net Classes       │  ← PCB: Net classification
│   Power             │
│   Signal            │
│   High-Speed        │
└─────────────────────┘
```

#### Toolbar - PCB Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Schematic: [Wire] [Bus] [Label] [Power] [No-Connect] [Annotate]              │
│ Layout:    [Route] [Via] [Zone] [Keepout] [Dimension] [Drill Table]          │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| Wire | ➖ | `W` | Draw wire connection |
| Bus | ═ | `B` | Draw bus |
| Label | 🏷️ | `L` | Add net label |
| Power | ⚡ | `P` | Add power symbol |
| Route | 🛤️ | `X` | Interactive routing |
| Via | ⭕ | `V` | Place via |
| Zone | ▢ | `Z` | Draw copper zone |

---

### IC Mode

#### View Tabs
```
[📝 RTL] [🔷 Schematic] [📐 Layout] [🎲 3D] [💻 Code]
```

#### Left Sidebar - IC Specific
```
┌─────────────────────┐
│ ▼ Standard Cells    │  ← IC: Cell library
│   INV_X1            │
│   NAND2_X1          │
│   DFF_X1            │
├─────────────────────┤
│ ▼ IP Blocks         │  ← IC: Hard macros
│   SRAM_4K           │
│   PLL_1G            │
├─────────────────────┤
│ ▼ PDK               │  ← IC: Process design kit
│   sky130            │
│   gf180mcu          │
└─────────────────────┘
```

#### Toolbar - IC Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ RTL:     [Module] [Port] [Wire] [Assign] [Always] [Instance]                 │
│ Layout:  [Cell] [Route] [Via] [Fill] [Blockage] [Pin]                        │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| Module | 📦 | `M` | Create module |
| Port | 🔌 | `P` | Add port |
| Cell | 🔷 | `C` | Place standard cell |
| Fill | ▦ | `F` | Metal fill |
| Blockage | 🚫 | `K` | Placement blockage |

---

### Quantum Mode

#### View Tabs
```
[⚛️ Circuit] [📐 Layout] [🎲 3D] [📊 Simulation] [💻 Code]
```

#### Left Sidebar - Quantum Specific
```
┌─────────────────────┐
│ ▼ Gates             │  ← Quantum: Gate library
│   Hadamard (H)      │
│   CNOT              │
│   Pauli-X/Y/Z       │
│   T Gate            │
├─────────────────────┤
│ ▼ Qubits            │  ← Quantum: Qubit types
│   Transmon          │
│   Fluxonium         │
│   Xmon              │
├─────────────────────┤
│ ▼ Control           │  ← Quantum: Control elements
│   Readout resonator │
│   Coupler           │
│   Drive line        │
└─────────────────────┘
```

#### Toolbar - Quantum Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Circuit: [Qubit] [Gate] [Measure] [Barrier] [Reset] [Condition]              │
│ Layout:  [Transmon] [CPW] [Resonator] [Coupler] [Air Bridge]                 │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| Qubit | ⚛️ | `Q` | Add qubit |
| Gate | 🚪 | `G` | Place gate |
| Measure | 📏 | `M` | Add measurement |
| Transmon | ⬡ | `T` | Place transmon |
| CPW | ═ | `C` | Draw CPW trace |

---

### MEMS Mode

#### View Tabs
```
[📐 Device] [🔧 Mechanical] [⚡ Electrical] [🎲 3D] [💻 Code]
```

#### Left Sidebar - MEMS Specific
```
┌─────────────────────┐
│ ▼ Structures        │  ← MEMS: Mechanical structures
│   Beam              │
│   Plate             │
│   Spring            │
│   Comb drive        │
├─────────────────────┤
│ ▼ Anchors           │  ← MEMS: Fixed points
│   Square anchor     │
│   Round anchor      │
├─────────────────────┤
│ ▼ Process           │  ← MEMS: Fabrication
│   PolyMUMPs         │
│   SOIMUMPs          │
│   PiezoMUMPs        │
└─────────────────────┘
```

#### Toolbar - MEMS Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Device:  [Beam] [Plate] [Spring] [Anchor] [Etch Hole] [Dimple]               │
│ Layout:  [Poly1] [Poly2] [Metal] [Oxide] [Release]                           │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| Beam | ═ | `B` | Draw beam |
| Plate | ▢ | `P` | Draw plate |
| Spring | 〰️ | `S` | Add spring |
| Etch Hole | ⭕ | `E` | Add etch hole |
| Dimple | • | `D` | Add anti-stiction dimple |

---

### RF Mode

#### View Tabs
```
[📐 Schematic] [📶 Layout] [🎲 3D] [📊 S-Params] [💻 Code]
```

#### Left Sidebar - RF Specific
```
┌─────────────────────┐
│ ▼ Components        │  ← RF: RF components
│   Transmission line │
│   Stub              │
│   Coupler           │
│   Filter            │
├─────────────────────┤
│ ▼ S-Parameters      │  ← RF: Measured data
│   Amplifier.s2p     │
│   Filter.s4p        │
├─────────────────────┤
│ ▼ Substrates        │  ← RF: Material stack
│   FR4               │
│   Rogers 4350       │
│   Alumina           │
└─────────────────────┘
```

#### Toolbar - RF Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Schematic: [TLine] [Stub] [Coupler] [Port] [Ground] [S-Param Block]          │
│ Layout:    [Microstrip] [CPW] [Via Fence] [Taper] [Bend] [Matching]          │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| TLine | ═ | `T` | Transmission line |
| Stub | ⊥ | `S` | Add stub |
| Port | ◉ | `P` | Add port |
| Microstrip | ━ | `M` | Draw microstrip |
| Via Fence | ⋮ | `V` | Add via fence |

---

### Packaging Mode

#### View Tabs
```
[📦 Die Map] [📐 Layout] [🎲 3D] [🌡️ Thermal] [💻 Code]
```

#### Left Sidebar - Packaging Specific
```
┌─────────────────────┐
│ ▼ Dies              │  ← Packaging: Die library
│   Compute die       │
│   HBM3 stack        │
│   IO die            │
├─────────────────────┤
│ ▼ Interconnects     │  ← Packaging: Connections
│   Microbump         │
│   TSV               │
│   RDL               │
├─────────────────────┤
│ ▼ Package Types     │  ← Packaging: Templates
│   2.5D Interposer   │
│   3D Stack          │
│   Fan-out           │
└─────────────────────┘
```

#### Toolbar - Packaging Specific
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Die Map:  [Die] [Bump Map] [TSV Array] [RDL] [Keep-out]                      │
│ Layout:   [Interposer] [Substrate] [Via] [Trace] [Ball]                      │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Tool | Icon | Shortcut | Description |
|------|------|----------|-------------|
| Die | 🔷 | `D` | Place die |
| Bump Map | ⬡ | `B` | Define bump array |
| TSV | ⭕ | `T` | Place TSV |
| RDL | ━ | `R` | Draw redistribution layer |
| Ball | ● | `A` | Place BGA ball |

---

## Shared vs. Domain-Specific Summary

### Shared Buttons (All Modes)

| Category | Buttons |
|----------|---------|
| **File** | New, Open, Save, Export, Import |
| **Edit** | Undo, Redo, Cut, Copy, Paste, Delete |
| **View** | Zoom In/Out, Fit, Pan, Grid, Layers |
| **Tools** | Select, Measure, Annotate, Search |
| **Panels** | Library, Properties, DRC, 3D Preview, Simulation |
| **Help** | Documentation, Shortcuts, About |

### Domain-Specific Buttons

| Mode | Unique Buttons |
|------|----------------|
| **PCB** | Wire, Bus, Route, Via, Zone, Copper Pour, Drill Table |
| **IC** | Module, Port, Cell, Fill, Blockage, Timing, Power |
| **Quantum** | Qubit, Gate, Measure, Barrier, Transmon, CPW, Resonator |
| **MEMS** | Beam, Plate, Spring, Anchor, Etch Hole, Dimple, Release |
| **RF** | TLine, Stub, Coupler, Microstrip, Via Fence, Matching |
| **Packaging** | Die, Bump Map, TSV, RDL, Interposer, Ball |

---

## Mode Switching Behavior

### Switching Modes

```rust
ModeSwitcher {
    // Current state
    current_mode: HardwareMode::PCB,
    
    // Switching behavior
    on_switch: ModeSwitchBehavior {
        // Save current state
        save_view_state: true,
        save_selection: false,
        
        // Transition
        animate_transition: true,
        transition_duration: 0.2,  // seconds
        
        // Load new mode
        restore_last_view: true,
        show_welcome_if_empty: true,
    },
    
    // Keyboard shortcut
    shortcut: Key::Ctrl_M,
    quick_switch: vec![
        (Key::Ctrl_1, HardwareMode::PCB),
        (Key::Ctrl_2, HardwareMode::IC),
        (Key::Ctrl_3, HardwareMode::Quantum),
        (Key::Ctrl_4, HardwareMode::MEMS),
        (Key::Ctrl_5, HardwareMode::RF),
        (Key::Ctrl_6, HardwareMode::Packaging),
    ],
}
```

### Mode Memory

Each mode remembers its state:
- Last active view (Schematic, Layout, 3D, etc.)
- Panel positions and sizes
- Zoom level and pan position
- Selected items
- Tool settings

---

## Keyboard Shortcuts

### Global (All Modes)

| Shortcut | Action |
|----------|--------|
| `Ctrl+M` | Open mode switcher |
| `Ctrl+1` | Switch to PCB mode |
| `Ctrl+2` | Switch to IC mode |
| `Ctrl+3` | Switch to Quantum mode |
| `Ctrl+4` | Switch to MEMS mode |
| `Ctrl+5` | Switch to RF mode |
| `Ctrl+6` | Switch to Packaging mode |

### View Switching (Within Mode)

| Shortcut | Action |
|----------|--------|
| `F5` | Schematic/Circuit/Device view |
| `F6` | Layout view |
| `F7` | Code view |
| `F8` | 3D view |
| `F9` | Simulation view |

---

## Configuration

```rust
ModeConfig {
    // Default mode for new projects
    default_mode: HardwareMode::PCB,
    
    // Remember last mode per project
    remember_per_project: true,
    
    // Show mode indicator
    show_mode_indicator: true,
    mode_indicator_position: Position::TitleBar,
    
    // Color coding
    use_mode_colors: true,
    color_intensity: 0.3,  // Subtle accent
    
    // Toolbar customization
    allow_toolbar_customization: true,
    show_unused_tools: false,
}
```

---

## Related Topics

- [Main Window Layout](./main-window-layout.md)
- [Shared Module Consolidation](../core-architecture/shared-module-consolidation.md)
- [Keyboard Shortcuts](./keyboard-shortcuts.md)
- [Toolbar Customization](./toolbar-customization.md)
