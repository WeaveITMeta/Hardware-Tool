# BOM & Pick-and-Place Generation

## Overview

Bill of Materials (BOM) and Pick-and-Place (centroid) files are essential for PCB assembly. Hardware Tool generates these files in various formats compatible with assembly houses and pick-and-place machines.

## Bill of Materials (BOM)

### BOM Content

| Field | Description |
|-------|-------------|
| Reference | Component designators (R1, R2, ...) |
| Value | Component value (10k, 100nF) |
| Footprint | Package type (0603, LQFP-64) |
| Quantity | Number of components |
| Manufacturer | Component manufacturer |
| MPN | Manufacturer part number |
| Description | Component description |
| DNP | Do Not Populate flag |

### BOM Export

```rust
BomExport::new(&pcb)
    .output_path("bom.csv")
    .format(BomFormat::Csv)
    
    // Grouping
    .group_by_value(true)
    .group_by_footprint(true)
    
    // Fields
    .fields(&[
        BomField::Reference,
        BomField::Value,
        BomField::Footprint,
        BomField::Quantity,
        BomField::Manufacturer,
        BomField::Mpn,
        BomField::Description,
    ])
    
    // Sorting
    .sort_by(BomSort::Reference)
    
    // Filtering
    .exclude_dnp(true)
    .exclude_virtual(true)
    
    .export()?;
```

### BOM Formats

#### CSV Format

```csv
Reference,Value,Footprint,Qty,Manufacturer,MPN,Description
"R1,R2,R3,R4",10k,0603,4,Yageo,RC0603FR-0710KL,Resistor 10k 1%
"R5,R6",4.7k,0603,2,Yageo,RC0603FR-074K7L,Resistor 4.7k 1%
"C1,C2,C3",100nF,0402,3,Samsung,CL05B104KO5NNNC,Capacitor 100nF 10%
U1,STM32F405,LQFP-64,1,STMicroelectronics,STM32F405RGT6,MCU ARM Cortex-M4
```

#### XML Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<bom project="my_board" date="2026-01-19">
  <components>
    <group>
      <references>R1,R2,R3,R4</references>
      <value>10k</value>
      <footprint>0603</footprint>
      <quantity>4</quantity>
      <manufacturer>Yageo</manufacturer>
      <mpn>RC0603FR-0710KL</mpn>
    </group>
  </components>
</bom>
```

#### HTML Format

```rust
BomExport::new(&pcb)
    .output_path("bom.html")
    .format(BomFormat::Html)
    
    // HTML options
    .html_template(Some("custom_template.html"))
    .include_images(true)
    .include_schematic_links(true)
    
    .export()?;
```

### BOM Configuration

```rust
BomConfig {
    // Grouping options
    grouping: BomGrouping {
        by_value: true,
        by_footprint: true,
        by_manufacturer: false,
        by_mpn: false,
    },
    
    // Field configuration
    fields: vec![
        FieldConfig { field: BomField::Reference, header: "Ref" },
        FieldConfig { field: BomField::Value, header: "Value" },
        FieldConfig { field: BomField::Quantity, header: "Qty" },
    ],
    
    // Filtering
    filters: BomFilters {
        exclude_dnp: true,
        exclude_virtual: true,
        exclude_fiducials: true,
        include_only: None,  // or Some(vec!["R*", "C*"])
    },
    
    // Sorting
    sort: BomSort::Reference,
    sort_direction: SortDirection::Ascending,
}
```

## Pick-and-Place (Centroid)

### Centroid Content

| Field | Description |
|-------|-------------|
| Reference | Component designator |
| X | X position (mm or mil) |
| Y | Y position (mm or mil) |
| Rotation | Rotation angle (degrees) |
| Side | Top or Bottom |
| Value | Component value |
| Footprint | Package type |

### Centroid Export

```rust
CentroidExport::new(&pcb)
    .output_path("centroid.csv")
    .format(CentroidFormat::Csv)
    
    // Units
    .units(Units::Millimeters)
    
    // Origin
    .origin(CentroidOrigin::BoardCornerLL)
    
    // Options
    .include_fiducials(true)
    .include_dnp(false)
    .separate_sides(false)  // Single file or top/bottom
    
    .export()?;
```

### Centroid Formats

#### CSV Format

```csv
Ref,Val,Package,PosX,PosY,Rot,Side
R1,10k,0603,10.500,15.200,0,top
R2,10k,0603,12.500,15.200,0,top
U1,STM32F405,LQFP-64,50.000,40.000,0,top
J1,USB_C,USB_C_Receptacle,5.000,40.000,90,top
```

#### ASCII Format

```
* Hardware Tool Pick and Place Export
* Date: 2026-01-19
* Units: mm
* Origin: Board lower-left corner

Ref        Val        Package          PosX      PosY      Rot    Side
R1         10k        0603             10.500    15.200    0      top
R2         10k        0603             12.500    15.200    0      top
U1         STM32F405  LQFP-64          50.000    40.000    0      top
```

### Centroid Configuration

```rust
CentroidConfig {
    // Position
    units: Units::Millimeters,
    origin: CentroidOrigin::BoardCornerLL,
    
    // Rotation
    rotation_format: RotationFormat::Degrees,  // or Radians
    rotation_direction: RotationDirection::CCW,
    
    // Output
    separate_files: false,  // top.csv, bottom.csv
    
    // Fields
    include_value: true,
    include_footprint: true,
    
    // Filtering
    include_fiducials: true,
    include_dnp: false,
    include_tht: false,  // Through-hole components
}
```

### Origin Options

```rust
pub enum CentroidOrigin {
    BoardCornerLL,    // Lower-left corner
    BoardCornerUL,    // Upper-left corner
    BoardCenter,      // Board center
    AuxOrigin,        // User-defined aux origin
    Custom(f64, f64), // Custom coordinates
}
```

## Assembly Notes

### Assembly Drawing Data

```rust
AssemblyNotes::new(&pcb)
    .output_path("assembly_notes.txt")
    
    // Content
    .include_component_count(true)
    .include_unique_parts(true)
    .include_smd_count(true)
    .include_tht_count(true)
    .include_special_instructions(true)
    
    .export()?;
```

### Notes Content

```
Assembly Notes - my_board Rev A
═══════════════════════════════

Component Summary:
  Total components: 156
  Unique parts: 45
  SMD components: 148
  Through-hole: 8
  
Side Distribution:
  Top side: 142
  Bottom side: 14

Special Instructions:
  - U1: Orientation mark on pin 1 corner
  - J1: Verify connector alignment before soldering
  - Q1: Apply thermal paste before mounting
  
DNP Components (Do Not Populate):
  R15, R16, C20
```

## Fiducial Markers

### Fiducial Export

```rust
FiducialExport::new(&pcb)
    .output_path("fiducials.csv")
    
    // Include
    .include_global(true)
    .include_local(true)
    
    .export()?;
```

### Fiducial Data

```csv
Type,X,Y,Side,Diameter
Global,5.000,5.000,top,1.000
Global,95.000,75.000,top,1.000
Local,25.000,35.000,top,0.500
Local,75.000,35.000,top,0.500
```

## Export Report

```
Assembly Export Report
══════════════════════

BOM:
  Output: bom.csv
  Format: CSV
  Components: 156 (45 unique)
  Excluded: 3 (DNP)

Pick-and-Place:
  Output: centroid.csv
  Format: CSV
  Units: mm
  Origin: Lower-left corner
  Components: 153
  Fiducials: 4

Files Generated:
  ✓ bom.csv
  ✓ bom.html
  ✓ centroid.csv
  ✓ centroid_top.csv
  ✓ centroid_bottom.csv
  ✓ assembly_notes.txt
  ✓ fiducials.csv
```

## Related Topics

- [Gerber Export](./gerber-export.md)
- [Fabrication Drawings](./fabrication-drawings.md)
- [Annotation & Reference Designators](../schematic-editor/annotation-reference-designators.md)
