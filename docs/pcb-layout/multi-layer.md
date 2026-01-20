# Multi-Layer Support

## Overview

Hardware Tool supports multi-layer PCB designs from simple 2-layer boards to complex HDI stackups with blind and buried vias. This document covers layer stackup definition, inner layer management, and via types.

## Layer Types

### Copper Layers

```rust
pub enum CopperLayer {
    FCu,        // Front (top) copper
    In1Cu,      // Inner layer 1
    In2Cu,      // Inner layer 2
    // ... up to In30Cu
    BCu,        // Back (bottom) copper
}
```

### Technical Layers

| Layer | Purpose |
|-------|---------|
| F.Paste | Top solder paste stencil |
| B.Paste | Bottom solder paste stencil |
| F.SilkS | Top silkscreen |
| B.SilkS | Bottom silkscreen |
| F.Mask | Top solder mask |
| B.Mask | Bottom solder mask |
| F.CrtYd | Top courtyard |
| B.CrtYd | Bottom courtyard |
| F.Fab | Top fabrication |
| B.Fab | Bottom fabrication |
| Edge.Cuts | Board outline |

## Stackup Definition

### Basic 4-Layer Stackup

```rust
Stackup::new()
    .layer(StackupLayer::copper("F.Cu", 35.0))      // 35µm (1oz)
    .layer(StackupLayer::prepreg(0.2))               // 0.2mm
    .layer(StackupLayer::copper("In1.Cu", 35.0))    // Ground plane
    .layer(StackupLayer::core(1.0))                  // 1.0mm core
    .layer(StackupLayer::copper("In2.Cu", 35.0))    // Power plane
    .layer(StackupLayer::prepreg(0.2))
    .layer(StackupLayer::copper("B.Cu", 35.0))
    .build()
```

### Layer Properties

```rust
StackupLayer {
    name: "F.Cu",
    type_: LayerType::Copper,
    
    // Physical properties
    thickness: 35.0,        // µm
    
    // Electrical properties (for copper)
    conductivity: 5.8e7,    // S/m (copper)
    
    // Dielectric properties (for insulator)
    dielectric_constant: 4.5,
    loss_tangent: 0.02,
    
    // Material
    material: "FR-4",
}
```

### Common Stackups

#### 2-Layer

```
┌─────────────────────────┐
│ F.Cu (Signal)           │ 35µm
├─────────────────────────┤
│ FR-4 Core               │ 1.6mm
├─────────────────────────┤
│ B.Cu (Signal)           │ 35µm
└─────────────────────────┘
Total: ~1.6mm
```

#### 4-Layer

```
┌─────────────────────────┐
│ F.Cu (Signal)           │ 35µm
├─────────────────────────┤
│ Prepreg                 │ 0.2mm
├─────────────────────────┤
│ In1.Cu (GND)            │ 35µm
├─────────────────────────┤
│ Core                    │ 1.0mm
├─────────────────────────┤
│ In2.Cu (VCC)            │ 35µm
├─────────────────────────┤
│ Prepreg                 │ 0.2mm
├─────────────────────────┤
│ B.Cu (Signal)           │ 35µm
└─────────────────────────┘
Total: ~1.6mm
```

#### 6-Layer HDI

```
┌─────────────────────────┐
│ F.Cu (Signal)           │ 35µm
├─────────────────────────┤
│ Prepreg                 │ 0.1mm
├─────────────────────────┤
│ In1.Cu (GND)            │ 35µm
├─────────────────────────┤
│ Core                    │ 0.4mm
├─────────────────────────┤
│ In2.Cu (Signal)         │ 35µm
├─────────────────────────┤
│ Prepreg                 │ 0.1mm
├─────────────────────────┤
│ In3.Cu (Signal)         │ 35µm
├─────────────────────────┤
│ Core                    │ 0.4mm
├─────────────────────────┤
│ In4.Cu (VCC)            │ 35µm
├─────────────────────────┤
│ Prepreg                 │ 0.1mm
├─────────────────────────┤
│ B.Cu (Signal)           │ 35µm
└─────────────────────────┘
Total: ~1.2mm
```

## Via Types

### Through-Hole Via

Connects all layers:

```rust
Via::through()
    .position(25.0, 30.0)
    .diameter(0.6)
    .drill(0.3)
    .net("VCC")
```

```
F.Cu  ════●════
          │
In1.Cu ───┼───
          │
In2.Cu ───┼───
          │
B.Cu  ════●════
```

### Blind Via

Connects outer layer to inner layer:

```rust
Via::blind(Layer::FCu, Layer::In1Cu)
    .position(25.0, 30.0)
    .diameter(0.4)
    .drill(0.2)
    .net("SIG")
```

```
F.Cu  ════●════
          │
In1.Cu ───●───
          
In2.Cu ────────
          
B.Cu  ─────────
```

### Buried Via

Connects inner layers only:

```rust
Via::buried(Layer::In1Cu, Layer::In2Cu)
    .position(25.0, 30.0)
    .diameter(0.4)
    .drill(0.2)
    .net("SIG")
```

```
F.Cu  ─────────
          
In1.Cu ───●───
          │
In2.Cu ───●───
          
B.Cu  ─────────
```

### Microvia

Single-layer laser-drilled via:

```rust
Via::micro(Layer::FCu, Layer::In1Cu)
    .position(25.0, 30.0)
    .diameter(0.2)
    .drill(0.1)
    .net("SIG")
```

## Layer Assignment

### Signal Layer Planning

```rust
LayerAssignment {
    // Horizontal routing
    horizontal: vec![Layer::FCu, Layer::In2Cu],
    
    // Vertical routing
    vertical: vec![Layer::In1Cu, Layer::BCu],
    
    // Plane layers
    planes: vec![
        (Layer::In1Cu, "GND"),
        (Layer::In2Cu, "VCC"),
    ],
}
```

### Layer Pairs

```rust
LayerPairs {
    // Define routing pairs for via transitions
    pairs: vec![
        (Layer::FCu, Layer::In1Cu),    // Top to inner
        (Layer::In2Cu, Layer::BCu),    // Inner to bottom
    ],
    
    // Via type for each pair
    via_types: hashmap! {
        (Layer::FCu, Layer::In1Cu) => ViaType::Blind,
        (Layer::FCu, Layer::BCu) => ViaType::Through,
    },
}
```

## Inner Layer Management

### Plane Layers

```rust
// Define ground plane
Zone::new("GND_PLANE")
    .layer(Layer::In1Cu)
    .net("GND")
    .fill_mode(FillMode::Solid)
    .from_board_outline();

// Define power plane with splits
Zone::new("VCC_3V3")
    .layer(Layer::In2Cu)
    .net("VCC_3V3")
    .region(left_half);

Zone::new("VCC_5V")
    .layer(Layer::In2Cu)
    .net("VCC_5V")
    .region(right_half);
```

### Signal on Inner Layers

```rust
// Route high-speed signals on inner layers
NetRule::new("DDR_*")
    .preferred_layers(&[Layer::In1Cu, Layer::In2Cu])
    .reference_plane(Layer::In1Cu);  // Ground reference
```

## Impedance Control

### Stackup for Impedance

```rust
ImpedanceStackup {
    // Single-ended
    single_ended: ImpedanceTarget {
        impedance: 50.0,
        tolerance: 10.0,
        trace_width: 0.15,
        reference_layer: Layer::In1Cu,
    },
    
    // Differential
    differential: ImpedanceTarget {
        impedance: 90.0,
        tolerance: 10.0,
        trace_width: 0.1,
        trace_gap: 0.15,
        reference_layer: Layer::In1Cu,
    },
}
```

### Impedance Calculator Integration

```rust
// Calculate trace width for target impedance
let width = impedance_calculator::microstrip(
    target_impedance: 50.0,
    dielectric_height: 0.2,
    dielectric_constant: 4.5,
    copper_thickness: 0.035,
);
```

## Layer Visibility

### Display Control

```rust
LayerVisibility {
    // Show/hide layers
    visible: vec![Layer::FCu, Layer::BCu, Layer::In1Cu],
    hidden: vec![Layer::In2Cu],
    
    // Transparency
    opacity: hashmap! {
        Layer::FCu => 1.0,
        Layer::In1Cu => 0.5,
    },
    
    // Colors
    colors: hashmap! {
        Layer::FCu => Color::Red,
        Layer::BCu => Color::Blue,
        Layer::In1Cu => Color::Green,
    },
}
```

### Layer Manager

| Layer | Visible | Color | Opacity |
|-------|---------|-------|---------|
| F.Cu | ✓ | Red | 100% |
| In1.Cu | ✓ | Yellow | 50% |
| In2.Cu | ○ | Green | 50% |
| B.Cu | ✓ | Blue | 100% |

## Manufacturing Considerations

### Layer Count Selection

| Layers | Use Case | Cost Factor |
|--------|----------|-------------|
| 2 | Simple designs | 1x |
| 4 | Most designs | 1.5-2x |
| 6 | Complex/high-speed | 2-3x |
| 8+ | HDI/advanced | 3-5x+ |

### HDI Considerations

```rust
HdiConfig {
    // Microvia requirements
    microvia_aspect_ratio: 0.75,  // depth/diameter
    
    // Stacked vias
    allow_stacked_microvias: true,
    max_stack: 2,
    
    // Sequential lamination
    lamination_cycles: 2,
}
```

## Related Topics

- [Via & Via Stitching](./via-stitching.md)
- [Copper Zones](./copper-zones.md)
- [Interactive Routing](./interactive-routing.md)
