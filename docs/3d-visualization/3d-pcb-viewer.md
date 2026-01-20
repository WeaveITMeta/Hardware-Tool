# 3D PCB Viewer

## Overview

Hardware Tool includes a real-time 3D PCB viewer powered by Bevy, providing photorealistic visualization of board designs with component models, copper layers, and mechanical features. The viewer supports raytracing for high-quality renders.

## Floating 3D Reference Panel

The 3D viewer appears as a **floating panel in the top-right corner** of the main workspace, providing constant mechanical awareness while working in schematic or PCB views.

### Panel Controls

```
┌─────────────────────────────────────────┐
│ 3D Preview                    [⬜][✕]  │  ← Maximize | Close
├─────────────────────────────────────────┤
│                                         │
│   ┌─────────────────────────────────┐   │
│   │                                 │   │
│   │   [Real-time 3D PCB render]     │   │
│   │                                 │   │
│   │                                 │   │
│   └─────────────────────────────────┘   │
│                                         │
└─────────────────────────────────────────┘
```

### Window Controls

| Button | Icon | Action |
|--------|------|--------|
| **Maximize** | `⬜` | Expand to full viewport, replaces main canvas |
| **Restore** | `❐` | Return to floating panel (top-right corner) |
| **Close** | `✕` | Hide panel completely |

### Configuration

```rust
Floating3DPanel {
    // Default state
    visible: true,
    position: PanelPosition::TopRight,
    
    // Size (when floating)
    width: 320,
    height: 240,
    
    // Behavior
    resizable: true,
    min_size: (200, 150),
    max_size: (600, 450),
    
    // State persistence
    remember_size: true,
    remember_visibility: true,
}
```

### Maximize/Restore Behavior

```rust
MaximizeBehavior {
    // When maximized
    maximized: MaximizedState {
        // Takes over main canvas area
        replace_main_canvas: true,
        
        // Animation
        animate_transition: true,
        transition_duration: 0.2,  // seconds
        
        // Toolbar
        show_full_toolbar: true,
    },
    
    // When restored
    restored: RestoredState {
        // Return to floating position
        position: PanelPosition::TopRight,
        
        // Restore previous size
        restore_size: true,
    },
}
```

### Accessing Hidden Panel

When closed, the 3D viewer can be reopened via:

- **Menu**: `View → 3D Preview Panel` or `View → Panels → 3D Preview`
- **Keyboard**: `F8` (toggle)
- **Command Palette**: `Ctrl+K` → "Show 3D Preview"

```rust
ViewMenu {
    panels: vec![
        MenuItem::Toggle {
            label: "3D Preview Panel",
            shortcut: Key::F8,
            checked: panel_visible,
            action: Action::Toggle3DPanel,
        },
        // ... other panel toggles
    ],
}
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `F8` | Toggle 3D panel visibility |
| `Shift+F8` | Maximize/restore 3D panel |
| `Ctrl+Shift+3` | Focus 3D panel |

## Viewer Features

### Real-Time Rendering

```rust
Viewer3D {
    // Rendering engine
    engine: RenderEngine::Bevy,
    
    // Quality settings
    quality: QualityPreset::High,
    
    // Features
    shadows: true,
    reflections: true,
    ambient_occlusion: true,
}
```

### Raytracing Mode

```rust
RaytracingConfig {
    enabled: true,
    
    // Quality
    samples_per_pixel: 64,
    max_bounces: 4,
    
    // Performance
    progressive: true,
    denoise: true,
}
```

## Navigation Controls

### Mouse Controls

| Action | Control |
|--------|---------|
| Rotate | Left-click + drag |
| Pan | Middle-click + drag |
| Zoom | Scroll wheel |
| Select | Left-click |
| Context menu | Right-click |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Home` | Reset view |
| `T` | Top view |
| `B` | Bottom view |
| `F` | Front view |
| `R` | Right view |
| `1-9` | Preset views |
| `F11` | Fullscreen |

### View Presets

```rust
ViewPreset {
    top: Camera::orthographic(0.0, 90.0, 0.0),
    bottom: Camera::orthographic(0.0, -90.0, 0.0),
    front: Camera::orthographic(0.0, 0.0, 0.0),
    back: Camera::orthographic(180.0, 0.0, 0.0),
    left: Camera::orthographic(-90.0, 0.0, 0.0),
    right: Camera::orthographic(90.0, 0.0, 0.0),
    isometric: Camera::perspective(45.0, 35.264, 45.0),
}
```

## Layer Visibility

### Board Layers

```rust
LayerVisibility3D {
    // Copper layers
    copper_top: true,
    copper_inner: true,
    copper_bottom: true,
    
    // Mask layers
    solder_mask_top: true,
    solder_mask_bottom: true,
    
    // Silkscreen
    silkscreen_top: true,
    silkscreen_bottom: true,
    
    // Substrate
    substrate: true,
    
    // Components
    components_top: true,
    components_bottom: true,
}
```

### Transparency Control

```rust
TransparencySettings {
    board: 1.0,           // Opaque
    solder_mask: 0.9,     // Slightly transparent
    components: 1.0,
    
    // X-ray mode
    xray_mode: false,
    xray_opacity: 0.3,
}
```

## Component Display

### 3D Model Sources

| Priority | Source |
|----------|--------|
| 1 | Component-specific STEP model |
| 2 | Footprint-assigned model |
| 3 | Auto-generated from footprint |
| 4 | Bounding box placeholder |

### Model Quality

```rust
ModelQuality {
    // LOD (Level of Detail)
    lod_enabled: true,
    lod_distances: [10.0, 50.0, 100.0],
    
    // Simplification
    max_triangles: 10000,
    
    // Textures
    texture_resolution: 1024,
}
```

### Component Highlighting

```rust
// Highlight specific component
viewer.highlight_component("U1");

// Highlight by selection
viewer.highlight_selection(&selected_refs);

// Clear highlights
viewer.clear_highlights();
```

## Board Visualization

### Copper Rendering

```rust
CopperRendering {
    // Material
    material: CopperMaterial::Realistic,
    
    // Colors
    copper_color: Color::rgb(0.72, 0.45, 0.20),
    gold_finish: false,
    
    // Detail
    show_traces: true,
    show_pads: true,
    show_vias: true,
    show_zones: true,
}
```

### Solder Mask

```rust
SolderMaskRendering {
    color: SolderMaskColor::Green,
    
    // Predefined colors
    // Green, Blue, Red, Black, White, Yellow, Purple
    
    // Custom color
    custom_color: Some(Color::rgb(0.0, 0.5, 0.0)),
    
    // Finish
    matte: false,
    glossiness: 0.7,
}
```

### Silkscreen

```rust
SilkscreenRendering {
    color: Color::WHITE,
    
    // Visibility
    show_references: true,
    show_values: false,
    show_graphics: true,
}
```

## Measurement Tools

### Distance Measurement

```rust
// Measure between two points
let distance = viewer.measure_distance(point_a, point_b);

// Measure component height
let height = viewer.measure_height("U1");
```

### Clearance Check

```rust
// Visual clearance analysis
viewer.show_clearance_zones(5.0);  // 5mm clearance

// Enclosure fit check
viewer.check_enclosure_fit(&enclosure_model);
```

## Cross-Section View

### Section Planes

```rust
SectionPlane {
    enabled: true,
    
    // Plane definition
    axis: Axis::X,
    position: 50.0,  // mm from origin
    
    // Display
    show_plane: true,
    plane_color: Color::rgba(1.0, 0.0, 0.0, 0.3),
    
    // Cap
    cap_cut: true,
    cap_color: Color::rgb(0.8, 0.8, 0.8),
}
```

### Layer Explode View

```rust
ExplodeView {
    enabled: true,
    
    // Separation
    layer_spacing: 5.0,  // mm between layers
    
    // Animation
    animated: true,
    duration: 1.0,  // seconds
}
```

## Rendering Output

### Screenshot

```rust
viewer.screenshot(ScreenshotConfig {
    path: "board_render.png",
    width: 1920,
    height: 1080,
    
    // Quality
    samples: 4,  // Anti-aliasing
    
    // Background
    background: Background::Transparent,
    // or Solid(Color), Gradient(Color, Color), HDRI(path)
});
```

### Animation Export

```rust
viewer.export_animation(AnimationConfig {
    path: "board_rotation.mp4",
    
    // Animation
    type_: AnimationType::Turntable,
    duration: 10.0,  // seconds
    fps: 30,
    
    // Quality
    resolution: (1920, 1080),
    codec: VideoCodec::H264,
});
```

## Performance Settings

### Quality Presets

| Preset | Description |
|--------|-------------|
| `Low` | Fast preview, basic shading |
| `Medium` | Balanced quality/performance |
| `High` | Full detail, shadows, AO |
| `Ultra` | Raytracing, max quality |

### GPU Acceleration

```rust
GpuConfig {
    // Prefer discrete GPU
    prefer_discrete: true,
    
    // Memory limits
    max_vram_mb: 4096,
    
    // Features
    use_compute: true,
    async_compute: true,
}
```

## Integration

### Bevy ECS Integration

```rust
use bevy::prelude::*;
use hardware_tool::viewer::*;

fn setup_viewer(
    mut commands: Commands,
    pcb: Res<PcbDesign>,
) {
    // Spawn PCB entity
    commands.spawn(PcbBundle {
        pcb: pcb.clone(),
        transform: Transform::default(),
        visibility: Visibility::Visible,
    });
    
    // Setup camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
```

### Slint UI Integration

```rust
// Embed 3D viewer in Slint UI
slint::slint! {
    import { Viewer3D } from "hardware_tool";
    
    MainWindow := Window {
        HorizontalLayout {
            SchematicView { }
            Viewer3D {
                pcb: root.current_pcb;
            }
        }
    }
}
```

## Related Topics

- [STEP & 3D Model Export/Import](./step-3d-models.md)
- [Footprints & Libraries](../pcb-layout/footprints-libraries.md)
- [Component Placement](../pcb-layout/component-placement.md)
