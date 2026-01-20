# Net Inspection & Highlighting

## Overview

Net inspection tools help visualize and analyze electrical connectivity in PCB layouts. Hardware Tool provides real-time net tracing, ratsnest display, and comprehensive net analysis features.

## Net Highlighting

### Single Net Highlight

```rust
// Highlight specific net
pcb.highlight_net("VCC");

// Highlight with options
pcb.highlight_net_with_options("CLK", HighlightOptions {
    color: Color::Yellow,
    include_pads: true,
    include_vias: true,
    include_zones: true,
    dim_others: 0.3,  // Dim other nets to 30%
});
```

### Multi-Net Highlight

```rust
// Highlight multiple nets
pcb.highlight_nets(&["USB_D+", "USB_D-"]);

// Highlight by pattern
pcb.highlight_nets_matching("DATA_*");

// Highlight net class
pcb.highlight_net_class("power");
```

### Visual Appearance

```
Normal view:
┌─────────────────────────────────────┐
│ ════════════════════════════════    │
│ ────────────────────────────────    │
│ ════════════════════════════════    │
│ ────────────────────────────────    │
└─────────────────────────────────────┘

Highlighted (VCC):
┌─────────────────────────────────────┐
│ ════════════════════════════════    │  ← Bright
│ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    │  ← Dimmed
│ ════════════════════════════════    │  ← Bright
│ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    │  ← Dimmed
└─────────────────────────────────────┘
```

## Ratsnest Display

### Ratsnest Configuration

```rust
RatsnestConfig {
    // Display options
    show_ratsnest: true,
    show_all_nets: false,
    show_selected_only: true,
    
    // Visual style
    line_style: LineStyle::Straight,  // or Curved
    color: Color::White,
    thickness: 0.1,
    
    // Filtering
    hide_short_rats: false,
    min_length: 1.0,  // mm
}
```

### Ratsnest Modes

| Mode | Description |
|------|-------------|
| **All** | Show all unrouted connections |
| **Selected** | Show only selected component/net |
| **Net** | Show specific net's connections |
| **None** | Hide ratsnest |

### Ratsnest Statistics

```
Ratsnest Statistics
═══════════════════

Total connections: 245
Routed: 198 (81%)
Unrouted: 47 (19%)

Longest unrouted: 45.2mm (NET_CLK)
Average length: 12.3mm
Crossings: 23

By net class:
  power: 5 unrouted
  signal: 35 unrouted
  high_speed: 7 unrouted
```

## Net Tracing

### Trace Path

```rust
// Get complete path of a net
let path = pcb.trace_net("SPI_CLK");

for segment in path.segments() {
    println!("Layer: {:?}, Length: {:.2}mm", 
             segment.layer, segment.length);
}
```

### Net Path Visualization

```
Net: SPI_CLK
Path: U1.PA5 → R1.1 → R1.2 → U2.SCK

┌─────┐         ┌───┐         ┌─────┐
│ U1  ├────●────┤R1 ├────●────┤ U2  │
│PA5  │  F.Cu   │   │  F.Cu   │ SCK │
└─────┘         └───┘         └─────┘

Total length: 25.4mm
Vias: 0
Layer changes: 0
```

### Cross-Probe

Click on schematic to highlight in PCB (and vice versa):

```rust
CrossProbe {
    enabled: true,
    
    // Sync options
    sync_selection: true,
    sync_zoom: true,
    center_on_item: true,
}
```

## Net Properties

### Net Information Panel

```
Net: VCC
══════════════════════════════════════

Properties:
  Class: power
  Voltage: 3.3V
  
Connections: 45
  Pads: 38
  Vias: 7
  
Routing:
  Total length: 234.5mm
  Layers used: F.Cu, In2.Cu, B.Cu
  
Statistics:
  Min width: 0.3mm
  Max width: 0.5mm
  Via count: 7
  
Connected components:
  U1 (VDD), U2 (VCC), U3 (VDD)
  C1 (1), C2 (1), C3 (1), ...
```

### Net List

```rust
// Get all nets
let nets = pcb.nets();

// Filter nets
let power_nets = pcb.nets_by_class("power");
let unrouted = pcb.nets_with_unrouted();

// Sort by various criteria
nets.sort_by_length();
nets.sort_by_connection_count();
nets.sort_by_name();
```

## Net Analysis

### Length Analysis

```rust
let analysis = pcb.analyze_net("DDR_CLK");

println!("Total length: {:.2}mm", analysis.total_length);
println!("Routed: {:.2}mm", analysis.routed_length);
println!("Via stubs: {:.2}mm", analysis.via_stub_length);
```

### Impedance Analysis

```rust
let impedance = pcb.analyze_impedance("USB_D+");

println!("Target: {}Ω", impedance.target);
println!("Calculated: {}Ω", impedance.calculated);
println!("Variation: ±{}%", impedance.variation);
```

### Crosstalk Analysis

```rust
let crosstalk = pcb.analyze_crosstalk(&["DATA_0", "DATA_1"]);

println!("Parallel length: {:.2}mm", crosstalk.parallel_length);
println!("Min spacing: {:.2}mm", crosstalk.min_spacing);
println!("Coupling: {:.1}%", crosstalk.coupling_percent);
```

## Net Comparison

### Differential Pair Check

```rust
let diff_check = pcb.check_differential_pair("USB_D+", "USB_D-");

println!("Length D+: {:.2}mm", diff_check.length_positive);
println!("Length D-: {:.2}mm", diff_check.length_negative);
println!("Skew: {:.3}mm", diff_check.skew);
println!("Status: {:?}", diff_check.status);  // Pass/Fail
```

### Length Matching Report

```
Length Matching: DDR_DQ Group
═════════════════════════════

Reference: DQ_CLK (45.0mm)
Tolerance: ±0.5mm

Net      Length    Delta    Status
────────────────────────────────────
DQ0      45.2mm    +0.2mm   ✓ Pass
DQ1      44.8mm    -0.2mm   ✓ Pass
DQ2      45.5mm    +0.5mm   ✓ Pass
DQ3      46.1mm    +1.1mm   ✗ Fail
DQ4      44.9mm    -0.1mm   ✓ Pass
DQ5      45.0mm     0.0mm   ✓ Pass
DQ6      44.5mm    -0.5mm   ✓ Pass
DQ7      45.3mm    +0.3mm   ✓ Pass
```

## Interactive Features

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `H` | Highlight net under cursor |
| `Ctrl+H` | Clear highlight |
| `\`` | Toggle ratsnest |
| `Ctrl+Click` | Add to selection |
| `I` | Inspect net properties |

### Context Menu

```
Right-click on net:
├── Highlight Net
├── Highlight Net Class
├── Select Entire Net
├── Route Net
├── Inspect Net
│   ├── Properties
│   ├── Length Analysis
│   └── Connections
├── Find in Schematic
└── Copy Net Name
```

## Net Topology Preview

Visualize preferred routing topologies before routing to optimize signal integrity and timing.

### Topology Types

| Topology | Description | Use Case |
|----------|-------------|----------|
| **Star** | All branches from single point | Clock distribution, power |
| **Daisy-Chain** | Sequential point-to-point | I2C, SPI chip select |
| **T-Branch** | Split at midpoint | Differential termination |
| **Fly-By** | Sequential with stubs | DDR memory |
| **H-Tree** | Balanced binary tree | High-speed clocks |

### Topology Visualization

```
Star Topology:           Daisy-Chain:
                         
      ┌── B               A ─── B ─── C ─── D
      │                   
  A ──┼── C               
      │                   
      └── D               

T-Branch:                Fly-By (DDR):

      ┌── B               CLK ─┬─ U1 ─┬─ U2 ─┬─ U3
      │                        │      │      │
  A ──┤                       ─┴─    ─┴─    ─┴─
      │                   
      └── C               

H-Tree:

          ┌─ D
      ┌─┬─┤
      │ │ └─ E
  A ──┤
      │ │ ┌─ F
      └─┴─┤
          └─ G
```

### Topology Preview UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Net Topology Preview                                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Net: CLK_100MHz                                                 │
│ Connections: U1.CLK, U2.CLK, U3.CLK, U4.CLK                    │
│                                                                 │
│ Recommended Topology: [Star        ▼]                          │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │           U2.CLK                                            │ │
│ │              ↑                                              │ │
│ │              │                                              │ │
│ │  U1.CLK ←── OSC ──→ U3.CLK                                 │ │
│ │              │                                              │ │
│ │              ↓                                              │ │
│ │           U4.CLK                                            │ │
│ │                                                             │ │
│ │  Legend: ─── Suggested route path                           │ │
│ │          ● Source point                                     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Estimated lengths:                                              │
│   U1.CLK: 25.3mm    U2.CLK: 24.8mm                             │
│   U3.CLK: 25.1mm    U4.CLK: 25.0mm                             │
│   Max skew: 0.5mm ✓                                            │
│                                                                 │
│ [Apply Topology] [Route with Topology] [Cancel]                 │
└─────────────────────────────────────────────────────────────────┘
```

### Topology Configuration

```rust
NetTopology {
    net: "CLK_100MHz",
    
    // Topology type
    topology: TopologyType::Star,
    
    // Source pin (for star/tree)
    source: PinRef::new("U1", "CLK_OUT"),
    
    // Target pins
    targets: vec![
        PinRef::new("U2", "CLK"),
        PinRef::new("U3", "CLK"),
        PinRef::new("U4", "CLK"),
    ],
    
    // Constraints
    constraints: TopologyConstraints {
        max_skew: 0.5,           // mm
        match_lengths: true,
        min_stub_length: 0.0,    // For fly-by
        max_stub_length: 2.0,
    },
}
```

### Automatic Topology Detection

```rust
// Analyze net and suggest topology
let suggestion = pcb.suggest_topology("DDR_DQ0")?;

println!("Suggested: {:?}", suggestion.topology);
println!("Reason: {}", suggestion.reason);
println!("Source: {}", suggestion.source);

// Apply suggestion
pcb.apply_topology("DDR_DQ0", suggestion)?;
```

### Topology-Aware Routing

```rust
// Route net following topology
pcb.route_with_topology("CLK_100MHz", RoutingOptions {
    follow_topology: true,
    topology: TopologyType::Star,
    
    // Length matching
    match_lengths: true,
    length_tolerance: 0.5,
    
    // Routing style
    use_teardrops: true,
    smooth_corners: true,
});
```

### Topology Templates

```rust
// DDR3 fly-by topology template
TopologyTemplate::ddr3_flyby(DdrConfig {
    controller: "U1",
    memory: vec!["U2", "U3"],
    
    // Signal groups
    address_cmd: true,
    clock: true,
    data: false,  // Data uses point-to-point
});

// Clock tree template
TopologyTemplate::clock_tree(ClockTreeConfig {
    source: ("Y1", "OUT"),
    fanout: 8,
    balance: BalanceType::HTree,
});
```

## Route by Schematic Sheet

Filter and highlight nets from the current schematic sheet for focused routing.

### Sheet-Based Routing Mode

```
┌─────────────────────────────────────────────────────────────────┐
│ Route by Sheet                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Current Sheet: power_supply.hwt_sch                            │
│                                                                 │
│ Nets on this sheet: 24                                          │
│   Routed: 18 (75%)                                              │
│   Unrouted: 6                                                   │
│                                                                 │
│ Unrouted nets:                                                  │
│   [☑] VIN           [☑] VOUT_3V3                               │
│   [☑] VOUT_5V       [☑] FB                                     │
│   [☑] SW            [☑] PGOOD                                  │
│                                                                 │
│ Display:                                                        │
│   [●] Show only this sheet's nets                              │
│   [○] Highlight this sheet's nets                              │
│   [○] Show all nets                                            │
│                                                                 │
│ [Start Routing] [Select All Unrouted] [Cancel]                  │
└─────────────────────────────────────────────────────────────────┘
```

### Sheet Filter API

```rust
// Filter to current sheet
pcb.set_sheet_filter("power_supply.hwt_sch");

// Get nets from sheet
let sheet_nets = pcb.nets_from_sheet("power_supply.hwt_sch")?;

// Highlight sheet nets
pcb.highlight_sheet_nets("power_supply.hwt_sch");

// Route only sheet nets
pcb.route_sheet("power_supply.hwt_sch", RouteOptions::default())?;
```

### Cross-Reference Navigation

```rust
// Jump from schematic sheet to PCB with filter
schematic.navigate_to_pcb_filtered("io_interface.hwt_sch");

// PCB now shows only nets from io_interface sheet
// Other nets dimmed or hidden
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+S` | Open sheet filter dialog |
| `Alt+S` | Toggle sheet filter on/off |
| `Ctrl+[` | Previous sheet |
| `Ctrl+]` | Next sheet |

## Related Topics

- [Interactive Routing](./interactive-routing.md)
- [Design Rule Check](./drc.md)
- [Wiring & Connectivity](../schematic-editor/wiring-connectivity.md)
