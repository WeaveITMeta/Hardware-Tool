# Symbols & Libraries

## Overview

Symbols are the graphical representations of electronic components in schematics. Hardware Tool provides a comprehensive library system supporting custom symbols, official libraries, and hierarchical component definitions.

## Symbol Anatomy

### Basic Structure

```
┌─────────────────────────┐
│      Component Name     │
│  ┌───────────────────┐  │
│  │                   │  │
│  │   Graphic Body    │  │
│  │                   │  │
│  └───────────────────┘  │
│  Pin 1 ●           ● Pin 2
│         Reference: U1   │
│         Value: 74HC00   │
└─────────────────────────┘
```

### Symbol Elements

| Element | Description |
|---------|-------------|
| **Graphic Body** | Visual representation (rectangles, lines, arcs) |
| **Pins** | Connection points with electrical types |
| **Reference** | Designator field (R, C, U, etc.) |
| **Value** | Component value or part number |
| **Properties** | Custom fields (footprint, datasheet, etc.) |

## Pin Definitions

### Electrical Types

```rust
pub enum PinElectricalType {
    Input,          // Signal input
    Output,         // Signal output
    Bidirectional,  // I/O pin
    TriState,       // High-impedance capable
    Passive,        // Resistors, capacitors
    Power,          // VCC, VDD
    Ground,         // GND, VSS
    OpenCollector,  // Open drain/collector
    OpenEmitter,    // Open emitter
    NotConnected,   // NC pins
    Unspecified,    // Unknown/generic
}
```

### Pin Properties

```rust
Pin {
    number: "1",
    name: "VCC",
    electrical_type: PinElectricalType::Power,
    position: Point { x: -100, y: 0 },
    length: 100,
    orientation: PinOrientation::Right,
    name_visible: true,
    number_visible: true,
}
```

## Symbol File Format

### `.hwt_sym` Format

```toml
[symbol]
name = "resistor"
reference_prefix = "R"
keywords = ["resistor", "res", "passive"]
description = "Standard resistor symbol"

[graphics]
body = [
    { type = "rectangle", start = [-30, -10], end = [30, 10] }
]

[[pins]]
number = "1"
name = "1"
position = [-50, 0]
length = 20
electrical_type = "passive"

[[pins]]
number = "2"
name = "2"
position = [50, 0]
length = 20
electrical_type = "passive"

[properties]
footprint = { value = "", visible = false }
datasheet = { value = "", visible = false }
```

## Library Organization

### Directory Structure

```
libraries/
├── official/
│   ├── Device.hwt_lib          # Basic components
│   ├── Connector.hwt_lib       # Connectors
│   ├── MCU_ST.hwt_lib          # STM32 microcontrollers
│   └── Analog_ADI.hwt_lib      # Analog Devices ICs
├── community/
│   └── ...
└── local/
    └── MyComponents.hwt_lib
```

### Library File Structure

```toml
[library]
name = "Device"
version = "1.0.0"
description = "Basic electronic components"

[symbols]
# Symbols are stored as separate files or embedded
resistor = "symbols/resistor.hwt_sym"
capacitor = "symbols/capacitor.hwt_sym"
```

## Creating Custom Symbols

### Using the GUI Editor

1. **File → New Symbol**
2. Draw graphic body using shape tools
3. Add pins with electrical types
4. Define properties and fields
5. Save to library

### Programmatic Definition

```rust
use hardware_tool::symbol::*;

let symbol = Symbol::builder("op_amp")
    .reference_prefix("U")
    .description("Operational Amplifier")
    .graphic(|g| {
        g.triangle([(0, -40), (0, 40), (60, 0)]);
    })
    .pin(Pin::new("1", "+").input().at(-20, 20))
    .pin(Pin::new("2", "-").input().at(-20, -20))
    .pin(Pin::new("3", "OUT").output().at(80, 0))
    .pin(Pin::new("4", "V+").power().at(30, -30))
    .pin(Pin::new("5", "V-").power().at(30, 30))
    .property("footprint", "")
    .build();
```

## Multi-Unit Symbols

### Definition

Components with multiple units (e.g., quad op-amp, dual flip-flop):

```rust
Symbol::builder("74HC00")
    .units(4)  // 4 NAND gates
    .unit(1, |u| {
        u.pins(&[("1", "A"), ("2", "B"), ("3", "Y")]);
    })
    .unit(2, |u| {
        u.pins(&[("4", "A"), ("5", "B"), ("6", "Y")]);
    })
    // ... units 3 and 4
    .common_pins(&[("7", "GND"), ("14", "VCC")])
    .build();
```

### Power Pin Handling

- **Shared power pins**: Displayed once, connected to all units
- **Per-unit power**: Each unit has own power connections
- **Hidden power pins**: Auto-connected to global power nets

## Hierarchical Symbols

### Sheet Symbols

Represent sub-sheets in hierarchical designs:

```rust
SheetSymbol::new("power_supply")
    .size(200, 150)
    .hierarchical_pin("VIN", Direction::Input)
    .hierarchical_pin("VOUT", Direction::Output)
    .hierarchical_pin("GND", Direction::Bidirectional)
    .sheet_file("power_supply.hwt_sch")
```

## Library Management

### Search & Browse

- Full-text search across all libraries
- Filter by category, manufacturer, package
- Preview symbol graphics and properties

### Parametric Library Browser

Filter components by electrical and physical parameters:

```
┌─────────────────────────────────────────────────────────────────┐
│ Library Browser                                    [Parametric] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Category: [Capacitors     ▼]                                    │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Filters:                                                    │ │
│ │                                                             │ │
│ │ Capacitance:  [100nF    ] to [100nF    ]                   │ │
│ │ Voltage:      [16V      ] to [Any      ]                   │ │
│ │ Package:      [☑ 0402] [☑ 0603] [☐ 0805] [☐ 1206]         │ │
│ │ Tolerance:    [Any           ▼]                             │ │
│ │ Dielectric:   [☑ X5R] [☑ X7R] [☐ C0G] [☐ Y5V]             │ │
│ │ Manufacturer: [Any           ▼]                             │ │
│ │                                                             │ │
│ │ [Apply Filters] [Clear] [Save as Preset]                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Results (24 matches):                                           │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ ▼ Samsung (12)                                            │   │
│ │   CL05B104KO5NNNC  100nF  0402  X5R  16V  [$0.01]        │   │
│ │   CL10B104KB8NNNC  100nF  0603  X7R  50V  [$0.02]        │   │
│ │                                                           │   │
│ │ ▼ Murata (8)                                              │   │
│ │   GRM155R71C104KA88D  100nF  0402  X7R  16V  [$0.01]     │   │
│ │                                                           │   │
│ │ ▼ Yageo (4)                                               │   │
│ │   CC0402KRX7R7BB104  100nF  0402  X7R  16V  [$0.01]      │   │
│ └───────────────────────────────────────────────────────────┘   │
│                                                                 │
│ [Add to Schematic] [Compare] [View Datasheet]                   │
└─────────────────────────────────────────────────────────────────┘
```

### Parametric Filter API

```rust
// Parametric search
let capacitors = library.search_parametric(ParametricFilter {
    category: Category::Capacitor,
    
    parameters: vec![
        Parameter::Capacitance { min: 100e-9, max: 100e-9 },
        Parameter::Voltage { min: 16.0, max: None },
        Parameter::Package(vec!["0402", "0603"]),
        Parameter::Dielectric(vec!["X5R", "X7R"]),
    ],
    
    // Sorting
    sort_by: SortBy::Price,
    sort_order: SortOrder::Ascending,
    
    // Availability
    in_stock: true,
    preferred_suppliers: vec!["LCSC", "DigiKey"],
})?;

// Filter resistors by value
let resistors = library.search_parametric(ParametricFilter {
    category: Category::Resistor,
    parameters: vec![
        Parameter::Resistance { min: 10e3, max: 10e3 },
        Parameter::Tolerance { max_percent: 1.0 },
        Parameter::Power { min: 0.1, max: None },
        Parameter::Package(vec!["0603"]),
    ],
    ..Default::default()
})?;
```

### Parameter Types by Category

| Category | Available Parameters |
|----------|---------------------|
| **Resistors** | Resistance, tolerance, power, package, TCR |
| **Capacitors** | Capacitance, voltage, tolerance, dielectric, package |
| **Inductors** | Inductance, current, DCR, package, shielded |
| **Diodes** | Forward voltage, current, reverse voltage, package |
| **Transistors** | Type (NPN/PNP/N-CH/P-CH), Vds, Id, Rds(on), package |
| **ICs** | Function, supply voltage, package, temperature range |
| **Connectors** | Pitch, pins, rows, mounting, current rating |

### Filter Presets

```rust
// Save filter preset
library.save_filter_preset("decoupling_caps", ParametricFilter {
    category: Category::Capacitor,
    parameters: vec![
        Parameter::Capacitance { min: 100e-9, max: 100e-9 },
        Parameter::Package(vec!["0402", "0603"]),
        Parameter::Dielectric(vec!["X5R", "X7R"]),
    ],
    ..Default::default()
});

// Load preset
let filter = library.load_filter_preset("decoupling_caps")?;
let results = library.search_parametric(filter)?;
```

### Import/Export

| Format | Import | Export |
|--------|--------|--------|
| KiCAD `.kicad_sym` | ✓ | ✓ |
| Eagle `.lbr` | ✓ | - |
| Altium `.SchLib` | ✓ | - |
| Custom JSON | ✓ | ✓ |

### Library Conventions

Following KLC-style guidelines:

1. **Naming**: `Manufacturer_PartFamily_Package`
2. **Pin placement**: Inputs left, outputs right
3. **Reference origin**: Center of symbol
4. **Grid alignment**: 50mil (1.27mm) grid

## Related Topics

- [Hierarchical Schematics](./hierarchical-schematics.md)
- [Annotation & Reference Designators](./annotation-reference-designators.md)
- [Library Conventions](../advanced-features/library-conventions.md)
