# Automatic Routing & Layout Engines

## Overview

Hardware Tool provides automatic routing capabilities inspired by TSCircuit's layout engines. These tools automate trace routing while respecting design rules and constraints, significantly reducing manual routing effort for complex designs.

## Auto-Layout Engines

### pcbFlex

Flexible constraint-based auto-layout:

```rust
use hardware_tool::autoroute::PcbFlex;

let router = PcbFlex::new()
    // Routing strategy
    .strategy(Strategy::MinimizeVias)
    
    // Layer preferences
    .horizontal_layers(&[Layer::FCu, Layer::In2Cu])
    .vertical_layers(&[Layer::In1Cu, Layer::BCu])
    
    // Constraints
    .max_vias_per_net(4)
    .prefer_45_degree(true)
    
    .build();

router.route(&mut pcb)?;
```

### pcbGrid

Grid-based routing for regular structures:

```rust
use hardware_tool::autoroute::PcbGrid;

let router = PcbGrid::new()
    .grid_size(0.5)  // 0.5mm grid
    .channel_width(3)  // 3 tracks per channel
    
    // BGA fanout
    .bga_escape(BgaEscape {
        pattern: EscapePattern::Diagonal,
        via_in_pad: true,
    })
    
    .build();
```

### pcbPack

Density-optimized routing:

```rust
use hardware_tool::autoroute::PcbPack;

let router = PcbPack::new()
    .target_density(0.6)  // 60% routing density
    .balance_layers(true)
    .minimize_crosstalk(true)
    .build();
```

## Routing Strategies

### Strategy Selection

| Strategy | Best For | Trade-offs |
|----------|----------|------------|
| `MinimizeVias` | Signal integrity | Longer routes |
| `MinimizeLength` | High-speed | More vias |
| `BalanceLayers` | Manufacturing | Moderate |
| `MaximizeClearance` | EMI sensitive | Lower density |

### Strategy Configuration

```rust
RoutingStrategy {
    primary: Strategy::MinimizeVias,
    fallback: Strategy::MinimizeLength,
    
    // Weights
    via_cost: 10.0,
    length_cost: 1.0,
    layer_change_cost: 5.0,
    
    // Limits
    max_iterations: 1000,
    timeout_seconds: 60,
}
```

## Net Ordering

### Priority-Based Routing

```rust
NetPriority {
    // Route critical nets first
    high: vec!["CLK", "USB_D+", "USB_D-"],
    
    // Route power last (use planes)
    low: vec!["VCC", "GND"],
    
    // Default priority for others
    default: Priority::Medium,
}
```

### Automatic Ordering

```rust
AutoOrder {
    // Order by constraint complexity
    differential_pairs_first: true,
    length_matched_first: true,
    
    // Order by connectivity
    short_nets_first: true,
}
```

## Fanout Routing

### BGA Fanout

```rust
BgaFanout {
    component: "U1",
    
    // Escape pattern
    pattern: FanoutPattern::DogBone,
    
    // Via placement
    via_in_pad: true,
    via_offset: 0.5,  // mm from pad center
    
    // Layer assignment
    inner_rows: Layer::In1Cu,
    outer_rows: Layer::FCu,
}
```

### Fanout Patterns

```
Dog-bone:           Via-in-pad:         Staggered:
  ●─○                  ⊙                  ●─○
  ●─○                  ⊙                    ●─○
  ●─○                  ⊙                  ●─○
```

## Differential Pair Routing

### Auto-Route Diff Pairs

```rust
DiffPairRouter {
    pairs: vec![
        DiffPair::new("USB_D+", "USB_D-").impedance(90.0),
        DiffPair::new("ETH_TX+", "ETH_TX-").impedance(100.0),
    ],
    
    // Matching
    max_skew: 0.1,  // mm
    
    // Geometry
    maintain_gap: true,
    gap_tolerance: 0.05,
}
```

## Length Matching

### Auto Length Tuning

```rust
LengthMatcher {
    groups: vec![
        MatchGroup::new("DDR_DQ")
            .nets(&["DQ0", "DQ1", "DQ2", "DQ3"])
            .tolerance(0.5),
        
        MatchGroup::new("DDR_ADDR")
            .nets(&["A0", "A1", "A2", "A3"])
            .reference("CLK")
            .tolerance(1.0),
    ],
    
    // Tuning style
    meander_style: MeanderStyle::Rounded,
    meander_spacing: 0.3,
}
```

## Routing Regions

### Region Definition

```rust
RoutingRegion::new("analog_section")
    .bounds(Rect::new(10.0, 10.0, 40.0, 40.0))
    .rules(RegionRules {
        min_clearance: 0.3,
        preferred_width: 0.25,
        no_vias: false,
        single_layer: Some(Layer::FCu),
    });
```

### Keep-Out Regions

```rust
KeepOut::new("mounting_hole")
    .shape(Circle::new(50.0, 50.0, 4.0))
    .affects(KeepOutType::Routing);
```

## Progress & Feedback

### Routing Progress

```
Auto-Routing Progress
═════════════════════

Phase 1: Fanout
  [████████████████████] 100% - 45 nets

Phase 2: Critical Nets
  [████████████░░░░░░░░]  60% - 12/20 nets
  Current: USB_D+ (differential pair)

Phase 3: Remaining Nets
  [░░░░░░░░░░░░░░░░░░░░]   0% - 0/180 nets

Statistics:
  Routed: 57/245 nets (23%)
  Vias: 34
  Unrouted: 188
```

### Routing Report

```
Auto-Route Report
═════════════════

Summary:
  Total nets: 245
  Routed: 240 (98%)
  Unrouted: 5

Unrouted Nets:
  NET_A: Blocked by keep-out
  NET_B: No valid path (clearance)
  NET_C: Length constraint impossible
  NET_D: Differential pair mismatch
  NET_E: Via limit exceeded

Statistics:
  Total vias: 156
  Total length: 2,450mm
  Layer usage:
    F.Cu:   35%
    In1.Cu: 28%
    In2.Cu: 25%
    B.Cu:   32%
```

## Manual Intervention

### Partial Auto-Route

```rust
// Route only selected nets
router.route_nets(&["NET_A", "NET_B", "NET_C"]);

// Route within region
router.route_region(region);

// Complete unfinished routes
router.complete_routes();
```

### Rip-Up and Retry

```rust
RipUpRetry {
    max_attempts: 3,
    rip_up_radius: 5.0,  // mm
    
    // Nets to rip up
    allow_rip_up: vec!["low_priority_*"],
    protect: vec!["CLK", "USB_*"],
}
```

## Performance Optimization

### Multi-Threading

```rust
RouterConfig {
    threads: 8,
    
    // Parallel routing
    parallel_nets: true,
    
    // Memory limits
    max_memory_mb: 4096,
}
```

### Incremental Routing

```rust
// Only route changed nets
router.incremental_route(&changed_nets);

// Preserve existing routes
router.preserve_routes(true);
```

## Related Topics

- [Interactive Routing](./interactive-routing.md)
- [Component Placement](./component-placement.md)
- [Design Rule Check](./drc.md)
