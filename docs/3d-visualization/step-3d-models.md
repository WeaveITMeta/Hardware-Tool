# STEP & 3D Model Export/Import

## Overview

Hardware Tool supports industry-standard 3D formats for mechanical integration, enabling fit checks with enclosures, collaboration with mechanical engineers, and manufacturing visualization. STEP is the primary format for CAD interoperability.

## Supported Formats

### Import Formats

| Format | Extension | Use Case |
|--------|-----------|----------|
| STEP | .step, .stp | Mechanical CAD exchange |
| IGES | .iges, .igs | Legacy CAD systems |
| VRML | .wrl | Colored visualization |
| STL | .stl | 3D printing, simple geometry |
| OBJ | .obj | General 3D graphics |
| glTF | .gltf, .glb | Web/game engines |

### Export Formats

| Format | Extension | Use Case |
|--------|-----------|----------|
| STEP | .step, .stp | MCAD integration |
| VRML | .wrl | Colored models |
| STL | .stl | 3D printing |
| OBJ | .obj | Rendering |
| IDF | .emn, .emp | Board exchange |

## STEP Export

### Full Board Export

```rust
StepExport::new(&pcb)
    .output_path("board_assembly.step")
    
    // Include options
    .include_board(true)
    .include_components(true)
    .include_copper(false)  // Usually not needed
    
    // Quality
    .tolerance(0.01)  // mm
    
    .export()?;
```

### Export Options

```rust
StepExportConfig {
    // Geometry
    include_board_outline: true,
    include_cutouts: true,
    include_mounting_holes: true,
    
    // Components
    include_components: true,
    component_detail: ComponentDetail::Full,
    // Full, Simplified, BoundingBox
    
    // Copper (optional, large files)
    include_copper: false,
    copper_layers: vec![Layer::FCu, Layer::BCu],
    
    // Coordinate system
    origin: Origin::BoardCenter,
    // BoardCenter, BoardCorner, Custom(x, y, z)
    
    // Units
    units: Units::Millimeters,
}
```

### Component Detail Levels

```rust
pub enum ComponentDetail {
    Full,           // Complete 3D models
    Simplified,     // Reduced polygon count
    BoundingBox,    // Simple boxes
    None,           // Board only
}
```

## STEP Import

### Enclosure Import

```rust
let enclosure = StepImport::load("enclosure.step")?;

// Position relative to board
enclosure.transform(Transform {
    translation: Vector3::new(0.0, 0.0, -5.0),
    rotation: Quaternion::identity(),
    scale: Vector3::new(1.0, 1.0, 1.0),
});

// Add to viewer
viewer.add_reference_model(enclosure);
```

### Component Model Import

```rust
// Import 3D model for footprint
let model = Model3D::import("capacitor_0603.step")?;

// Assign to footprint
footprint.set_3d_model(Model3DRef {
    path: "capacitor_0603.step",
    offset: Vector3::new(0.0, 0.0, 0.0),
    rotation: Vector3::new(0.0, 0.0, 0.0),
    scale: Vector3::new(1.0, 1.0, 1.0),
});
```

## Mechanical Fit Checks

### Clearance Analysis

```rust
let clearance_report = viewer.check_clearance(
    &pcb,
    &enclosure,
    ClearanceConfig {
        min_clearance: 1.0,  // mm
        check_components: true,
        check_connectors: true,
    }
)?;

for violation in clearance_report.violations {
    println!("Collision: {} at {:?}", 
             violation.component, 
             violation.location);
}
```

### Height Analysis

```rust
let height_map = pcb.component_heights();

println!("Max height (top): {:.2}mm", height_map.max_top);
println!("Max height (bottom): {:.2}mm", height_map.max_bottom);
println!("Total thickness: {:.2}mm", height_map.total);

// Tallest components
for (ref_des, height) in height_map.top_components(5) {
    println!("{}: {:.2}mm", ref_des, height);
}
```

### Mounting Hole Alignment

```rust
let alignment = viewer.check_mounting_alignment(
    &pcb,
    &enclosure,
    MountingConfig {
        hole_tolerance: 0.1,  // mm
        position_tolerance: 0.2,
    }
)?;

for hole in alignment.holes {
    println!("Hole at {:?}: {:?}", 
             hole.position, 
             hole.status);  // Aligned, Misaligned, Missing
}
```

## IDF Export/Import

### IDF Export (Board Exchange)

```rust
IdfExport::new(&pcb)
    .board_file("board.emn")
    .library_file("board.emp")
    
    // Options
    .include_components(true)
    .include_mounting_holes(true)
    .units(Units::Millimeters)
    
    .export()?;
```

### IDF Import

```rust
let idf_board = IdfImport::load("mechanical_design.emn")?;

// Apply mechanical constraints to PCB
pcb.apply_mechanical_constraints(&idf_board);
```

## 3D Model Libraries

### Model Search Paths

```rust
Model3DPaths {
    // Standard paths
    system: vec![
        "${HWTOOLS}/3dmodels",
    ],
    
    // Project-specific
    project: vec![
        "${PROJECT}/3dmodels",
    ],
    
    // User libraries
    user: vec![
        "~/hardware_tool/3dmodels",
    ],
}
```

### Model Assignment

```rust
// Automatic model matching
footprint.auto_assign_3d_model(ModelMatchConfig {
    // Match by footprint name
    match_footprint_name: true,
    
    // Match by component value
    match_value: true,
    
    // Fallback to generic
    use_generic: true,
});
```

### Missing Model Report

```
3D Model Report
═══════════════

Components with models: 45/52 (87%)

Missing models:
  U3 (Custom_IC_QFN32) - No match found
  J5 (Custom_Connector) - No match found
  X1 (Crystal_HC49) - Generic used
  
Suggestions:
  U3: Consider creating custom model
  J5: Check manufacturer website
  X1: Model available in extended library
```

## VRML Export

### Colored Model Export

```rust
VrmlExport::new(&pcb)
    .output_path("board_colored.wrl")
    
    // Colors
    .board_color(Color::rgb(0.1, 0.3, 0.1))  // Green
    .copper_color(Color::rgb(0.72, 0.45, 0.2))
    .silkscreen_color(Color::WHITE)
    
    // Options
    .include_components(true)
    .texture_resolution(512)
    
    .export()?;
```

## STL Export

### 3D Printing Export

```rust
StlExport::new(&pcb)
    .output_path("board_print.stl")
    
    // Options
    .binary(true)  // Binary STL (smaller)
    .include_components(false)  // Board only
    
    // Mesh quality
    .tolerance(0.05)
    .min_angle(15.0)
    
    .export()?;
```

## Coordinate Systems

### Origin Options

```rust
pub enum Origin {
    BoardCenter,           // Center of board outline
    BoardCornerLL,         // Lower-left corner
    BoardCornerUL,         // Upper-left corner
    AuxOrigin,             // User-defined aux origin
    GridOrigin,            // Grid origin
    Custom(f64, f64, f64), // Custom coordinates
}
```

### Axis Conventions

```
Hardware Tool:          MCAD (typical):
    Z (up)                  Z (up)
    │                       │
    │                       │
    └───── X                └───── X
   ╱                       ╱
  Y                       Y

(Right-handed coordinate system)
```

## Related Topics

- [3D PCB Viewer](./3d-pcb-viewer.md)
- [Footprints & Libraries](../pcb-layout/footprints-libraries.md)
- [Fabrication Drawings](../manufacturing-output/fabrication-drawings.md)
