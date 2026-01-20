# Native AI Design Assistant

## Overview

Hardware Tool's AI assistant is **natively integrated** — not a bolted-on feature, but a core part of the design experience. The AI has direct access to all tooling, understands electronics first principles, and can manipulate designs with the precision of an expert engineer.

## Core Philosophy

### First-Principles Thinking

The AI doesn't just pattern-match — it **thinks** like an expert engineer:

```
Traditional Auto-router:          Hardware Tool AI:
                                  
┌─────────────────────┐          ┌─────────────────────┐
│ Pattern matching    │          │ First principles    │
│ Heuristic rules     │          │ ─────────────────── │
│ Trial and error     │          │ • Ohm's law         │
│                     │          │ • Maxwell's eqns    │
│                     │          │ • Thermal dynamics  │
│                     │          │ • Signal integrity  │
└─────────────────────┘          └─────────────────────┘
         │                                │
         ▼                                ▼
   "Good enough"                  Optimal solution
   routing                        with understanding
```

### Native Tool Access

The AI has direct access to all Hardware Tool capabilities:

```rust
AiToolAccess {
    // Read capabilities
    read_schematic: true,
    read_pcb: true,
    read_netlist: true,
    read_design_rules: true,
    read_component_data: true,
    
    // Write capabilities
    place_components: true,
    route_traces: true,
    modify_zones: true,
    adjust_stackup: true,
    
    // Analysis capabilities
    run_drc: true,
    run_erc: true,
    run_dfm: true,
    run_simulation: true,
    
    // No hacky workarounds needed
    uses_mcp_server: false,
    uses_screen_scraping: false,
    uses_macro_injection: false,
}
```

## AI Capabilities

### Natural Language Commands

Speak to your design tool naturally:

```
User: "Route the differential pair from U1 to J1 with 90 ohm impedance"

AI: Analyzing design...
    - Source: U1 pins 45-46 (USB_D+/USB_D-)
    - Target: J1 pins 2-3
    - Calculating trace geometry for 90Ω on current stackup...
    - Trace width: 0.12mm, gap: 0.18mm
    - Routing with length matching (±0.1mm)...
    
    ✅ Differential pair routed
    - Length: 45.2mm (matched within 0.05mm)
    - Impedance: 90.2Ω (simulated)
    - Via count: 2 (layer transition)
```

### Design Review

```
User: "Review my power delivery network"

AI: Analyzing PDN...

    ⚠️ Issues Found:
    
    1. CRITICAL: Insufficient decoupling for U1 (STM32F4)
       - Current: 2x 100nF
       - Recommended: 100nF + 10nF + 1µF per VDD pin
       - Resonance gap at 50-80MHz
    
    2. WARNING: Long power trace to U3
       - Length: 45mm with 0.3mm width
       - Voltage drop: ~50mV at 500mA
       - Recommendation: Widen to 0.5mm or add via to plane
    
    3. INFO: Consider adding bulk capacitor
       - 10µF near power input would improve transient response
    
    [Apply Suggestions] [Explain More] [Ignore]
```

### Component Suggestions

```
User: "I need a 3.3V regulator that can handle 500mA with good thermal performance"

AI: Based on your requirements and board constraints:

    Recommended: TLV1117-33 (SOT-223)
    - Output: 3.3V @ 800mA max
    - Dropout: 1.1V typical
    - Thermal: SOT-223 provides 50°C/W to PCB
    - LCSC: C6186 ($0.12)
    
    Alternatives:
    1. AP2112K-3.3 (SOT-23-5) - Smaller, 600mA
    2. AMS1117-3.3 (SOT-223) - Cheaper, higher dropout
    3. TPS73633 (SOT-23-5) - Ultra-low noise, premium
    
    [Add to Schematic] [Compare Datasheets] [Show More]
```

## Interaction Modes

### Chat Interface

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Assistant                                            [─][□][×]│
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ 🤖 How can I help with your design?                            │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ You: Place decoupling caps for U1                           │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 🤖 I'll add decoupling capacitors for U1 (STM32F405RGT6).  │ │
│ │                                                             │ │
│ │ Based on the datasheet recommendations:                     │ │
│ │ • 4x 100nF (one per VDD pin)                               │ │
│ │ • 1x 4.7µF bulk capacitor                                  │ │
│ │ • 1x 10nF for VDDA (analog supply)                         │ │
│ │                                                             │ │
│ │ Placing within 3mm of respective pins...                    │ │
│ │                                                             │ │
│ │ ✅ Done! 6 capacitors placed.                               │ │
│ │                                                             │ │
│ │ [Undo] [Adjust Placement] [Add to Template]                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Type a message...                              [Send] [🎤]  │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Suggestions: [Route power] [Check DRC] [Optimize placement]     │
└─────────────────────────────────────────────────────────────────┘
```

### Inline Suggestions

```rust
InlineSuggestions {
    enabled: true,
    
    // When to show
    triggers: vec![
        Trigger::AfterPlacement,
        Trigger::AfterRouting,
        Trigger::OnDrcError,
        Trigger::OnIdle { seconds: 5 },
    ],
    
    // How to show
    display: SuggestionDisplay {
        style: DisplayStyle::Subtle,  // Non-intrusive
        position: Position::NearCursor,
        auto_dismiss: Duration::seconds(10),
    },
}
```

### Command Palette Integration

```
Ctrl+K → "optimize thermal vias for U1"

┌─────────────────────────────────────────────────────────────────┐
│ 🔍 optimize thermal vias for U1                                 │
├─────────────────────────────────────────────────────────────────┤
│ 🤖 AI: Add thermal vias under U1                               │
│    Estimated improvement: -15°C junction temperature            │
│                                                                 │
│ 🤖 AI: Optimize existing via pattern                           │
│    Current: 4 vias, Suggested: 9 vias in grid pattern          │
│                                                                 │
│ 📖 Docs: Thermal Via Design Guide                              │
│ 🔧 Tool: Via Stitching Tool                                    │
└─────────────────────────────────────────────────────────────────┘
```

## AI-Assisted Workflows

### Schematic to PCB

```rust
AiSchematicToPcb {
    // Automatic steps
    auto_assign_footprints: true,
    suggest_alternatives: true,
    
    // Placement assistance
    group_by_function: true,
    optimize_for_routing: true,
    consider_thermal: true,
    
    // User control
    require_approval: true,
    explain_decisions: true,
}
```

### Design Optimization

```rust
AiOptimization {
    targets: vec![
        OptTarget::SignalIntegrity,
        OptTarget::PowerDelivery,
        OptTarget::Thermal,
        OptTarget::Manufacturability,
        OptTarget::Cost,
    ],
    
    // Constraints
    constraints: Constraints {
        max_layers: 4,
        max_board_size: (100.0, 80.0),
        target_cost: 5.0,  // per unit
    },
    
    // Approach
    method: OptMethod::FirstPrinciples,
    iterations: 100,
    show_progress: true,
}
```

## Learning & Adaptation

### Project Context

The AI learns your project's context:

```rust
ProjectContext {
    // Remembered across sessions
    component_preferences: HashMap<Category, Vec<Component>>,
    routing_style: RoutingStyle,
    design_rules_used: Vec<DesignRuleSet>,
    
    // Inferred from design
    application_type: ApplicationType::IoT,
    power_requirements: PowerProfile::LowPower,
    signal_types: vec![SignalType::I2C, SignalType::SPI, SignalType::USB],
}
```

### Feedback Loop

```rust
AiFeedback {
    // Learn from corrections
    track_undo_after_ai: true,
    track_manual_adjustments: true,
    
    // Explicit feedback
    thumbs_up_down: true,
    detailed_feedback: true,
    
    // Improvement
    adapt_to_user_style: true,
    remember_preferences: true,
}
```

## Privacy & Control

### User Control

```rust
AiControl {
    // Always ask before
    require_approval_for: vec![
        Action::ModifySchematic,
        Action::ModifyPcb,
        Action::DeleteAnything,
    ],
    
    // Auto-approve (optional)
    auto_approve: vec![
        Action::Suggestions,
        Action::Analysis,
        Action::DrcCheck,
    ],
    
    // Undo everything
    full_undo_support: true,
    checkpoint_before_ai: true,
}
```

## Related Topics

- [API Keys Configuration](./api-keys-configuration.md)
- [AI-Powered Routing & Optimization](./ai-routing-optimization.md)
- [Benchmarking Simulator](./benchmarking-simulator.md)
