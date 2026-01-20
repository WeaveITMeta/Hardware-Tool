# Main Window Layout

## Overview

Hardware Tool's interface is designed around a **fluid canvas-first experience** — one continuous, zoomable, pannable workspace (inspired by Figma × Blender × modern game editors). The UI adapts contextually, showing elements exactly when and where needed while minimizing permanent chrome.

## Layout Architecture

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Title Bar / Global Controls ─────────────────────────────────────────────────│
│ [Project Name]  ◀ Back to Projects   Mode: Schematic / PCB / 3D / Code      │
│──────────────────────────────────────────────────────────────────────────────│
│ Left Sidebar (collapsible / pinned)                                          │
│ ┌─────────────────────┐                                                      │
│ │ Library Browser     │  ── Contextual panels (changes with mode)           │
│ │ Components          │                                                      │
│ │ Footprints          │  • Schematic: Symbol palette, net classes           │
│ │ Recent Used         │  • PCB: Placement tools, routing modes, zones       │
│ │ Favorites / Snippets│  • 3D: Material overrides, camera bookmarks         │
│ └─────────────────────┘                                                      │
│                                                                              │
│ Main Canvas (Bevy powered – 99% of screen real-estate)                       │
│   • Infinite zoom / pan                                                      │
│   • Multi-view split (horizontal/vertical/diagonal) support                  │
│   • Floating mini-map in corner when zoomed in                               │
│                                                                              │
│ Right Sidebar (floating / dockable / context-sensitive)                      │
│   • Properties inspector (very powerful, searchable)                         │
│   • Net inspector / ratsnest manager                                         │
│   • DRC / ERC / DFM live report (collapsible severity levels)               │
│   • Command palette history / quick actions                                  │
│                                                                              │
│ Bottom Bar (thin – status + tools)                                           │
│   Mode switcher • Grid / Snap settings • Coordinates • Selection info        │
│   • Live performance stats (FPS, memory, render time)                        │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Title Bar

### Global Controls

```rust
TitleBar {
    // Project info
    project_name: "Smart Sensor Board",
    project_path: "./projects/sensor",
    unsaved_indicator: true,  // Shows • when unsaved
    
    // Navigation
    back_to_projects: true,
    recent_projects: vec![...],
    
    // Mode switcher (prominent)
    modes: vec![
        Mode::Schematic,
        Mode::Pcb,
        Mode::ThreeD,
        Mode::Code,
    ],
    current_mode: Mode::Pcb,
}
```

### Mode Indicator

| Mode | Icon | Color Accent |
|------|------|--------------|
| Schematic | 📐 | Cyan (#00D4FF) |
| PCB | 🔲 | Copper (#B87333) |
| 3D | 🎲 | Purple (#9B59B6) |
| Code | 💻 | Green (#00FF9D) |

## Left Sidebar

### Context-Adaptive Panels

The left sidebar transforms based on current mode:

#### Schematic Mode

```
┌─────────────────────┐
│ 🔍 Search symbols...│
├─────────────────────┤
│ ▼ Recent            │
│   LM1117            │
│   STM32F4           │
│   0603 Resistor     │
├─────────────────────┤
│ ▼ Favorites         │
│   ⭐ Power Section  │
│   ⭐ Decoupling     │
├─────────────────────┤
│ ▼ Libraries         │
│   📁 Device         │
│   📁 MCU_ST         │
│   📁 Connector      │
├─────────────────────┤
│ ▼ Code Snippets     │
│   📄 RC Filter      │
│   📄 Voltage Div    │
└─────────────────────┘
```

#### PCB Mode

```
┌─────────────────────┐
│ 🔍 Search footprints│
├─────────────────────┤
│ ▼ Placement Tools   │
│   🎯 Auto-place     │
│   📐 Align          │
│   📏 Distribute     │
├─────────────────────┤
│ ▼ Routing           │
│   ✏️ Interactive    │
│   🤖 Auto-route     │
│   ⚡ Diff Pair      │
├─────────────────────┤
│ ▼ Zones             │
│   🟩 GND Pour       │
│   🟨 VCC Pour       │
│   ➕ New Zone       │
├─────────────────────┤
│ ▼ Layers            │
│   [✓] F.Cu          │
│   [✓] B.Cu          │
│   [ ] In1.Cu        │
└─────────────────────┘
```

### Sidebar Behavior

```rust
LeftSidebar {
    // State
    collapsed: false,
    pinned: true,
    width: 280,  // pixels
    
    // Behavior
    auto_collapse_on_canvas_focus: false,
    remember_scroll_position: true,
    
    // Panels
    panels: vec![
        Panel::Search { always_visible: true },
        Panel::Recent { collapsed: false },
        Panel::Favorites { collapsed: true },
        Panel::Libraries { collapsed: false },
    ],
}
```

## Main Canvas

### Bevy-Powered Rendering

The canvas is the heart of Hardware Tool — a GPU-accelerated infinite workspace:

```rust
Canvas {
    // Rendering
    renderer: Renderer::Bevy,
    antialiasing: Antialiasing::Msaa4x,
    
    // Navigation
    zoom: ZoomConfig {
        min: 0.01,    // 1% - see entire large board
        max: 100.0,   // 10000% - sub-mil precision
        smooth: true,
        wheel_sensitivity: 1.2,
    },
    pan: PanConfig {
        middle_mouse: true,
        space_drag: true,
        edge_scroll: true,
        inertia: true,
    },
    
    // Grid
    grid: GridConfig {
        visible: true,
        adaptive: true,  // Changes density with zoom
        snap: true,
        size: 0.1,  // mm
    },
}
```

### Multi-View Split

```
┌─────────────────┬─────────────────┐
│                 │                 │
│   Schematic     │      PCB        │
│                 │                 │
├─────────────────┼─────────────────┤
│                 │                 │
│      3D         │      Code       │
│                 │                 │
└─────────────────┴─────────────────┘
```

### Mini-Map

```rust
MiniMap {
    enabled: true,
    position: Corner::BottomRight,
    size: (200, 150),
    opacity: 0.8,
    
    // Show when
    show_when_zoomed: true,
    zoom_threshold: 2.0,
    
    // Interaction
    click_to_navigate: true,
    drag_viewport: true,
}
```

### Floating 3D Preview Panel

A real-time 3D preview panel (powered by Bevy) floats in the **top-right corner** of the main canvas, providing constant mechanical awareness without switching modes.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   Main Canvas (Schematic or PCB)              ┌─────────────────────────┐  │
│                                               │ 3D Preview    [⬜][✕]  │  │
│                                               ├─────────────────────────┤  │
│                                               │                         │  │
│                                               │  [Real-time 3D render]  │  │
│                                               │                         │  │
│                                               └─────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Panel Controls

| Button | Icon | Action |
|--------|------|--------|
| **Maximize** | `⬜` | Expand 3D view to full canvas area |
| **Restore** | `❐` | Return to floating panel in top-right |
| **Close** | `✕` | Hide panel (reopen via View menu) |

#### Configuration

```rust
Floating3DPreview {
    // Position
    position: PanelPosition::TopRight,
    
    // Default size (floating)
    width: 320,
    height: 240,
    
    // Controls
    show_maximize_button: true,
    show_close_button: true,
    
    // Behavior
    resizable: true,
    draggable: false,  // Fixed to top-right
    
    // Rendering
    renderer: Renderer::Bevy,
    quality: QualityPreset::Medium,  // Balance performance
    
    // Sync
    follow_cursor: true,
    auto_rotate: false,
}
```

#### Accessing Hidden Panel

When closed via the `✕` button, reopen through:

- **Menu**: `View → 3D Preview Panel`
- **Keyboard**: `F8`
- **Command Palette**: `Ctrl+K` → "3D Preview"

## Right Sidebar

### Properties Inspector

```
┌─────────────────────────┐
│ Properties        [📌]  │
├─────────────────────────┤
│ 🔍 Search properties... │
├─────────────────────────┤
│ Component: U1           │
│ ─────────────────────── │
│ Reference:  [U1      ]  │
│ Value:      [STM32F4 ]  │
│ Footprint:  [LQFP-64 ]  │
│ ─────────────────────── │
│ Position                │
│   X: [50.00    ] mm     │
│   Y: [40.00    ] mm     │
│   Rotation: [0  ] °     │
│   Layer: [Top   ] ▼     │
│ ─────────────────────── │
│ ▼ Custom Fields         │
│   MPN: STM32F405RGT6    │
│   Manufacturer: ST      │
└─────────────────────────┘
```

### Live DRC/ERC/DFM Panel

```
┌─────────────────────────┐
│ Design Checks     [🔄]  │
├─────────────────────────┤
│ ▼ Errors (3)       🔴   │
│   ⚠ Clearance violation │
│   ⚠ Unconnected pad     │
│   ⚠ Track width         │
├─────────────────────────┤
│ ▼ Warnings (12)    🟡   │
│   ... (collapsed)       │
├─────────────────────────┤
│ ▶ Info (5)         🔵   │
├─────────────────────────┤
│ DFM Score: 94/100  🟢   │
│ [Run Full Check]        │
└─────────────────────────┘
```

## Bottom Bar

### Status Bar Layout

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ [Schematic│PCB│3D│Code]  Grid: 0.1mm  Snap: On  │ X: 45.20  Y: 32.10  │     │
│                                                  │ Selected: 3 items   │     │
│ 🟢 Ready                                         │ FPS: 144  Mem: 245MB│     │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Components

```rust
BottomBar {
    // Left section
    mode_switcher: true,
    grid_settings: true,
    snap_toggle: true,
    
    // Center section
    coordinates: CoordinateDisplay {
        format: "X: {x:.2}  Y: {y:.2}",
        units: Units::Millimeters,
        origin_indicator: true,
    },
    selection_info: true,
    
    // Right section
    status_message: true,
    performance_stats: PerformanceStats {
        fps: true,
        memory: true,
        render_time: false,  // Toggle with click
    },
}
```

## Responsive Behavior

### Screen Size Adaptation

| Screen Width | Layout Adjustment |
|--------------|-------------------|
| < 1280px | Sidebars auto-collapse, floating panels |
| 1280-1920px | Standard layout |
| > 1920px | Wider sidebars, more panel space |
| Ultra-wide | Multi-view default |

### Touch Support

```rust
TouchConfig {
    enabled: true,
    
    // Gestures
    pinch_zoom: true,
    two_finger_pan: true,
    long_press_context: true,
    
    // Touch-friendly sizing
    min_touch_target: 44,  // pixels
}
```

## Related Topics

- [Innovative Interaction Patterns](./innovative-interaction-patterns.md)
- [Visual Style Guidelines](./visual-style-guidelines.md)
- [Keyboard Shortcuts Reference](./keyboard-shortcuts-reference.md)
