# Command-Line Interface (CLI)

## Overview

Hardware Tool provides a comprehensive command-line interface for batch processing, automation, and CI/CD integration. The CLI supports all major operations including export, DRC/ERC, and project management.

## Installation

```bash
# Install via cargo
cargo install hardware-tool-cli

# Or use from project
./target/release/hwt
```

## Basic Usage

```bash
hwt [OPTIONS] <COMMAND> [ARGS]

Commands:
  new         Create a new project
  open        Open existing project
  export      Export design files
  drc         Run design rule check
  erc         Run electrical rules check
  bom         Generate bill of materials
  convert     Convert between formats
  validate    Validate project files
  help        Show help information

Options:
  -v, --verbose    Verbose output
  -q, --quiet      Suppress output
  --config <FILE>  Use config file
  --version        Show version
```

## Project Commands

### Create New Project

```bash
# Create new project
hwt new my_project

# Create with template
hwt new my_project --template arduino

# Create in specific directory
hwt new my_project --path ./projects/
```

### Open Project

```bash
# Open project (launches GUI)
hwt open my_project.hwt

# Open in headless mode
hwt open my_project.hwt --headless
```

## Export Commands

### Gerber Export

```bash
# Export Gerbers with defaults
hwt export gerber my_board.hwt_pcb

# Export to specific directory
hwt export gerber my_board.hwt_pcb --output ./gerber/

# Export specific layers
hwt export gerber my_board.hwt_pcb --layers F.Cu,B.Cu,F.Mask,B.Mask

# Use Protel extensions
hwt export gerber my_board.hwt_pcb --protel
```

### Drill Export

```bash
# Export drill files
hwt export drill my_board.hwt_pcb --output ./gerber/

# Excellon format options
hwt export drill my_board.hwt_pcb --format excellon2 --units mm
```

### BOM Export

```bash
# Export BOM as CSV
hwt export bom my_project.hwt --format csv --output bom.csv

# Export grouped BOM
hwt export bom my_project.hwt --group-by value,footprint

# Export with specific fields
hwt export bom my_project.hwt --fields ref,value,footprint,mpn
```

### Pick-and-Place Export

```bash
# Export centroid file
hwt export centroid my_board.hwt_pcb --output centroid.csv

# Specify origin
hwt export centroid my_board.hwt_pcb --origin board-corner-ll
```

### 3D Export

```bash
# Export STEP file
hwt export step my_board.hwt_pcb --output board.step

# Export with options
hwt export step my_board.hwt_pcb --include-components --output board.step
```

### Netlist Export

```bash
# Export netlist
hwt export netlist my_schematic.hwt_sch --format spice --output circuit.cir

# Export as JSON
hwt export netlist my_schematic.hwt_sch --format json --output netlist.json
```

## Validation Commands

### Design Rule Check (DRC)

```bash
# Run DRC
hwt drc my_board.hwt_pcb

# Run with specific rules
hwt drc my_board.hwt_pcb --rules jlcpcb

# Output report
hwt drc my_board.hwt_pcb --output drc_report.json

# Fail on warnings
hwt drc my_board.hwt_pcb --strict
```

### Electrical Rules Check (ERC)

```bash
# Run ERC
hwt erc my_schematic.hwt_sch

# Output report
hwt erc my_schematic.hwt_sch --output erc_report.json
```

### DFM Check

```bash
# Run DFM analysis
hwt dfm my_board.hwt_pcb --manufacturer jlcpcb

# Generate report
hwt dfm my_board.hwt_pcb --output dfm_report.html
```

## Conversion Commands

### Format Conversion

```bash
# Convert KiCAD to Hardware Tool
hwt convert my_board.kicad_pcb --to hwt

# Convert schematic
hwt convert my_schematic.kicad_sch --to hwt

# Batch convert library
hwt convert ./kicad_libs/ --to hwt --output ./hwt_libs/
```

### Supported Conversions

| From | To |
|------|-----|
| KiCAD (.kicad_*) | Hardware Tool |
| Eagle (.brd, .sch) | Hardware Tool |
| Hardware Tool | KiCAD |

## Batch Processing

### Process Multiple Files

```bash
# Export all boards in directory
hwt export gerber ./boards/*.hwt_pcb --output ./output/

# Run DRC on all boards
hwt drc ./boards/*.hwt_pcb --report-dir ./reports/
```

### Script Mode

```bash
# Run script file
hwt script build.hwt_script

# Inline script
hwt script --inline "
  open my_project.hwt
  export gerber --output ./gerber/
  export bom --output bom.csv
  drc --strict
"
```

## Configuration

### Config File

```toml
# hwt.toml
[export]
gerber_output = "./output/gerber"
bom_format = "csv"
bom_fields = ["ref", "value", "footprint", "mpn"]

[drc]
rules = "jlcpcb"
strict = true

[defaults]
units = "mm"
```

### Using Config

```bash
# Use project config
hwt export gerber my_board.hwt_pcb

# Override config
hwt export gerber my_board.hwt_pcb --config custom.toml

# Ignore config
hwt export gerber my_board.hwt_pcb --no-config
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: PCB Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Hardware Tool
        run: cargo install hardware-tool-cli
      
      - name: Run ERC
        run: hwt erc schematic.hwt_sch --strict
      
      - name: Run DRC
        run: hwt drc board.hwt_pcb --strict
      
      - name: Run DFM
        run: hwt dfm board.hwt_pcb --manufacturer jlcpcb
      
      - name: Export Gerbers
        run: hwt export gerber board.hwt_pcb --output ./gerber/
      
      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: gerber-files
          path: ./gerber/
```

### GitLab CI Example

```yaml
stages:
  - validate
  - export

validate:
  stage: validate
  script:
    - hwt erc schematic.hwt_sch --strict
    - hwt drc board.hwt_pcb --strict

export:
  stage: export
  script:
    - hwt export gerber board.hwt_pcb --output ./gerber/
    - hwt export bom project.hwt --output bom.csv
  artifacts:
    paths:
      - gerber/
      - bom.csv
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | File not found |
| 4 | DRC/ERC errors |
| 5 | Export failed |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `HWT_CONFIG` | Default config file path |
| `HWT_LIBS` | Library search path |
| `HWT_OUTPUT` | Default output directory |
| `HWT_VERBOSE` | Enable verbose output |

## Related Topics

- [Project Structure](../core-architecture/project-structure-management.md)
- [Gerber Export](../manufacturing-output/gerber-export.md)
- [Design Rule Check](../pcb-layout/drc.md)
