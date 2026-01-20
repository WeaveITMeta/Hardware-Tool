# Real-Time Preview & Iteration

## Overview

Hardware Tool provides live synchronization between schematic and PCB views, enabling rapid design iteration with immediate visual feedback. Changes propagate instantly, allowing designers to see the impact of modifications in real-time.

## Live Sync Architecture

```
┌─────────────────┐         ┌─────────────────┐
│    Schematic    │◄───────►│    PCB Layout   │
│     Editor      │  Sync   │     Editor      │
└────────┬────────┘         └────────┬────────┘
         │                           │
         ▼                           ▼
┌─────────────────────────────────────────────┐
│              Unified Data Model             │
│            (Circuit JSON IR)                │
└─────────────────────────────────────────────┘
```

## Schematic-to-PCB Sync

### Forward Annotation

Changes in schematic immediately reflect in PCB:

```rust
ForwardSync {
    sync_new_components: true,
    sync_deleted_components: true,
    sync_value_changes: true,
    sync_footprint_changes: true,
    sync_new_nets: true,
    sync_deleted_nets: true,
    sync_net_renames: true,
}
```

### Sync Indicators

| Indicator | Meaning |
|-----------|---------|
| 🟢 Green | Fully synchronized |
| 🟡 Yellow | Pending changes |
| 🔴 Red | Sync conflict |

## PCB-to-Schematic Sync

### Back Annotation

PCB changes propagate to schematic:

- Reference designator swaps
- Pin/gate swaps
- Component value updates

```rust
BackSync {
    sync_ref_swaps: true,
    sync_pin_swaps: true,
    sync_value_changes: true,
    require_confirmation: true,
}
```

## Cross-Probing

### Selection Sync

Select component in one view, highlight in others:

```rust
CrossProbe {
    enabled: true,
    sync_selection: true,
    sync_zoom: true,
    center_on_item: true,
    highlight_duration: 2.0,
}
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `F3` | Find in PCB |
| `F4` | Find in schematic |
| `Ctrl+Click` | Cross-probe item |

## Visual Feedback

### Ratsnest Updates

Real-time ratsnest as components move:

```rust
RatsnestFeedback {
    update_on_drag: true,
    show_length: true,
    highlight_long_rats: true,
    long_threshold: 50.0,
}
```

### DRC Feedback

Live DRC markers during editing:

```rust
LiveDrc {
    enabled: true,
    check_clearance: true,
    check_connectivity: true,
    update_delay_ms: 100,
}
```

## Preview Modes

### Split View

```
┌─────────────────┬─────────────────┐
│                 │                 │
│   Schematic     │   PCB Layout    │
│                 │                 │
└─────────────────┴─────────────────┘
```

### Tabbed View

Switch between views with tabs, sync maintained.

### Floating Windows

Detach views to separate monitors.

## Performance

### Incremental Updates

Only changed elements re-render:

```rust
IncrementalRender {
    enabled: true,
    dirty_tracking: true,
    batch_updates: true,
    batch_delay_ms: 16,
}
```

### Background Processing

Heavy operations run asynchronously:

- Zone refills
- DRC checks
- 3D model loading

## Related Topics

- [Schematic Capture Workflow](../core-architecture/schematic-capture-workflow.md)
- [PCB Layout Workflow](../core-architecture/pcb-layout-workflow.md)
- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md)
