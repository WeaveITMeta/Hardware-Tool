# ODB++ Export

## Overview

ODB++ is a comprehensive folder-based format developed by Mentor Graphics (now Siemens) for transferring complete PCB design data to manufacturing. It's widely used by professional fabricators and provides rich manufacturing intelligence.

## Format Structure

### Directory Layout

```
my_board.odb/
├── matrix/
│   └── matrix              # Layer stackup definition
├── misc/
│   ├── info                # General information
│   └── attrlist            # Attribute definitions
├── fonts/
│   └── standard            # Font definitions
├── symbols/
│   └── ...                 # Symbol definitions
├── wheels/
│   └── ...                 # Aperture wheels
├── steps/
│   └── pcb/
│       ├── stephdr         # Step header
│       ├── profile         # Board outline
│       ├── netlists/
│       │   └── cadnet      # Net connectivity
│       ├── layers/
│       │   ├── top/
│       │   │   └── features
│       │   ├── bottom/
│       │   ├── solder_mask_top/
│       │   └── ...
│       └── eda/
│           └── data        # EDA data
└── input/
    └── ...                 # Original input files
```

## Export Configuration

### Basic Export

```rust
OdbExport::new(&pcb)
    .output_dir("./odb")
    .job_name("my_board")
    
    // Content
    .include_netlist(true)
    .include_components(true)
    .include_eda_data(true)
    
    // Compression
    .compress(true)  // Creates .tgz
    
    .export()?;
```

### Detailed Configuration

```rust
OdbConfig {
    // Output
    output_dir: "./odb",
    job_name: "my_board",
    compress: true,
    
    // Content
    content: OdbContent {
        layers: true,
        netlist: true,
        components: true,
        eda_data: true,
        drill: true,
        rout: true,
    },
    
    // Options
    units: Units::Millimeters,
    angle_units: AngleUnits::Degrees,
    
    // Metadata
    customer: Some("Customer Name"),
    revision: Some("A"),
}
```

## Layer Definition

### Matrix File

```
UNITS=MM
STEP {
   COL_NAME=pcb
   COL_POSITIVE=YES
   COL_CONTEXT=BOARD
}

LAYER {
   ROW_NAME=top
   ROW_CONTEXT=BOARD
   ROW_TYPE=SIGNAL
   ROW_POLARITY=POSITIVE
   ROW_START_NAME=
   ROW_END_NAME=
}

LAYER {
   ROW_NAME=gnd
   ROW_CONTEXT=BOARD
   ROW_TYPE=POWER_GROUND
   ROW_POLARITY=POSITIVE
}
```

### Layer Types

| Type | Description |
|------|-------------|
| SIGNAL | Signal routing layer |
| POWER_GROUND | Plane layer |
| MIXED | Signal and plane |
| SOLDER_MASK | Solder mask |
| SILK_SCREEN | Silkscreen |
| SOLDER_PASTE | Paste stencil |
| DRILL | Drill layer |
| ROUT | Board outline/routing |
| DOCUMENT | Documentation |

## Feature Data

### Layer Features

```
#
#Feature file for layer: top
#
UNITS=MM
#
#num_features
$0 r10
$1 r20
$2 rect10x20
#
#feature_data
P 10.000 15.000 0 P 0 ;0;
P 20.000 15.000 0 P 0 ;0;
L 10.000 15.000 20.000 15.000 1 P 0 ;0;
```

### Feature Types

| Code | Type |
|------|------|
| P | Pad |
| L | Line |
| A | Arc |
| S | Surface |
| T | Text |

## Netlist Data

### CAD Netlist

```
H optimize n
H staggered n
H units mm

$0 VCC
$1 GND
$2 DATA_0

# Net VCC
FID C1 1 10.000 15.000 top
FID U1 14 50.000 40.000 top
FID C2 1 30.000 15.000 top

# Net GND
FID C1 2 10.000 12.000 top
FID U1 7 50.000 20.000 top
```

### Net Properties

```rust
OdbNet {
    name: "VCC",
    
    // Electrical
    net_class: Some("POWER"),
    voltage: Some(3.3),
    
    // Routing
    min_width: Some(0.3),
    max_width: Some(1.0),
    
    // Connections
    pins: vec![
        NetPin { component: "U1", pin: "14", layer: "top" },
        NetPin { component: "C1", pin: "1", layer: "top" },
    ],
}
```

## Component Data

### EDA Data

```
#
#Component data
#
CMP 0 10.000 15.000 0 N R1 0603 ;
CMP 1 20.000 15.000 0 N R2 0603 ;
CMP 2 50.000 40.000 0 N U1 LQFP-64 ;

#
#Package data
#
PKG 0603 0603
PRP MOUNT_TYPE 'SMD'
TOP 0 0.000 0.000 0 1 0.9 0.95 r
TOP 1 1.500 0.000 0 2 0.9 0.95 r
```

### Component Properties

```rust
OdbComponent {
    ref_des: "U1",
    package: "LQFP-64",
    
    // Position
    x: 50.0,
    y: 40.0,
    rotation: 0.0,
    mirror: false,
    layer: "top",
    
    // Properties
    value: Some("STM32F4"),
    manufacturer: Some("STMicroelectronics"),
    mpn: Some("STM32F405RGT6"),
}
```

## Drill Data

### Drill Layer

```
#
#Drill data
#
UNITS=MM

# Tool definitions
T01 0.300 PLATED
T02 0.800 PLATED
T03 1.000 NON_PLATED

# Drill hits
T01
10.000 15.000
20.000 15.000
30.000 15.000

T02
50.000 40.000

T03
5.000 5.000
95.000 5.000
```

### Drill Properties

```rust
OdbDrill {
    // Tool
    tool_number: 1,
    diameter: 0.3,
    
    // Type
    plated: true,
    
    // Span
    start_layer: "top",
    end_layer: "bottom",
    
    // Tolerance
    tolerance_plus: 0.05,
    tolerance_minus: 0.05,
}
```

## Profile (Board Outline)

### Profile Definition

```
#
#Board profile
#
UNITS=MM
OB 0.000 0.000 I
OS 100.000 0.000
OS 100.000 80.000
OS 0.000 80.000
OE

# Cutout
OB 45.000 35.000 H
OS 55.000 35.000
OS 55.000 45.000
OS 45.000 45.000
OE
```

## Export Options

### Compression

```rust
OdbCompression {
    // Output format
    format: OdbFormat::Tgz,  // or Folder, Zip
    
    // Compression level
    level: 6,  // 1-9
}
```

### Attribute Export

```rust
OdbAttributes {
    // Standard attributes
    include_standard: true,
    
    // Custom attributes
    custom: vec![
        ("CUSTOMER", "Acme Corp"),
        ("PROJECT", "Widget v2"),
    ],
}
```

## Validation

### ODB++ Checker

```rust
let validation = OdbValidator::validate("./my_board.odb")?;

println!("Valid: {}", validation.is_valid());
println!("Warnings: {}", validation.warnings().len());
println!("Errors: {}", validation.errors().len());
```

### Export Report

```
ODB++ Export Report
═══════════════════

Output: ./my_board.odb.tgz
Size: 2.4 MB

Content:
  ✓ Matrix (12 layers)
  ✓ Step: pcb
  ✓ Netlist (245 nets)
  ✓ Components (156)
  ✓ Drill (5 tools, 234 hits)
  ✓ Profile (board + 2 cutouts)

Layers:
  top, gnd, pwr, bottom
  solder_mask_top, solder_mask_bottom
  silk_screen_top, silk_screen_bottom
  solder_paste_top, solder_paste_bottom
  drill, rout

Validation: PASSED
```

## Related Topics

- [Gerber Export](./gerber-export.md)
- [IPC-2581 Export](./ipc2581-export.md)
- [BOM Generation](./bom-pick-place.md)
