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

## Related Topics

- [3D PCB Viewer](../3d-visualization/3d-pcb-viewer.md)
- [Design Rule Check](../pcb-layout/drc.md)
- [Real-Time Preview](../advanced-features/realtime-preview.md)
