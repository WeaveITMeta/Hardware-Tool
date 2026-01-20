# Project Structure & Management

## Overview

Hardware Tool uses a unified project file format designed for modern EDA workflows, combining the best aspects of KiCAD's `.kicad_pro` structure with enhanced Rust-native capabilities.

## Project File Format

### `.hwt` Project Format

The primary project format (`.hwt`) is a human-readable TOML-based configuration that defines:

- **Project metadata** - Name, version, author, license
- **File references** - Schematics, PCB layouts, libraries
- **Build configuration** - Output formats, manufacturing settings
- **Dependencies** - External libraries, component sources

```toml
[project]
name = "my-circuit"
version = "1.0.0"
format_version = "1"

[schematics]
main = "main.hwt_sch"
sheets = ["power.hwt_sch", "io.hwt_sch"]

[pcb]
layout = "board.hwt_pcb"
stackup = "4-layer-standard"

[libraries]
local = ["./libs"]
remote = ["https://hardware-tool.dev/libs/standard"]

[output]
gerber = true
ipc2581 = true
odbpp = false
```

### KiCAD Compatibility

Hardware Tool provides import/export compatibility with KiCAD projects:

- **Import**: `.kicad_pro`, `.kicad_sch`, `.kicad_pcb`
- **Export**: Native KiCAD format for interoperability

## Directory Structure

```
my-project/
├── project.hwt              # Main project file
├── schematics/
│   ├── main.hwt_sch
│   └── sheets/
├── pcb/
│   └── board.hwt_pcb
├── libraries/
│   ├── symbols/
│   └── footprints/
├── 3d-models/
├── output/
│   ├── gerber/
│   ├── ipc2581/
│   └── bom/
└── docs/
```

## Project Management Features

### Version Control Integration

- Git-friendly file formats (text-based, diff-able)
- `.hwtignore` for excluding generated files
- Lock files for reproducible builds

### Multi-User Collaboration

- File-level locking support
- Merge conflict resolution tools
- Change tracking and audit logs

### Build System

- Declarative build targets
- Incremental compilation
- Parallel processing for large designs

## Project-Wide Search

Hardware Tool provides comprehensive search capabilities across the entire project.

### Search Scope

| Scope | Searches |
|-------|----------|
| **Components** | Reference designators, values, footprints, properties |
| **Nets** | Net names, net classes, connections |
| **Symbols** | Library symbols, custom symbols |
| **Footprints** | Library footprints, pad names |
| **Properties** | Custom fields, manufacturer data, MPNs |
| **Text** | Schematic text, silkscreen, comments |

### Search Interface

```
┌─────────────────────────────────────────────────────────────────┐
│ 🔍 Search Project...                              [Ctrl+Shift+F]│
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Query: [STM32                                              ]    │
│                                                                 │
│ Scope: [All        ▼]  Type: [Any        ▼]                    │
│                                                                 │
│ ─────────────────────────────────────────────────────────────── │
│                                                                 │
│ Results (5 matches):                                            │
│                                                                 │
│ 📦 Components                                                   │
│   U1 - STM32F405RGT6 (main.hwt_sch:45,120)                     │
│   U3 - STM32G071 (sensors.hwt_sch:80,200)                      │
│                                                                 │
│ 📚 Symbols                                                      │
│   MCU_ST:STM32F405RGT6                                         │
│   MCU_ST:STM32G071CBT6                                         │
│                                                                 │
│ 📄 Text                                                         │
│   "STM32 Power Section" (power.hwt_sch:10,50)                  │
│                                                                 │
│ [Jump to Selected] [Replace] [Export Results]                   │
└─────────────────────────────────────────────────────────────────┘
```

### Search API

```rust
// Basic search
let results = project.search("STM32")?;

// Filtered search
let results = project.search_with_options(SearchOptions {
    query: "10k",
    scope: SearchScope::Components,
    match_type: MatchType::Exact,
    case_sensitive: false,
    include_libraries: false,
})?;

// Search by property
let results = project.search_property("manufacturer", "Texas Instruments")?;

// Search nets
let results = project.search_nets("VCC*")?;  // Wildcard support

// Regex search
let results = project.search_regex(r"R[0-9]{1,3}")?;
```

### Search Results

```rust
SearchResult {
    item_type: SearchItemType::Component,
    reference: "U1",
    value: "STM32F405RGT6",
    location: Location {
        file: "main.hwt_sch",
        position: Point { x: 45.0, y: 120.0 },
        sheet: Some(1),
    },
    context: "MCU Section",
    
    // Actions
    actions: vec![
        Action::JumpTo,
        Action::Select,
        Action::ShowProperties,
        Action::CrossProbe,
    ],
}
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+F` | Open project search |
| `Ctrl+F` | Search current view |
| `F3` | Find next |
| `Shift+F3` | Find previous |
| `Ctrl+H` | Search and replace |

### Search & Replace

```rust
// Replace component values
project.search_replace(SearchReplace {
    find: "100nF",
    replace: "100n",
    scope: SearchScope::ComponentValues,
    preview: true,  // Show changes before applying
})?;

// Replace net names
project.search_replace(SearchReplace {
    find: "VCC_3V3",
    replace: "VDD_3V3",
    scope: SearchScope::NetNames,
    preview: true,
})?;
```

## Related Topics

- [Circuit JSON as Intermediate Representation](./circuit-json-ir.md)
- [Programmatic Design](./programmatic-design.md)
