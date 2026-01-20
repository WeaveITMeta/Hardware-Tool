# Electrical Rules Check (ERC)

## Overview

The Electrical Rules Check (ERC) validates schematic connectivity and electrical correctness before proceeding to PCB layout. It catches common design errors that could result in non-functional circuits.

## ERC Categories

### Pin Connection Validation

Checks compatibility between connected pins based on electrical types:

```
Pin Type Matrix (● = OK, ⚠ = Warning, ✗ = Error)

              │ Input │Output│Bidir │Power │Ground│Passive│
──────────────┼───────┼──────┼──────┼──────┼──────┼───────┤
Input         │   ⚠   │  ●   │  ●   │  ●   │  ●   │   ●   │
Output        │   ●   │  ✗   │  ⚠   │  ✗   │  ✗   │   ●   │
Bidirectional │   ●   │  ⚠   │  ●   │  ●   │  ●   │   ●   │
Power         │   ●   │  ✗   │  ●   │  ●   │  ✗   │   ●   │
Ground        │   ●   │  ✗   │  ●   │  ✗   │  ●   │   ●   │
Passive       │   ●   │  ●   │  ●   │  ●   │  ●   │   ●   │
```

### Common Pin Errors

| Error | Description |
|-------|-------------|
| **Output Conflict** | Two outputs driving same net |
| **Undriven Input** | Input pin with no driver |
| **Power Short** | VCC connected to GND |
| **Floating Pin** | Pin not connected to any net |

## Unconnected Pin Detection

### Pin States

```rust
pub enum PinConnectionState {
    Connected,           // Part of a valid net
    Unconnected,         // No connection (error)
    NoConnect,           // Intentionally unconnected (NC flag)
    PowerFlag,           // Connected via power symbol
}
```

### No-Connect Markers

Explicitly mark intentionally unconnected pins:

```
    ┌─────────┐
    │   U1    │
    │         ├──● VCC
    │         ├──● GND
    │         ├──✕ NC    ← No-connect marker
    │         ├──● DATA
    └─────────┘
```

## Duplicate Label Detection

### Label Conflicts

```
Sheet 1:                    Sheet 2:
┌─────────────────┐        ┌─────────────────┐
│                 │        │                 │
│  CLK ●──────    │        │  CLK ●──────    │
│      (local)    │        │      (local)    │
│                 │        │                 │
│  >CLK ●─────    │        │  >CLK ●─────    │
│      (global)   │        │      (global)   │
└─────────────────┘        └─────────────────┘

Local labels: OK (different sheets)
Global labels: WARNING (same net name, verify intent)
```

### Label Rules

| Rule | Severity | Description |
|------|----------|-------------|
| Duplicate global | Warning | Same global label on multiple sheets |
| Orphan label | Error | Label with no matching connection |
| Case mismatch | Warning | `VCC` vs `Vcc` vs `vcc` |

## Power Connection Validation

### Power Pin Checks

```rust
ErcRule::PowerPins {
    require_power_flag: true,    // Power pins need power symbols
    check_voltage_levels: true,  // Verify compatible voltages
    allow_implicit_power: false, // Require explicit connections
}
```

### Power Net Validation

```
Error: Power pin U1.VDD not connected to power net
Warning: Mixed voltage levels on net VLOGIC (3.3V and 5V sources)
Info: Power net VCC has 5 consumers, 1 source
```

## Hierarchical ERC

### Cross-Sheet Validation

- Sheet pin ↔ hierarchical label matching
- Missing sheet pins
- Orphaned hierarchical labels
- Type mismatches (input vs output)

### Hierarchy Errors

```
Error: Sheet pin "SPI_CLK" has no matching hierarchical label
       in sub-sheet "spi_interface.hwt_sch"

Warning: Hierarchical label "DEBUG_TX" in "mcu.hwt_sch"
         has no corresponding sheet pin on parent
```

## ERC Configuration

### Rule Severity

```rust
ErcConfig {
    rules: vec![
        ErcRule::UnconnectedPin { severity: Severity::Error },
        ErcRule::InputToInput { severity: Severity::Warning },
        ErcRule::DuplicateReference { severity: Severity::Error },
        ErcRule::MissingValue { severity: Severity::Warning },
    ],
    
    // Exclusions
    ignore_pins: vec!["NC", "DNC"],
    ignore_nets: vec!["TESTPOINT_*"],
}
```

### Severity Levels

| Level | Action Required |
|-------|-----------------|
| **Error** | Must fix before netlist export |
| **Warning** | Review recommended |
| **Info** | Informational only |
| **Ignore** | Suppressed |

## ERC Report

### Report Format

```
═══════════════════════════════════════════════════════════
ERC Report - my_project.hwt_sch
Generated: 2026-01-19 16:20:00
═══════════════════════════════════════════════════════════

Summary:
  Errors:   2
  Warnings: 5
  Info:     3

───────────────────────────────────────────────────────────
ERRORS
───────────────────────────────────────────────────────────

[E001] Output driving output
  Location: Sheet 1, (125.0, 75.0)
  Net: DATA_OUT
  Pins: U1.PB0 (Output) ↔ U2.PA5 (Output)
  
[E002] Unconnected input pin
  Location: Sheet 2, (200.0, 150.0)
  Component: U3 (74HC00)
  Pin: 2 (Input)

───────────────────────────────────────────────────────────
WARNINGS
───────────────────────────────────────────────────────────

[W001] Input pin driven by passive
  Location: Sheet 1, (80.0, 120.0)
  Net: SENSE
  Pins: U1.ADC0 (Input) ← R5.2 (Passive)
  Suggestion: Add buffer or verify signal integrity
```

### Interactive Navigation

- Click error to jump to location
- Filter by severity
- Group by error type
- Export to JSON/CSV

## ERC Markers

### Visual Indicators

```
┌─────────┐
│   U1    │
│         ├──● VCC
│         ├──⚠ FLOAT   ← Warning marker
│         ├──✗ ERROR   ← Error marker
│         ├──● GND
└─────────┘
```

### Marker Management

```rust
// Suppress specific marker
schematic.erc_exclude(ErcExclusion {
    location: Point::new(125.0, 75.0),
    rule: "E001",
    reason: "Directly connected outputs are intentional (wired-OR)",
});
```

## Best Practices

1. **Run ERC frequently** during schematic entry
2. **Fix errors immediately** before they compound
3. **Document exclusions** with clear reasoning
4. **Use no-connect markers** for unused pins
5. **Verify power connections** before layout

## Related Topics

- [Wiring & Connectivity](./wiring-connectivity.md)
- [Annotation & Reference Designators](./annotation-reference-designators.md)
- [Schematic Capture Workflow](../core-architecture/schematic-capture-workflow.md)
