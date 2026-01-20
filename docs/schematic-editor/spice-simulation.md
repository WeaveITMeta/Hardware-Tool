# SPICE Simulation Integration

## Overview

Hardware Tool integrates with SPICE simulators (primarily ngspice) to enable circuit simulation directly from schematics. Export netlists, run simulations, and visualize results without leaving the design environment.

## Simulation Workflow

```
┌─────────────────┐
│    Schematic    │
└────────┬────────┘
         ▼
┌─────────────────┐
│ SPICE Netlist   │
│   Generation    │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Simulation     │
│   (ngspice)     │
└────────┬────────┘
         ▼
┌─────────────────┐
│    Results      │
│  Visualization  │
└─────────────────┘
```

## SPICE Models

### Model Assignment

```rust
Component {
    reference: "Q1",
    value: "2N2222",
    spice_model: SpiceModel {
        type_: ModelType::NPN,
        model_name: "2N2222",
        library: "standard.lib",
    },
}
```

### Built-in Models

| Category | Examples |
|----------|----------|
| Diodes | 1N4148, 1N5819, LED |
| BJTs | 2N2222, 2N3904, BC547 |
| MOSFETs | 2N7000, IRF540, BSS138 |
| Op-Amps | LM741, TL072, OPA2134 |
| Regulators | LM7805, LM317 |

### Custom Models

```spice
* Custom MOSFET model
.MODEL CUSTOM_NMOS NMOS (
+ VTO=1.5 KP=2.0E-3 LAMBDA=0.02
+ CBD=5PF CBS=5PF PB=0.8
)
```

## Netlist Generation

### SPICE Netlist Format

```spice
* Hardware Tool SPICE Export
* Project: amplifier_stage
* Date: 2026-01-19

.INCLUDE "models/transistors.lib"

* Component definitions
R1 VIN BASE 10k
R2 VCC BASE 47k
R3 VCC COLLECTOR 4.7k
R4 EMITTER GND 1k
C1 VIN BASE 10u
C2 COLLECTOR VOUT 10u
Q1 COLLECTOR BASE EMITTER 2N2222

* Power supply
VCC VCC GND DC 12

* Input signal
VIN VIN_SRC GND SIN(0 100m 1k)

.END
```

### Export Options

```rust
SpiceExportConfig {
    format: SpiceFormat::Ngspice,
    include_models: true,
    include_subcircuits: true,
    
    // Node naming
    use_net_names: true,      // Use schematic net names
    ground_net: "GND",        // Map to SPICE ground (0)
    
    // Simulation directives
    include_analysis: true,
}
```

## Simulation Types

### DC Analysis

```spice
* DC operating point
.OP

* DC sweep
.DC VIN 0 5 0.1
```

```rust
Simulation::dc_sweep("VIN", 0.0, 5.0, 0.1)
```

### AC Analysis

```spice
* Frequency response
.AC DEC 100 1 1MEG
```

```rust
Simulation::ac_analysis()
    .sweep(Sweep::Decade)
    .points_per_decade(100)
    .start(1.0)
    .stop(1e6)
```

### Transient Analysis

```spice
* Time domain simulation
.TRAN 1u 10m
```

```rust
Simulation::transient()
    .step(1e-6)
    .stop(10e-3)
    .start(0.0)
```

### Parameter Sweep

```spice
* Component value sweep
.STEP PARAM Rval 1k 10k 1k
```

```rust
Simulation::parameter_sweep("R1", 1e3, 10e3, 1e3)
```

## Simulation Probes

### Voltage Probes

```rust
// Probe net voltage
schematic.add_probe(Probe::voltage("VOUT"));

// Differential voltage
schematic.add_probe(Probe::voltage_diff("IN+", "IN-"));
```

### Current Probes

```rust
// Current through component
schematic.add_probe(Probe::current("R1"));

// Current into pin
schematic.add_probe(Probe::current_pin("U1", "VCC"));
```

### Power Probes

```rust
// Power dissipation
schematic.add_probe(Probe::power("Q1"));
```

## Results Visualization

### Waveform Viewer

```
┌─────────────────────────────────────────────────────┐
│ Transient Analysis - VOUT                           │
├─────────────────────────────────────────────────────┤
│     │                                               │
│ 5V ─┤    ╭──╮    ╭──╮    ╭──╮    ╭──╮              │
│     │   ╱    ╲  ╱    ╲  ╱    ╲  ╱    ╲             │
│ 0V ─┤──╱──────╲╱──────╲╱──────╲╱──────╲────        │
│     │                                               │
│-5V ─┤                                               │
│     └───┬───────┬───────┬───────┬───────┬──► Time  │
│         0      2.5ms    5ms    7.5ms   10ms        │
└─────────────────────────────────────────────────────┘
```

### Measurement Tools

| Measurement | Description |
|-------------|-------------|
| **Cursor** | Point value readout |
| **Delta** | Difference between cursors |
| **Min/Max** | Signal extremes |
| **RMS** | Root mean square |
| **Frequency** | Period measurement |
| **Rise/Fall** | Edge timing |

### Bode Plot

```
┌─────────────────────────────────────────────────────┐
│ AC Analysis - Gain & Phase                          │
├─────────────────────────────────────────────────────┤
│ 40dB ─┤────────────╲                                │
│       │             ╲                               │
│ 20dB ─┤              ╲                              │
│       │               ╲                             │
│  0dB ─┤                ╲────────────────────        │
│       │                                             │
│-20dB ─┤                                             │
│       └───┬───────┬───────┬───────┬───────┬──► Hz  │
│          10     100      1k     10k    100k        │
└─────────────────────────────────────────────────────┘
```

## Simulation Directives

### In-Schematic Directives

Add SPICE commands directly to schematic:

```rust
SpiceDirective::new(".OPTIONS RELTOL=1e-4")
    .position(100, 200)
    .visible(true);
```

### Common Directives

```spice
* Simulation options
.OPTIONS RELTOL=1e-4 ABSTOL=1e-12

* Initial conditions
.IC V(VOUT)=0

* Include library
.INCLUDE "custom_models.lib"

* Subcircuit definition
.SUBCKT opamp IN+ IN- OUT VCC VEE
...
.ENDS
```

## Subcircuit Support

### Hierarchical Simulation

Sub-schematics become SPICE subcircuits:

```spice
.SUBCKT filter IN OUT GND
R1 IN MID 10k
C1 MID OUT 100n
R2 OUT GND 10k
.ENDS

X1 VIN VOUT GND filter
X2 VOUT VOUT2 GND filter
```

### Parameterized Subcircuits

```spice
.SUBCKT rc_filter IN OUT GND PARAMS: R=10k C=100n
R1 IN OUT {R}
C1 OUT GND {C}
.ENDS

X1 VIN VOUT GND rc_filter R=4.7k C=220n
```

## Integration with ngspice

### Configuration

```rust
NgspiceConfig {
    executable: "ngspice",
    working_dir: "./simulation",
    
    // Output options
    raw_file: true,
    print_results: false,
    
    // Performance
    threads: 4,
}
```

### Running Simulation

```rust
let netlist = schematic.export_spice(config);
let results = ngspice::simulate(&netlist)?;

// Access results
let vout = results.get_voltage("VOUT")?;
for (time, voltage) in vout.iter() {
    println!("{:.3}ms: {:.3}V", time * 1000.0, voltage);
}
```

## Related Topics

- [Schematic Capture Workflow](../core-architecture/schematic-capture-workflow.md)
- [Symbols & Libraries](./symbols-libraries.md)
- [Calculator Tools](../advanced-features/calculator-tools.md)
