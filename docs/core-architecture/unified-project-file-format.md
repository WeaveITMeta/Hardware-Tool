# Unified Project File Format

## Overview

Hardware Tool uses a unified, human-readable project file format designed for modern EDA workflows. The format combines the organizational strengths of KiCAD's `.kicad_pro` structure with enhanced Rust-native capabilities, Git-friendly text serialization, and seamless integration with programmatic workflows.

## File Extensions

| Extension | Purpose |
|-----------|---------|
| `.hwt` | Main project file (TOML) |
| `.hwt_sch` | Schematic file (JSON) |
| `.hwt_pcb` | PCB layout file (JSON) |
| `.hwt_lib` | Symbol library |
| `.hwt_fplib` | Footprint library |
| `.hwt_sym` | Individual symbol |
| `.hwt_fp` | Individual footprint |

## Project File Structure (.hwt)

### Root Configuration

```toml
[project]
name = "smart-sensor-board"
version = "1.2.0"
format_version = "1"
uuid = "550e8400-e29b-41d4-a716-446655440000"
created = "2026-01-15T10:30:00Z"
modified = "2026-01-19T16:00:00Z"

[metadata]
author = "Engineering Team"
company = "Acme Electronics"
license = "MIT"
description = "IoT sensor board with BLE connectivity"
tags = ["iot", "sensor", "ble", "low-power"]

[schematic]
root = "schematics/main.hwt_sch"
sheets = [
    "schematics/power.hwt_sch",
    "schematics/mcu.hwt_sch",
    "schematics/sensors.hwt_sch",
    "schematics/connectivity.hwt_sch"
]

[pcb]
board = "pcb/board.hwt_pcb"
stackup = "4-layer-1.6mm"
constraints = "pcb/constraints.toml"

[libraries]
symbols = [
    "${HWTOOLS_LIBS}/symbols",
    "./libraries/symbols"
]
footprints = [
    "${HWTOOLS_LIBS}/footprints",
    "./libraries/footprints"
]
models_3d = [
    "${HWTOOLS_LIBS}/3dmodels",
    "./libraries/3dmodels"
]

[output]
directory = "./output"
gerber = { enabled = true, format = "rs274x" }
ipc2581 = { enabled = true }
odbpp = { enabled = false }
bom = { enabled = true, format = "csv", grouped = true }
centroid = { enabled = true, units = "mm" }

[simulation]
spice_models = "./simulation/models"
default_simulator = "ngspice"

[ai]
enabled = true
provider = "openai"
model = "gpt-4-turbo"
# API key stored in environment or secure keychain
features = ["autoroute", "placement", "optimization", "review"]

[versioning]
vcs = "git"
auto_checkpoint = true
checkpoint_interval = 300  # seconds
```

## Schematic File Structure (.hwt_sch)

### JSON Schema

```json
{
  "$schema": "https://hardware-tool.dev/schemas/schematic-v1.json",
  "version": "1.0",
  "uuid": "sheet-uuid-here",
  "metadata": {
    "title": "Power Supply",
    "sheet_number": 2,
    "total_sheets": 5,
    "revision": "A",
    "date": "2026-01-19"
  },
  "page": {
    "size": "A4",
    "orientation": "landscape",
    "title_block": { ... }
  },
  "components": [
    {
      "uuid": "component-uuid",
      "reference": "U1",
      "value": "LM1117-3.3",
      "symbol": "Regulator_Linear:LM1117-3.3",
      "footprint": "Package_TO_SOT_SMD:SOT-223-3",
      "position": { "x": 100.0, "y": 50.0 },
      "rotation": 0,
      "mirror": false,
      "properties": {
        "manufacturer": "Texas Instruments",
        "mpn": "LM1117IMPX-3.3",
        "datasheet": "https://...",
        "dnp": false
      },
      "fields": [ ... ]
    }
  ],
  "wires": [
    {
      "uuid": "wire-uuid",
      "points": [[100, 50], [150, 50], [150, 75]],
      "net": "VCC_3V3"
    }
  ],
  "labels": [ ... ],
  "power_symbols": [ ... ],
  "buses": [ ... ],
  "junctions": [ ... ],
  "no_connects": [ ... ],
  "sheet_instances": [ ... ],
  "text_items": [ ... ],
  "graphics": [ ... ]
}
```

## PCB File Structure (.hwt_pcb)

### JSON Schema

```json
{
  "$schema": "https://hardware-tool.dev/schemas/pcb-v1.json",
  "version": "1.0",
  "uuid": "board-uuid-here",
  "metadata": {
    "title": "Smart Sensor Board",
    "revision": "A",
    "date": "2026-01-19"
  },
  "board": {
    "outline": {
      "type": "polygon",
      "points": [[0,0], [100,0], [100,80], [0,80]]
    },
    "thickness": 1.6,
    "layers": 4
  },
  "stackup": {
    "layers": [
      { "name": "F.Cu", "type": "copper", "thickness": 0.035 },
      { "name": "Prepreg1", "type": "dielectric", "thickness": 0.2, "er": 4.5 },
      { "name": "In1.Cu", "type": "copper", "thickness": 0.035 },
      { "name": "Core", "type": "dielectric", "thickness": 1.0, "er": 4.5 },
      { "name": "In2.Cu", "type": "copper", "thickness": 0.035 },
      { "name": "Prepreg2", "type": "dielectric", "thickness": 0.2, "er": 4.5 },
      { "name": "B.Cu", "type": "copper", "thickness": 0.035 }
    ]
  },
  "design_rules": {
    "clearance": 0.15,
    "track_width": { "min": 0.1, "default": 0.2 },
    "via": { "diameter": 0.6, "drill": 0.3 },
    "diff_pair": { "width": 0.1, "gap": 0.15 }
  },
  "net_classes": [ ... ],
  "footprints": [ ... ],
  "tracks": [ ... ],
  "vias": [ ... ],
  "zones": [ ... ],
  "graphics": [ ... ],
  "dimensions": [ ... ]
}
```

## Circuit JSON Intermediate Representation

The unified format uses Circuit JSON as the canonical intermediate representation for data exchange between schematic and PCB, enabling:

- Programmatic circuit generation
- External tool integration
- AI-powered optimization
- Version diffing and merging

See [Circuit JSON IR](./circuit-json-ir.md) for detailed schema.

## KiCAD Compatibility

### Import Support

```rust
// Import KiCAD project
let project = KicadImporter::import("design.kicad_pro")?;
project.save_as("design.hwt")?;
```

### Export Support

```rust
// Export to KiCAD format
let project = Project::open("design.hwt")?;
KicadExporter::export(&project, "design.kicad_pro")?;
```

### Supported KiCAD Versions

| Version | Import | Export |
|---------|--------|--------|
| KiCAD 8.x | ✓ | ✓ |
| KiCAD 7.x | ✓ | ✓ |
| KiCAD 6.x | ✓ | - |
| KiCAD 5.x | ✓ | - |

## Version Control Integration

### Git-Friendly Design

- Text-based formats (TOML, JSON)
- Deterministic serialization (sorted keys)
- Meaningful diffs
- Merge-friendly structure

### .gitignore Template

```gitignore
# Hardware Tool
*.hwt_backup
*.hwt_autosave
.hwt_cache/
output/

# Keep in version control
!*.hwt
!*.hwt_sch
!*.hwt_pcb
!libraries/
```

### Lock Files

```toml
# project.hwt.lock
[dependencies]
"Device" = { version = "1.2.3", checksum = "sha256:..." }
"MCU_ST" = { version = "2.0.1", checksum = "sha256:..." }
```

## Related Topics

- [Project Structure & Management](./project-structure-management.md)
- [Circuit JSON IR](./circuit-json-ir.md)
- [Programmatic Design](./programmatic-design.md)
