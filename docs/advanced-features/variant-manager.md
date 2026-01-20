# Variant Manager

## Overview

The Variant Manager enables "what-if" design exploration by creating parallel board configurations with different components, stackups, or design rules. Compare DRC results, cost estimates, and manufacturability across variants without duplicating project files.

## Variant Concepts

### What is a Variant?

A variant is a named configuration that modifies the base design:

```rust
Variant {
    name: "low_cost",
    description: "Budget version with alternative components",
    
    // What can vary
    component_substitutions: vec![...],
    dnp_components: vec![...],      // Do Not Populate
    stackup: Some("2-layer"),
    design_rules: Some("relaxed"),
}
```

### Variant Types

| Type | Description | Use Case |
|------|-------------|----------|
| **Component** | Substitute parts | Cost optimization, availability |
| **Assembly** | DNP certain parts | Feature tiers, debug versions |
| **Stackup** | Different layer counts | Cost vs. performance |
| **Region** | Geographic compliance | RoHS, REACH variants |

## Creating Variants

### From UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Variant Manager                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Base Design: my_board.hwt                                       │
│                                                                 │
│ Variants:                                                       │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ● base (active)                                             │ │
│ │   Full-featured production version                          │ │
│ │   Components: 45  |  Layers: 4  |  Est: $12.50             │ │
│ │                                                             │ │
│ │ ○ low_cost                                                  │ │
│ │   Budget version with alternative components                │ │
│ │   Components: 42  |  Layers: 2  |  Est: $6.80              │ │
│ │                                                             │ │
│ │ ○ debug                                                     │ │
│ │   Debug version with test points and headers               │ │
│ │   Components: 52  |  Layers: 4  |  Est: $14.20             │ │
│ │                                                             │ │
│ │ ○ rohs_compliant                                            │ │
│ │   RoHS-compliant component substitutions                    │ │
│ │   Components: 45  |  Layers: 4  |  Est: $13.10             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [New Variant] [Duplicate] [Compare] [Delete]                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Programmatic Creation

```rust
// Create variant from base
let low_cost = project.create_variant("low_cost", |v| {
    v.description("Budget version with alternative components");
    
    // Substitute components
    v.substitute("U1", "STM32F103C8T6", "STM32F103C6T6");  // Smaller flash
    v.substitute("C1", "100nF X7R", "100nF X5R");          // Cheaper cap
    
    // Remove optional components
    v.dnp(&["LED1", "LED2", "LED3"]);  // Status LEDs
    v.dnp(&["J3"]);                     // Debug header
    
    // Change stackup
    v.stackup("2-layer");
    
    // Relaxed design rules
    v.design_rules("standard");
});
```

## Component Substitutions

### Substitution Rules

```rust
Substitution {
    original: ComponentRef {
        reference: "U1",
        value: "STM32F405RGT6",
        footprint: "LQFP-64",
    },
    
    replacement: ComponentRef {
        value: "STM32F401RET6",      // Lower-spec MCU
        footprint: "LQFP-64",         // Same footprint
    },
    
    // Validation
    require_same_footprint: true,
    require_pin_compatible: true,
}
```

### Bulk Substitutions

```rust
// Substitute all 0402 caps with 0603
variant.substitute_by_footprint("C_0402_*", "C_0603_*");

// Substitute by manufacturer
variant.substitute_by_field("manufacturer", "Murata", "Yageo");

// Substitute by value range
variant.substitute_by_value("resistor", "1%", "5%");
```

## DNP (Do Not Populate)

### Mark Components as DNP

```rust
// Individual components
variant.dnp(&["R5", "R6", "C10"]);

// By reference pattern
variant.dnp_pattern("LED*");      // All LEDs
variant.dnp_pattern("TP*");       // All test points

// By schematic sheet
variant.dnp_sheet("debug_interface");
```

### DNP Visualization

```
┌─────────────────────────────────────────┐
│                                         │
│   [R1]  [R2]  [R3]  [R4]  [R5]         │
│                            ╳            │  ← DNP marker
│   [C1]  [C2]  [C3]  [C4]  [C5]         │
│                            ╳            │
│                                         │
│   Legend: ╳ = DNP in current variant    │
└─────────────────────────────────────────┘
```

## Stackup Variants

### Define Stackup Options

```rust
// 4-layer (base)
Stackup::new("4-layer-standard")
    .layer(Layer::signal("F.Cu"))
    .layer(Layer::plane("In1.Cu", "GND"))
    .layer(Layer::plane("In2.Cu", "VCC"))
    .layer(Layer::signal("B.Cu"));

// 2-layer (low cost)
Stackup::new("2-layer")
    .layer(Layer::signal("F.Cu"))
    .layer(Layer::signal("B.Cu"));

// Apply to variant
variant.stackup("2-layer");
```

### Stackup Impact Analysis

```
Stackup Comparison: 4-layer vs 2-layer
══════════════════════════════════════

                    4-layer     2-layer
─────────────────────────────────────────
Layer count:        4           2
Board cost:         $8.50       $3.20
Signal integrity:   Excellent   Good
EMI performance:    Excellent   Fair
Routing density:    High        Medium

Routing impact:
  - 12 nets require re-routing
  - 3 differential pairs affected
  - Ground plane continuity: WARN

[Apply] [Cancel] [Show Affected Nets]
```

## Variant Comparison

### Side-by-Side Comparison

```
┌─────────────────────────────────────────────────────────────────┐
│ Variant Comparison                                              │
├───────────────────────┬───────────────────────┬─────────────────┤
│ base                  │ low_cost              │ Difference      │
├───────────────────────┼───────────────────────┼─────────────────┤
│                       │                       │                 │
│ Components: 45        │ Components: 42        │ -3              │
│ Unique parts: 28      │ Unique parts: 25      │ -3              │
│                       │                       │                 │
│ Layers: 4             │ Layers: 2             │ -2              │
│ Board area: 50×40mm   │ Board area: 50×40mm   │ Same            │
│                       │                       │                 │
│ DRC errors: 0         │ DRC errors: 2         │ +2 ⚠️           │
│ DRC warnings: 3       │ DRC warnings: 8       │ +5              │
│                       │                       │                 │
│ BOM cost: $12.50      │ BOM cost: $6.80       │ -$5.70 (45%)   │
│ PCB cost: $8.50       │ PCB cost: $3.20       │ -$5.30 (62%)   │
│ Total: $21.00         │ Total: $10.00         │ -$11.00 (52%)  │
│                       │                       │                 │
├───────────────────────┴───────────────────────┴─────────────────┤
│ [View DRC Diff] [View BOM Diff] [Export Comparison]             │
└─────────────────────────────────────────────────────────────────┘
```

### DRC Comparison

```rust
let comparison = project.compare_variants_drc("base", "low_cost")?;

for diff in comparison.differences {
    match diff {
        DrcDiff::NewError { variant, error } => {
            println!("{}: New error - {}", variant, error);
        }
        DrcDiff::ResolvedError { variant, error } => {
            println!("{}: Resolved - {}", variant, error);
        }
    }
}
```

## Cost Estimation

### BOM Cost Analysis

```rust
CostEstimate {
    variant: "low_cost",
    
    // Component costs
    bom_cost: 6.80,
    bom_breakdown: vec![
        ("MCU", 2.50),
        ("Passives", 1.20),
        ("Connectors", 1.80),
        ("ICs", 1.30),
    ],
    
    // PCB costs
    pcb_cost: 3.20,
    pcb_breakdown: PcbCost {
        base: 2.00,
        layer_adder: 0.00,      // 2-layer = base
        area_adder: 0.50,
        finish_adder: 0.70,     // HASL
    },
    
    // Assembly
    assembly_cost: 5.00,
    
    // Total
    total_unit_cost: 15.00,
}
```

### Supplier Integration

```rust
// Fetch live pricing
let pricing = variant.fetch_pricing(PricingConfig {
    suppliers: vec!["LCSC", "DigiKey", "Mouser"],
    quantity: 100,
    include_shipping: true,
})?;

println!("Best price: ${} from {}", pricing.best_total, pricing.best_supplier);
```

## Variant Export

### Export Variant BOMs

```rust
// Export all variant BOMs
project.export_variant_boms(BomExportConfig {
    variants: vec!["base", "low_cost", "debug"],
    format: BomFormat::CSV,
    output_dir: "./output/boms/",
    
    // Naming
    filename_pattern: "{project}_{variant}_bom.csv",
})?;

// Output:
//   ./output/boms/my_board_base_bom.csv
//   ./output/boms/my_board_low_cost_bom.csv
//   ./output/boms/my_board_debug_bom.csv
```

### Export Variant Gerbers

```rust
// Export manufacturing files per variant
for variant in ["base", "low_cost"] {
    project.with_variant(variant, |p| {
        p.export_gerbers(&format!("./output/{}/gerber/", variant))?;
        p.export_bom(&format!("./output/{}/bom.csv", variant))?;
        p.export_pick_place(&format!("./output/{}/pnp.csv", variant))?;
    })?;
}
```

## Variant Switching

### Active Variant

```rust
// Switch active variant
project.set_active_variant("low_cost");

// All operations now use low_cost configuration
project.run_drc()?;           // DRC with 2-layer rules
project.generate_bom()?;      // BOM without DNP components
project.export_gerbers()?;    // 2-layer Gerbers
```

### Temporary Variant Context

```rust
// Temporarily work in variant context
project.with_variant("debug", |p| {
    // Add debug-only components
    p.add_component("J10", "DEBUG_HEADER")?;
    p.add_test_points(&["NET_A", "NET_B"])?;
})?;

// Back to base variant
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+V` | Open Variant Manager |
| `Ctrl+Alt+1-9` | Switch to variant 1-9 |
| `Ctrl+Shift+C` | Compare variants |
| `Ctrl+Shift+B` | Export all variant BOMs |

## Related Topics

- [BOM & Pick-and-Place](../manufacturing-output/bom-pick-place.md)
- [Design Rule Check](../pcb-layout/drc.md)
- [Multi-Layer Support](../pcb-layout/multi-layer.md)
