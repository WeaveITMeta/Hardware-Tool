# Programmatic / Code-First Design

## Overview

Hardware Tool embraces a code-first approach to circuit design, inspired by TSCircuit's methodology. Define circuits programmatically using Rust code and macros for maximum modularity, reusability, and version control friendliness.

## Philosophy

Traditional EDA tools treat schematics as graphical documents. Hardware Tool treats circuits as **code**:

- **Version Control**: Diff, merge, and review circuit changes
- **Reusability**: Create parameterized, reusable modules
- **Testing**: Unit test circuit configurations
- **Automation**: Generate variants programmatically
- **Documentation**: Code is self-documenting

## Basic Circuit Definition

### Component Declaration

```rust
use hardware_tool::prelude::*;

fn main() {
    let circuit = Circuit::new("led_blinker");
    
    // Define components
    let mcu = circuit.add(Mcu::atmega328p("U1"));
    let led = circuit.add(Led::red("D1"));
    let resistor = circuit.add(Resistor::new("R1", "330"));
    
    // Define connections
    circuit.connect(&mcu.pin("PB0"), &resistor.pin("1"));
    circuit.connect(&resistor.pin("2"), &led.pin("A"));
    circuit.connect(&led.pin("K"), &circuit.gnd());
    
    circuit.export_netlist("blinker.json");
}
```

### Using Macros

```rust
use hardware_tool::prelude::*;

circuit! {
    name: "voltage_regulator",
    
    components: {
        U1: Ldo::ams1117_3v3(),
        C1: Capacitor::ceramic("10uF", "0805"),
        C2: Capacitor::ceramic("10uF", "0805"),
    },
    
    nets: {
        VIN: [U1.VIN, C1.1],
        VOUT: [U1.VOUT, C2.1],
        GND: [U1.GND, C1.2, C2.2],
    }
}
```

## Parameterized Modules

### Reusable Subcircuits

```rust
/// Configurable RC low-pass filter
pub struct RcFilter {
    cutoff_hz: f64,
    impedance: f64,
}

impl RcFilter {
    pub fn new(cutoff_hz: f64, impedance: f64) -> Self {
        Self { cutoff_hz, impedance }
    }
    
    pub fn build(&self, circuit: &mut Circuit, prefix: &str) -> FilterPins {
        // Calculate component values
        let capacitance = 1.0 / (2.0 * PI * self.cutoff_hz * self.impedance);
        let resistance = self.impedance;
        
        // Create components
        let r = circuit.add(Resistor::new(
            &format!("{}R1", prefix),
            &format_resistance(resistance),
        ));
        let c = circuit.add(Capacitor::new(
            &format!("{}C1", prefix),
            &format_capacitance(capacitance),
        ));
        
        // Internal connections
        circuit.connect(&r.pin("2"), &c.pin("1"));
        
        // Return interface pins
        FilterPins {
            input: r.pin("1"),
            output: c.pin("1"),
            ground: c.pin("2"),
        }
    }
}
```

### Using Modules

```rust
fn main() {
    let mut circuit = Circuit::new("audio_filter");
    
    // Create multiple filter stages
    let lpf_100hz = RcFilter::new(100.0, 10_000.0).build(&mut circuit, "LPF1_");
    let lpf_1khz = RcFilter::new(1000.0, 10_000.0).build(&mut circuit, "LPF2_");
    
    // Chain filters
    circuit.connect(&lpf_100hz.output, &lpf_1khz.input);
}
```

## Component Libraries

### Standard Library

```rust
use hardware_tool::components::*;

// Passive components
Resistor::new("R1", "10k")
Capacitor::ceramic("C1", "100nF", "0402")
Inductor::new("L1", "10uH")

// Semiconductors
Diode::schottky("D1", "BAT54")
Led::smd("D2", "0603", Color::Green)
Transistor::nmos("Q1", "2N7002")

// ICs
Mcu::stm32f4("U1")
OpAmp::dual("U2", "LM358")
Regulator::ldo("U3", 3.3, "AMS1117")
```

### Custom Components

```rust
#[derive(Component)]
#[component(symbol = "custom_ic", footprint = "QFN-32")]
pub struct CustomChip {
    #[pin(1, "VDD", electrical = Power)]
    vdd: Pin,
    
    #[pin(2, "GND", electrical = Ground)]
    gnd: Pin,
    
    #[pin(3..=10, "DATA{}", electrical = Bidirectional)]
    data: [Pin; 8],
}
```

## Layout Hints

### Placement Constraints

```rust
circuit.layout_hints(|layout| {
    // Group components
    layout.group("power", &[&u1, &c1, &c2, &l1]);
    
    // Position constraints
    layout.place(&mcu).at(50.0, 40.0);
    layout.place(&crystal).near(&mcu).distance(5.0);
    
    // Alignment
    layout.align_horizontal(&[&r1, &r2, &r3, &r4]);
    
    // Keep-out
    layout.keepout_zone(Rect::new(0.0, 0.0, 10.0, 10.0));
});
```

### Routing Constraints

```rust
circuit.routing_hints(|routing| {
    // Differential pairs
    routing.differential_pair(&usb_dp, &usb_dm)
        .impedance(90.0)
        .max_skew(0.1);
    
    // Length matching
    routing.match_length(&[&ddr_dq0, &ddr_dq1, &ddr_dq2])
        .tolerance(1.0);
    
    // Net classes
    routing.net_class("power")
        .width(0.5)
        .clearance(0.3);
});
```

## Testing Circuits

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_values() {
        let filter = RcFilter::new(1000.0, 10_000.0);
        let mut circuit = Circuit::new("test");
        let pins = filter.build(&mut circuit, "");
        
        // Verify component values
        let r = circuit.get_component("R1").unwrap();
        assert_eq!(r.value(), "10k");
        
        let c = circuit.get_component("C1").unwrap();
        assert_eq!(c.value(), "15.9nF"); // Calculated value
    }
    
    #[test]
    fn test_connectivity() {
        let circuit = build_my_circuit();
        
        // Verify all nets are connected
        assert!(circuit.erc().is_ok());
        
        // Check specific connections
        assert!(circuit.are_connected("U1.VDD", "VCC"));
    }
}
```

## Integration with GUI

The programmatic design integrates seamlessly with the graphical editor:

1. **Code → GUI**: Generate schematic visualization from code
2. **GUI → Code**: Export graphical edits as code
3. **Hybrid**: Mix code-defined and graphically-edited sections

## Related Topics

- [Circuit JSON IR](./circuit-json-ir.md)
- [Symbols & Libraries](../schematic-editor/symbols-libraries.md)
- [CLI Tools](../advanced-features/cli.md)
