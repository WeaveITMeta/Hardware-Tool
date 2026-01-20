# Innovative Interaction Patterns

## Overview

Hardware Tool introduces novel interaction patterns that make EDA work feel **delightful, fast, and fluid**. These patterns reduce cognitive load, minimize mouse travel, and provide rich visual feedback — making the tool feel like it's actively helping you design.

## Interaction Pattern Catalog

### Magnet Cursor

The cursor subtly pulls toward sensible connection points, pads, and centers.

```rust
MagnetCursor {
    enabled: true,
    
    // Attraction targets
    targets: vec![
        MagnetTarget::Pads { strength: 1.0 },
        MagnetTarget::PinEnds { strength: 0.8 },
        MagnetTarget::GridPoints { strength: 0.3 },
        MagnetTarget::TraceEndpoints { strength: 0.7 },
        MagnetTarget::ComponentCenters { strength: 0.5 },
    ],
    
    // Behavior
    attraction_radius: 15.0,  // pixels
    snap_threshold: 8.0,      // pixels - hard snap when closer
    visual_indicator: true,   // Show subtle glow on target
    
    // Feel
    easing: Easing::EaseOutQuad,
    response_time: 0.05,      // seconds
}
```

**Why it's amazing:** Feels like the tool is helping you. Reduces precision stress and speeds up connection work by 40%+.

### Gesture Router

Hold right mouse + draw gesture → auto-route selected net with chosen style.

```rust
GestureRouter {
    enabled: true,
    activation: MouseButton::Right,
    
    // Recognized gestures
    gestures: vec![
        Gesture::StraightLine => RouteStyle::Direct,
        Gesture::ZigZag => RouteStyle::Manhattan,
        Gesture::Curve => RouteStyle::Curved,
        Gesture::Loop => RouteStyle::LengthMatched,
        Gesture::Spiral => RouteStyle::Serpentine,
    ],
    
    // Visual feedback
    trail_visible: true,
    trail_color: Color::Cyan.with_alpha(0.6),
    preview_route: true,
}
```

**Why it's amazing:** Very fast interactive routing. Draw your intent, get instant results.

### Shadow Nudge

Hold Alt + drag component → shows ghost + live clearance/DRC feedback.

```rust
ShadowNudge {
    enabled: true,
    activation: Modifier::Alt,
    
    // Ghost appearance
    ghost_opacity: 0.5,
    ghost_color: Color::Cyan,
    
    // Live feedback
    show_clearance_rings: true,
    show_drc_violations: true,
    show_ratsnest_preview: true,
    
    // Snap behavior
    snap_to_valid_positions: true,
    highlight_valid_zones: true,
}
```

**Why it's amazing:** Extremely fast "what-if" placement experiments. See DRC impact before committing.

### Live Zone Pour Preview

As you draw zone outline → real-time filled copper preview with thermal spokes.

```rust
LiveZonePour {
    enabled: true,
    
    // Preview quality
    preview_resolution: PreviewResolution::Medium,
    show_thermals: true,
    show_clearances: true,
    
    // Animation
    flood_animation: true,
    flood_duration: 0.3,  // seconds
    
    // Performance
    debounce_ms: 50,
    use_gpu_compute: true,
}
```

**Why it's amazing:** No more waiting to refill zones. See exactly what you'll get as you draw.

### Smart Context Bar

Small radial/linear toolbar appears near cursor after selection.

```rust
SmartContextBar {
    enabled: true,
    
    // Appearance
    style: ContextBarStyle::Radial,  // or Linear
    position: ContextBarPosition::NearCursor,
    offset: 30.0,  // pixels from cursor
    
    // Timing
    delay_ms: 200,        // Wait before showing
    fade_in_ms: 100,
    auto_hide_ms: 3000,   // Hide if no interaction
    
    // Actions (context-dependent)
    schematic_actions: vec![
        Action::Rotate,
        Action::Mirror,
        Action::EditValue,
        Action::Delete,
        Action::Duplicate,
    ],
    pcb_actions: vec![
        Action::Rotate,
        Action::Flip,
        Action::RouteFrom,
        Action::Properties,
        Action::Align,
    ],
}
```

**Why it's amazing:** Reduces mouse travel dramatically. Actions are always within reach.

### Inline Code Snippet Insertion

Drag code snippet from panel → instantly places modular block (TSCircuit style).

```rust
CodeSnippetInsertion {
    enabled: true,
    
    // Snippet sources
    sources: vec![
        SnippetSource::BuiltIn,
        SnippetSource::UserLibrary,
        SnippetSource::ProjectLocal,
    ],
    
    // Insertion behavior
    preview_on_drag: true,
    auto_connect_matching_nets: true,
    prompt_for_parameters: true,
    
    // Visual
    drop_zone_highlight: true,
    connection_preview: true,
}
```

**Why it's amazing:** Bridges GUI and code worlds beautifully. Reuse complex circuits instantly.

### Confidence Heatmap

Auto-placement/autorouter shows confidence heatmap (green→red) on proposed positions.

```rust
ConfidenceHeatmap {
    enabled: true,
    
    // Visualization
    color_scale: ColorScale::GreenYellowRed,
    opacity: 0.4,
    blend_mode: BlendMode::Overlay,
    
    // Metrics shown
    metrics: vec![
        Metric::RoutingDifficulty,
        Metric::ThermalRisk,
        Metric::SignalIntegrity,
        Metric::ManufacturabilityScore,
    ],
    
    // Interaction
    hover_for_details: true,
    click_to_accept: true,
}
```

**Why it's amazing:** Immediately understand quality of AI suggestions. Make informed decisions.

### Temporal History Scrubber

Bottom timeline lets you scrub through undo history with live preview.

```rust
HistoryScrubber {
    enabled: true,
    
    // Position
    location: ScrubberLocation::BottomOverlay,
    height: 40,  // pixels
    
    // Visualization
    show_thumbnails: true,
    thumbnail_interval: 10,  // Every 10 actions
    highlight_major_changes: true,
    
    // Interaction
    scrub_preview: true,      // Live preview while dragging
    click_to_restore: true,
    branch_visualization: true,
    
    // Feel
    snap_to_checkpoints: true,
    smooth_scrubbing: true,
}
```

**Why it's amazing:** Feels like video editing. Very intuitive recovery and exploration.

### Visual Net Spy

Hover any net label → animated highlight path across sheets & PCB (with glow pulses).

```rust
VisualNetSpy {
    enabled: true,
    activation: HoverDuration::Instant,  // or AfterDelay(ms)
    
    // Highlight style
    highlight_color: Color::Cyan,
    glow_intensity: 0.8,
    pulse_animation: true,
    pulse_speed: 1.5,  // Hz
    
    // Scope
    cross_sheet: true,
    cross_domain: true,  // Schematic ↔ PCB
    show_in_3d: true,
    
    // Information overlay
    show_net_length: true,
    show_connection_count: true,
    show_layer_usage: true,
}
```

**Why it's amazing:** Insanely fast debugging of connectivity. See entire net path instantly.

### Floating 3D Reference

A Bevy-powered 3D viewport floats in the **top-right corner** of the main canvas, providing constant mechanical awareness while working in schematic or PCB views.

```
┌─────────────────────────────────────────┐
│ 3D Preview                    [⬜][✕]  │  ← Maximize | Close
├─────────────────────────────────────────┤
│                                         │
│   [Real-time 3D PCB render in Bevy]     │
│                                         │
└─────────────────────────────────────────┘
```

#### Window Controls

| Button | Icon | Action |
|--------|------|--------|
| **Maximize** | `⬜` | Expand to full viewport (replaces main canvas) |
| **Restore** | `❐` | Return to floating panel in top-right corner |
| **Close** | `✕` | Hide panel completely |

#### Reopening Hidden Panel

When closed, access via:
- **Menu**: `View → 3D Preview Panel`
- **Keyboard**: `F8`
- **Command Palette**: `Ctrl+K` → "Show 3D Preview"

```rust
Floating3DReference {
    enabled: true,
    
    // Rendering engine
    renderer: Renderer::Bevy,
    
    // Position (fixed to top-right)
    position: PanelPosition::TopRight,
    
    // Size (when floating)
    size: (320, 240),
    resizable: true,
    min_size: (200, 150),
    max_size: (600, 450),
    
    // Window controls
    show_maximize_button: true,
    show_close_button: true,
    
    // Maximize behavior
    maximize: MaximizeBehavior {
        replace_main_canvas: true,
        animate_transition: true,
        transition_duration: 0.2,
        show_restore_button: true,  // ❐ to return to floating
    },
    
    // View settings
    auto_focus_cursor: true,
    focus_radius: 10.0,  // mm around cursor
    rotation: Rotation::Isometric,
    
    // Rendering
    quality: RenderQuality::Medium,
    show_component_under_cursor: true,
    
    // Keyboard
    toggle_hotkey: Key::F8,
    maximize_hotkey: Key::Shift_F8,
}
```

**Why it's amazing:** Constant mechanical awareness without mode switch. See height clearances while routing. Maximize for detailed inspection, restore to continue working.

### Command Palette 2.0

Ctrl+K → fuzzy search + natural language understanding.

```rust
CommandPalette {
    enabled: true,
    hotkey: Key::Ctrl_K,
    
    // Search
    fuzzy_search: true,
    natural_language: true,
    ai_powered: true,
    
    // Examples of understood commands:
    // "route differential pair 100mil"
    // "place 10 decoupling caps near U1"
    // "check clearance on power nets"
    // "export gerbers for JLCPCB"
    
    // History
    remember_recent: true,
    recent_count: 20,
    
    // Appearance
    position: PalettePosition::TopCenter,
    width: 600,
    max_results: 10,
}
```

**Why it's amazing:** Extremely fast power-user workflow. Natural language removes learning curve.

## AI-Powered Interactions

### AI Design Assistant

Native AI integration for intelligent design assistance:

```rust
AiAssistant {
    enabled: true,
    
    // Capabilities
    features: vec![
        AiFeature::AutoPlacement,
        AiFeature::AutoRouting,
        AiFeature::DesignReview,
        AiFeature::Optimization,
        AiFeature::NaturalLanguageCommands,
        AiFeature::ErrorExplanation,
        AiFeature::ComponentSuggestion,
    ],
    
    // Interaction modes
    inline_suggestions: true,
    chat_interface: true,
    voice_commands: false,  // Future
    
    // Feedback
    explain_decisions: true,
    show_confidence: true,
    allow_manual_override: true,
}
```

### AI Routing Engine

```rust
AiRouter {
    // Strategy
    approach: AiRoutingApproach::FirstPrinciples,
    
    // Optimization targets
    optimize_for: vec![
        OptimizationTarget::SignalIntegrity,
        OptimizationTarget::PowerDelivery,
        OptimizationTarget::Manufacturability,
        OptimizationTarget::ThermalPerformance,
    ],
    
    // Real-time visualization
    show_thinking_process: true,
    animate_route_exploration: true,
    benchmark_each_step: true,
}
```

## Interaction Feedback

### Visual Feedback Language

| Action | Feedback |
|--------|----------|
| Valid connection | Green glow pulse |
| Invalid action | Red flash + shake |
| DRC violation | Orange highlight + icon |
| Successful operation | Mint confirmation flash |
| AI suggestion | Cyan animated outline |

### Audio Feedback (Optional)

```rust
AudioFeedback {
    enabled: false,  // Off by default
    
    sounds: vec![
        Sound::Connect => "soft_click.wav",
        Sound::Error => "gentle_error.wav",
        Sound::Complete => "success_chime.wav",
    ],
    
    volume: 0.3,
    spatial: false,
}
```

## Related Topics

- [Main Window Layout](./main-window-layout.md)
- [Visual Style Guidelines](./visual-style-guidelines.md)
- [Keyboard Shortcuts Reference](./keyboard-shortcuts-reference.md)
