# IPC-2581 Export

## Overview

IPC-2581 is a single-file intelligent format that contains complete PCB manufacturing data including stackup, nets, BOM, and assembly information. It eliminates the need for multiple Gerber files and provides richer data for advanced manufacturing processes.

## Format Benefits

### Advantages over Gerber

| Feature | Gerber | IPC-2581 |
|---------|--------|----------|
| File count | Multiple | Single |
| Net information | No | Yes |
| Component data | No | Yes |
| Stackup | Separate | Included |
| BOM | Separate | Included |
| Assembly | Separate | Included |
| Intelligent data | No | Yes |

### Use Cases

- Advanced PCB fabricators
- Assembly houses
- Design-for-manufacturing analysis
- Automated quoting systems
- Full data exchange

## Export Configuration

### Basic Export

```rust
Ipc2581Export::new(&pcb)
    .output_path("board.cvg")  // or .xml
    
    // Content
    .include_stackup(true)
    .include_bom(true)
    .include_assembly(true)
    
    // Options
    .compress(true)  // .cvg (compressed)
    .version(Ipc2581Version::B)
    
    .export()?;
```

### Detailed Configuration

```rust
Ipc2581Config {
    // Output
    output_path: "board.cvg",
    format: Ipc2581Format::Compressed,  // or Xml
    version: Ipc2581Version::B,
    
    // Content sections
    content: Ipc2581Content {
        ecad: true,           // Design data
        bom: true,            // Bill of materials
        avl: false,           // Approved vendor list
        layup: true,          // Stackup
        profile: true,        // Board outline
        components: true,     // Component data
        nets: true,           // Net connectivity
        test_points: true,    // Test access
    },
    
    // Metadata
    metadata: Ipc2581Metadata {
        company: "My Company",
        author: "Designer Name",
        revision: "A",
        date: chrono::Utc::now(),
    },
}
```

## File Structure

### XML Schema

```xml
<?xml version="1.0" encoding="UTF-8"?>
<IPC-2581 revision="B">
  <Content>
    <FunctionMode mode="FABRICATION"/>
    <StepRef name="BOARD"/>
  </Content>
  
  <Bom>
    <BomHeader assembly="BOARD"/>
    <BomItem>...</BomItem>
  </Bom>
  
  <Ecad name="BOARD">
    <CadHeader/>
    <CadData>
      <Layer>...</Layer>
      <Step>...</Step>
    </CadData>
  </Ecad>
  
  <Avl>...</Avl>
</IPC-2581>
```

### Key Sections

| Section | Content |
|---------|---------|
| `Content` | File metadata and mode |
| `Bom` | Bill of materials |
| `Ecad` | Design data (layers, geometry) |
| `Avl` | Approved vendor list |
| `LogisticHeader` | Manufacturing logistics |

## Stackup Definition

### Layer Stack

```xml
<Stackup name="4-Layer">
  <StackupGroup name="CORE">
    <StackupLayer layerOrGroup="TOP" thickness="0.035" material="COPPER"/>
    <StackupLayer layerOrGroup="PREPREG1" thickness="0.2" material="FR4"/>
    <StackupLayer layerOrGroup="INNER1" thickness="0.035" material="COPPER"/>
    <StackupLayer layerOrGroup="CORE1" thickness="1.0" material="FR4"/>
    <StackupLayer layerOrGroup="INNER2" thickness="0.035" material="COPPER"/>
    <StackupLayer layerOrGroup="PREPREG2" thickness="0.2" material="FR4"/>
    <StackupLayer layerOrGroup="BOTTOM" thickness="0.035" material="COPPER"/>
  </StackupGroup>
</Stackup>
```

### Material Properties

```rust
StackupMaterial {
    name: "FR4",
    type_: MaterialType::Dielectric,
    
    // Electrical
    dielectric_constant: 4.5,
    loss_tangent: 0.02,
    
    // Physical
    thickness: 1.0,  // mm
    
    // Thermal
    tg: 170,  // Glass transition °C
}
```

## Net Information

### Net Export

```xml
<Net name="VCC">
  <NetNode layerRef="TOP" componentRef="U1" pinRef="14"/>
  <NetNode layerRef="TOP" componentRef="C1" pinRef="1"/>
  <NetNode layerRef="TOP" componentRef="C2" pinRef="1"/>
</Net>
```

### Net Properties

```rust
Ipc2581Net {
    name: "VCC",
    class: Some("POWER"),
    
    // Electrical properties
    voltage: Some(3.3),
    current_max: Some(0.5),
    
    // Routing constraints
    min_width: Some(0.3),
    min_clearance: Some(0.2),
}
```

## BOM Integration

### Component Data

```xml
<BomItem quantity="10" category="RESISTOR">
  <RefDes name="R1"/>
  <RefDes name="R2"/>
  <!-- ... -->
  <Characteristics>
    <Characteristic name="VALUE" value="10K"/>
    <Characteristic name="TOLERANCE" value="1%"/>
    <Characteristic name="PACKAGE" value="0603"/>
  </Characteristics>
</BomItem>
```

### BOM Export Options

```rust
BomConfig {
    // Grouping
    group_by_value: true,
    group_by_footprint: true,
    
    // Fields
    include_manufacturer: true,
    include_mpn: true,
    include_description: true,
    
    // Sorting
    sort_by: BomSort::Reference,
}
```

## Assembly Data

### Component Placement

```xml
<Component refDes="U1" packageRef="LQFP-64">
  <Location x="50.0" y="40.0"/>
  <Rotation rotation="0"/>
  <LayerRef name="TOP"/>
</Component>
```

### Assembly Information

```rust
AssemblyData {
    // Placement
    components: vec![...],
    
    // Fiducials
    fiducials: vec![
        Fiducial::new(5.0, 5.0, FiducialType::Global),
        Fiducial::new(95.0, 75.0, FiducialType::Global),
    ],
    
    // Panel info
    panel: Some(PanelInfo {
        rows: 2,
        columns: 3,
        spacing: 5.0,
    }),
}
```

## Test Points

### Test Access

```xml
<TestPoint refDes="TP1" netRef="VCC">
  <Location x="25.0" y="30.0"/>
  <LayerRef name="TOP"/>
  <Access top="true" bottom="false"/>
</TestPoint>
```

### Test Point Export

```rust
TestPointConfig {
    include_test_points: true,
    include_vias: false,
    include_pads: true,
    
    // Access requirements
    min_pad_size: 0.5,
    probe_size: 0.75,
}
```

## Validation

### Schema Validation

```rust
// Validate against IPC-2581 schema
let validation = Ipc2581Validator::validate("board.cvg")?;

if validation.is_valid() {
    println!("File is valid IPC-2581");
} else {
    for error in validation.errors() {
        println!("Error: {}", error);
    }
}
```

### Export Report

```
IPC-2581 Export Report
══════════════════════

Output: board.cvg
Format: IPC-2581 Rev B (Compressed)
Size: 1.2 MB

Content:
  ✓ ECAD data (12 layers)
  ✓ Stackup (4-layer)
  ✓ BOM (52 unique parts)
  ✓ Nets (245 nets)
  ✓ Components (156 placements)
  ✓ Test points (23 points)

Validation: PASSED
```

## Related Topics

- [Gerber Export](./gerber-export.md)
- [ODB++ Export](./odbpp-export.md)
- [BOM Generation](./bom-pick-place.md)
