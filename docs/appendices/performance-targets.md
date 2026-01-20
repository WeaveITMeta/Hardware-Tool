# Performance Targets

## Overview

Hardware Tool is built with **performance as a feature**. Instant response even on very large designs is a core requirement, enabled by Rust's zero-cost abstractions and Bevy's GPU-accelerated rendering.

## Response Time Targets

### User Interaction

| Action | Target | Maximum |
|--------|--------|---------|
| Mouse click response | < 16ms | 33ms |
| Keyboard input | < 16ms | 33ms |
| Pan/Zoom | < 8ms | 16ms |
| Component drag | < 16ms | 33ms |
| Selection highlight | < 8ms | 16ms |

### Operations

| Operation | Target | Maximum |
|-----------|--------|---------|
| Open small project (< 100 components) | < 500ms | 1s |
| Open medium project (100-1000 components) | < 2s | 5s |
| Open large project (1000+ components) | < 5s | 15s |
| Save project | < 500ms | 2s |
| Undo/Redo | < 50ms | 100ms |
| Component placement | < 16ms | 33ms |
| Wire routing (single segment) | < 16ms | 50ms |

### Rendering

| Metric | Target | Minimum |
|--------|--------|---------|
| Frame rate (2D) | 144 FPS | 60 FPS |
| Frame rate (3D) | 60 FPS | 30 FPS |
| Frame rate (3D raytracing) | 30 FPS | 15 FPS |
| First contentful paint | < 500ms | 1s |

## Design Size Benchmarks

### Schematic Editor

| Design Size | Components | Sheets | Target FPS |
|-------------|------------|--------|------------|
| Small | < 100 | 1-3 | 144 |
| Medium | 100-500 | 3-10 | 120 |
| Large | 500-2000 | 10-50 | 90 |
| Very Large | 2000+ | 50+ | 60 |

### PCB Editor

| Design Size | Components | Traces | Vias | Target FPS |
|-------------|------------|--------|------|------------|
| Small | < 50 | < 500 | < 100 | 144 |
| Medium | 50-200 | 500-5000 | 100-500 | 120 |
| Large | 200-1000 | 5000-50000 | 500-2000 | 90 |
| Very Large | 1000+ | 50000+ | 2000+ | 60 |

### 3D Viewer

| Complexity | Polygons | Target FPS (Standard) | Target FPS (Raytraced) |
|------------|----------|----------------------|------------------------|
| Low | < 100K | 144 | 60 |
| Medium | 100K-1M | 90 | 30 |
| High | 1M-10M | 60 | 15 |
| Very High | 10M+ | 30 | 10 |

## Memory Targets

### Base Memory

| Component | Target | Maximum |
|-----------|--------|---------|
| Application startup | < 100MB | 200MB |
| Empty project | < 150MB | 300MB |
| Idle (background) | < 50MB | 100MB |

### Per-Design Memory

| Design Size | Target | Maximum |
|-------------|--------|---------|
| Small (< 100 components) | < 200MB | 400MB |
| Medium (100-500 components) | < 500MB | 1GB |
| Large (500-2000 components) | < 1GB | 2GB |
| Very Large (2000+ components) | < 2GB | 4GB |

## Startup Performance

### Cold Start

| Phase | Target | Maximum |
|-------|--------|---------|
| Application launch | < 1s | 2s |
| Window visible | < 500ms | 1s |
| Ready for input | < 2s | 4s |
| Libraries indexed | < 5s | 10s |

### Warm Start

| Phase | Target |
|-------|--------|
| Application launch | < 500ms |
| Ready for input | < 1s |

## Export Performance

### Gerber Export

| Board Size | Layers | Target |
|------------|--------|--------|
| Small (< 50cm²) | 2 | < 1s |
| Medium (50-200cm²) | 4 | < 3s |
| Large (200-500cm²) | 6+ | < 10s |

### 3D Export (STEP)

| Complexity | Target |
|------------|--------|
| Simple (< 50 components) | < 5s |
| Medium (50-200 components) | < 15s |
| Complex (200+ components) | < 60s |

## DRC/ERC Performance

### Design Rule Check

| Design Size | Target | Maximum |
|-------------|--------|---------|
| Small | < 1s | 3s |
| Medium | < 5s | 15s |
| Large | < 30s | 60s |
| Incremental (after edit) | < 100ms | 500ms |

### Electrical Rules Check

| Design Size | Target | Maximum |
|-------------|--------|---------|
| Small | < 500ms | 1s |
| Medium | < 2s | 5s |
| Large | < 10s | 30s |

## AI Performance

### AI-Assisted Operations

| Operation | Target | Maximum |
|-----------|--------|---------|
| Auto-placement suggestion | < 2s | 5s |
| Route suggestion (single net) | < 500ms | 2s |
| Full auto-route (small board) | < 30s | 60s |
| Design review | < 10s | 30s |
| Natural language command | < 1s | 3s |

## Optimization Strategies

### Rendering Optimizations

```rust
RenderOptimizations {
    // Level of detail
    lod_enabled: true,
    lod_distances: [10.0, 50.0, 200.0],
    
    // Culling
    frustum_culling: true,
    occlusion_culling: true,
    
    // Batching
    instanced_rendering: true,
    draw_call_batching: true,
    
    // Caching
    geometry_cache: true,
    texture_atlas: true,
}
```

### Data Structure Optimizations

```rust
DataOptimizations {
    // Spatial indexing
    spatial_index: SpatialIndex::RTree,
    
    // Incremental updates
    incremental_drc: true,
    incremental_render: true,
    
    // Lazy loading
    lazy_3d_models: true,
    lazy_library_load: true,
    
    // Memory pooling
    object_pooling: true,
}
```

## Benchmarking

### Built-in Benchmarks

```rust
// Run performance benchmarks
hwt benchmark --suite full

// Output:
// Benchmark Results
// ═════════════════
// Render (1000 components): 8.2ms avg (122 FPS)
// DRC (medium board): 3.4s
// Zone fill (100cm²): 1.2s
// STEP export (200 components): 12.3s
```

### Continuous Performance Testing

- Automated benchmarks on every commit
- Performance regression detection
- Memory leak detection
- Frame time analysis

---

## Competitor Benchmarks

### PCB Design Tools Comparison

| Metric | Hardware Tool | KiCad 8 | Altium Designer | Cadence Allegro |
|--------|---------------|---------|-----------------|-----------------|
| **Cold Start** | < 2s | 5-8s | 15-25s | 30-60s |
| **Open 500-component board** | < 2s | 3-5s | 5-10s | 10-20s |
| **Pan/Zoom latency** | < 8ms | 16-33ms | 16-33ms | 33-50ms |
| **DRC (medium board)** | < 5s | 10-30s | 5-15s | 10-30s |
| **3D render (first frame)** | < 500ms | 2-5s | 1-3s | 3-10s |
| **Memory (500 components)** | < 500MB | 800MB-1.2GB | 1-2GB | 2-4GB |
| **Gerber export** | < 3s | 5-10s | 3-8s | 5-15s |

### IC Design Tools Comparison

| Metric | Hardware Tool | Cadence Virtuoso | Synopsys Custom | Magic VLSI |
|--------|---------------|------------------|-----------------|------------|
| **Cold Start** | < 2s | 30-60s | 45-90s | 3-5s |
| **Open 10K cell design** | < 5s | 15-30s | 20-45s | 5-10s |
| **DRC (1mm² die)** | < 30s | 60-180s | 60-180s | 30-90s |
| **LVS (1mm² die)** | < 60s | 120-300s | 120-300s | 60-180s |
| **Parasitic extraction** | < 120s | 300-600s | 300-600s | N/A |
| **Memory (10K cells)** | < 1GB | 4-8GB | 4-8GB | 500MB-1GB |

### 3D Visualization Comparison

| Metric | Hardware Tool (Bevy) | KiCad 3D | Altium 3D | Fusion 360 |
|--------|---------------------|----------|-----------|------------|
| **Render engine** | GPU (Vulkan/Metal) | OpenGL | DirectX | GPU |
| **FPS (1000 components)** | 90+ | 30-60 | 30-60 | 60+ |
| **Raytracing** | Real-time | Offline only | N/A | Real-time |
| **Max polygons (60 FPS)** | 10M+ | 1-2M | 2-5M | 5-10M |
| **STEP export** | < 15s | 30-60s | 20-45s | 10-30s |

### Why Hardware Tool is Faster

| Advantage | Impact |
|-----------|--------|
| **Rust** | Zero-cost abstractions, no GC pauses |
| **Bevy ECS** | Cache-friendly data layout, parallel systems |
| **GPU-first rendering** | Vulkan/Metal/DX12 backends |
| **Incremental computation** | Only recompute what changed |
| **Lazy loading** | Load libraries/models on demand |
| **Memory-mapped files** | Fast project loading |

---

## Memory Usage Targets

### Application Memory Budget

```
┌─────────────────────────────────────────────────────────────────┐
│ Memory Budget: Hardware Tool                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Base Application:                                               │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Component              │ Target   │ Max      │ Notes        │ │
│ │ ───────────────────────┼──────────┼──────────┼───────────── │ │
│ │ Executable + runtime   │ 50 MB    │ 80 MB    │ Rust binary  │ │
│ │ UI framework (Slint)   │ 20 MB    │ 40 MB    │ Widgets, fonts│ │
│ │ Render engine (Bevy)   │ 30 MB    │ 60 MB    │ GPU context  │ │
│ │ ───────────────────────┼──────────┼──────────┼───────────── │ │
│ │ Total base             │ 100 MB   │ 180 MB   │              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Per-Project Scaling:                                            │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Data Type              │ Per Item │ Example (1000 items)    │ │
│ │ ───────────────────────┼──────────┼──────────────────────── │ │
│ │ Component instance     │ 1 KB     │ 1 MB                    │ │
│ │ Net                    │ 500 B    │ 500 KB                  │ │
│ │ Trace segment          │ 100 B    │ 100 KB                  │ │
│ │ Via                    │ 50 B     │ 50 KB                   │ │
│ │ 3D model (cached)      │ 100 KB   │ 100 MB                  │ │
│ │ Undo history (step)    │ 10 KB    │ 10 MB (1000 undos)      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Memory Limits by System

| System RAM | Max Recommended Design | Notes |
|------------|------------------------|-------|
| **4 GB** | 500 components | Basic PCB, limited 3D |
| **8 GB** | 2,000 components | Standard PCB, full 3D |
| **16 GB** | 10,000 components | Complex PCB, IC design |
| **32 GB** | 50,000+ components | Large IC, multi-die |
| **64 GB+** | Unlimited | Full chip, server mode |

### Memory Optimization Strategies

```rust
MemoryOptimization {
    // Lazy loading
    lazy_load_3d_models: true,      // Load on first 3D view
    lazy_load_libraries: true,       // Load symbols on demand
    
    // Caching limits
    model_cache_size: 512 * MB,      // 3D model cache
    texture_cache_size: 256 * MB,    // Texture atlas
    undo_history_limit: 100,         // Max undo steps
    
    // Memory pressure response
    on_low_memory: LowMemoryAction::EvictCaches,
    warning_threshold: 0.80,         // 80% of available RAM
    critical_threshold: 0.95,        // 95% - start evicting
    
    // Large design mode
    large_design_threshold: 5000,    // components
    large_design_optimizations: vec![
        Optimization::ReduceLOD,
        Optimization::StreamGeometry,
        Optimization::CompressUndo,
    ],
}
```

---

## Startup Time Goals

### Startup Timeline

```
┌─────────────────────────────────────────────────────────────────┐
│ Startup Timeline: Cold Start                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ 0ms        500ms       1000ms      1500ms      2000ms          │
│ │           │           │           │           │               │
│ ├───────────┼───────────┼───────────┼───────────┤               │
│ │                                                               │
│ │ ▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░               │
│ │ Process start (100ms)                                        │
│ │                                                               │
│ │          ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░               │
│ │          Load core (200ms)                                   │
│ │                                                               │
│ │                       ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░               │
│ │                       Init UI (200ms)                        │
│ │                                                               │
│ │                                    ▓▓▓▓▓▓▓▓▓▓▓               │
│ │                                    Window visible (500ms)    │
│ │                                                               │
│ │                                              ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │
│ │                                              Ready (< 2s)    │
│ │                                                               │
│ └───────────────────────────────────────────────────────────────┘
│                                                                 │
│ Background (after ready):                                       │
│ • Library indexing: 2-5s (async)                               │
│ • Plugin loading: 1-3s (async)                                 │
│ • Update check: 500ms (async)                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Startup Phases

| Phase | Target | What Happens |
|-------|--------|--------------|
| **Process start** | < 100ms | OS loads executable |
| **Core init** | < 200ms | Rust runtime, allocators |
| **UI framework** | < 200ms | Slint initialization |
| **Window visible** | < 500ms | First frame rendered |
| **Ready for input** | < 2s | All UI responsive |
| **Libraries indexed** | < 5s | Background, non-blocking |

### Startup Optimizations

```rust
StartupOptimizations {
    // Parallel initialization
    parallel_init: true,
    init_threads: 4,
    
    // Deferred loading
    defer_plugins: true,
    defer_libraries: true,
    defer_3d_engine: true,           // Until 3D view opened
    
    // Caching
    cache_ui_layout: true,
    cache_library_index: true,
    precompiled_shaders: true,
    
    // Splash screen
    show_splash: true,
    splash_timeout: 500,             // ms, then show anyway
}
```

### Warm Start (Cached)

| Phase | Target | Notes |
|-------|--------|-------|
| **Process start** | < 50ms | OS page cache |
| **Window visible** | < 200ms | Cached shaders |
| **Ready for input** | < 500ms | Cached libraries |

### Startup Benchmark Results

```bash
$ hwt benchmark startup --iterations 10

Startup Benchmark Results
═════════════════════════
Cold start (first run):
  Process to window:  487ms ± 32ms
  Process to ready:   1.82s ± 0.15s
  
Warm start (cached):
  Process to window:  156ms ± 12ms
  Process to ready:   423ms ± 28ms

Comparison to competitors:
  Hardware Tool:  1.82s  ████░░░░░░░░░░░░░░░░
  KiCad 8:        6.2s   ██████████████░░░░░░
  Altium:        18.5s   ████████████████████████████████████████
  Cadence:       45.0s   (off chart)
```

---

## Performance Monitoring

### Built-in Profiler

```rust
// Enable performance overlay
hwt --perf-overlay

// Displays:
// ┌─────────────────────────┐
// │ FPS: 142 (7.0ms)        │
// │ CPU: 12% (4 cores)      │
// │ GPU: 45% (RTX 3080)     │
// │ RAM: 892 MB / 16 GB     │
// │ VRAM: 1.2 GB / 10 GB    │
// │ DRC: idle               │
// │ Render: 2.1ms           │
// └─────────────────────────┘
```

### Performance Alerts

```rust
PerformanceAlerts {
    // Frame time
    frame_time_warning: 16.67,       // ms (60 FPS)
    frame_time_critical: 33.33,      // ms (30 FPS)
    
    // Memory
    memory_warning: 0.75,            // 75% of limit
    memory_critical: 0.90,           // 90% of limit
    
    // Operations
    operation_timeout: 30.0,         // seconds
    
    // Actions
    on_warning: Action::LogAndNotify,
    on_critical: Action::AutoOptimize,
}
```

---

## Related Topics

- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md)
- [Design Rule Check](../pcb-layout/drc.md)
- [Real-Time Preview](../advanced-features/realtime-preview.md)
