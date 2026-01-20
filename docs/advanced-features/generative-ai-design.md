# Generative AI Design

## Overview

Hardware Tool features revolutionary generative AI capabilities that transform natural language specifications into complete, manufacturable PCB designs. The AI understands electrical engineering principles, component selection, layout constraints, and manufacturing requirements—enabling "describe and build" hardware design.

### The Vision: Hardware Copilot

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  User: "I need a 48V to 5V buck converter, 10A output,         │
│         90% efficiency, fits in 50×30mm, automotive temp"       │
│                                                                 │
│  AI: ✓ Analyzing requirements...                               │
│      ✓ Selecting topology (synchronous buck)                   │
│      ✓ Choosing components (TPS548B22 + inductors + caps)      │
│      ✓ Generating schematic                                    │
│      ✓ Creating PCB layout                                     │
│      ✓ Running thermal simulation                              │
│      ✓ Verifying EMC compliance                                │
│                                                                 │
│  [View Schematic] [View PCB] [BOM: $4.23] [Order Samples]      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Generative Design Modes

### 1. Specification-to-Design

Transform requirements into complete designs:

```rust
GenerativeDesign {
    mode: GenerativeMode::SpecToDesign,
    
    // Natural language specification
    specification: r#"
        Design a USB-C Power Delivery sink controller:
        - Input: USB-C PD up to 100W (20V/5A)
        - Outputs: 5V/3A, 12V/2A, adjustable 3.3-20V/2A
        - MCU: STM32G0 for PD negotiation
        - Protection: OVP, OCP, reverse polarity
        - Size: 40×25mm max
        - 4-layer PCB
    "#,
    
    // Constraints
    constraints: DesignConstraints {
        max_cost: 15.0,               // $15 BOM target
        max_layers: 4,
        temperature_range: (-20.0, 85.0),
        certifications: vec!["USB-IF", "CE", "FCC"],
    },
    
    // Output
    output: GenerativeOutput {
        schematic: true,
        pcb_layout: true,
        bom: true,
        simulation_results: true,
        documentation: true,
    },
}
```

### 2. Component-to-Application

Generate application circuits from component selection:

```rust
GenerativeDesign {
    mode: GenerativeMode::ComponentToApplication,
    
    // Selected component
    component: Component {
        mpn: "LM5146",
        manufacturer: "Texas Instruments",
        description: "Wide VIN synchronous buck controller",
    },
    
    // Application requirements
    application: ApplicationRequirements {
        vin_range: (18.0, 75.0),      // V
        vout: 12.0,                    // V
        iout_max: 10.0,                // A
        efficiency_target: 0.95,
        switching_frequency: 400e3,    // Hz
    },
    
    // Generate complete application circuit
    generate: vec![
        Generate::PowerStage,
        Generate::Feedback,
        Generate::SoftStart,
        Generate::Protection,
        Generate::InputFilter,
        Generate::OutputFilter,
    ],
}
```

### 3. Block Diagram to Schematic

Convert high-level block diagrams to detailed schematics:

```rust
GenerativeDesign {
    mode: GenerativeMode::BlockToSchematic,
    
    // Block diagram definition
    blocks: vec![
        Block {
            name: "Power Input",
            type_: BlockType::PowerInput,
            specs: json!({ "voltage": "12V", "current": "5A" }),
        },
        Block {
            name: "MCU",
            type_: BlockType::Microcontroller,
            specs: json!({ "family": "STM32", "peripherals": ["SPI", "I2C", "UART", "ADC"] }),
        },
        Block {
            name: "Sensors",
            type_: BlockType::SensorArray,
            specs: json!({ "types": ["temperature", "humidity", "pressure"] }),
        },
        Block {
            name: "Wireless",
            type_: BlockType::Wireless,
            specs: json!({ "protocol": "BLE 5.0" }),
        },
    ],
    
    // Connections
    connections: vec![
        Connection { from: "Power Input", to: "MCU", type_: "power" },
        Connection { from: "MCU", to: "Sensors", type_: "I2C" },
        Connection { from: "MCU", to: "Wireless", type_: "SPI" },
    ],
}
```

### 4. Reference Design Customization

Adapt reference designs to specific requirements:

```rust
GenerativeDesign {
    mode: GenerativeMode::CustomizeReference,
    
    // Base reference design
    reference: ReferenceDesign {
        source: "TI TIDA-010086",     // 48V motor driver
        import_from: "ti.com/tool/TIDA-010086",
    },
    
    // Customizations
    customizations: vec![
        Customization::ChangeVoltage { rail: "VIN", new_value: 36.0 },
        Customization::ChangeCurrent { output: "MOTOR", new_value: 20.0 },
        Customization::AddProtection { type_: "thermal_shutdown" },
        Customization::ResizePCB { width: 60.0, height: 40.0 },
        Customization::ChangePackages { prefer: "automotive_grade" },
    ],
}
```

## AI Architecture

### Multi-Model Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                    Generative AI Pipeline                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   Natural   │    │  Reasoning  │    │  Component  │         │
│  │  Language   │───▶│   Engine    │───▶│  Selection  │         │
│  │   Parser    │    │   (LLM)     │    │    Agent    │         │
│  └─────────────┘    └─────────────┘    └──────┬──────┘         │
│                                               │                 │
│                                               ▼                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   Layout    │◀───│  Schematic  │◀───│  Topology   │         │
│  │  Generator  │    │  Generator  │    │   Planner   │         │
│  └──────┬──────┘    └─────────────┘    └─────────────┘         │
│         │                                                       │
│         ▼                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │ Verification│───▶│ Optimization│───▶│   Output    │         │
│  │   Suite     │    │    Loop     │    │  Generator  │         │
│  └─────────────┘    └─────────────┘    └─────────────┘         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### AI Model Configuration

```rust
AIModelConfig {
    // Primary reasoning model
    reasoning: ModelConfig {
        provider: Provider::Anthropic,
        model: "claude-sonnet-4-20250514",
        temperature: 0.3,             // Lower for determinism
        max_tokens: 16000,
    },
    
    // Component selection model
    component_selection: ModelConfig {
        provider: Provider::Local,
        model: "hardware-tool-components-v2",  // Fine-tuned
        context_window: 32000,
    },
    
    // Layout optimization model
    layout: ModelConfig {
        provider: Provider::Local,
        model: "hardware-tool-layout-v2",      // Fine-tuned
        use_gpu: true,
    },
    
    // Verification model
    verification: ModelConfig {
        provider: Provider::Anthropic,
        model: "claude-sonnet-4-20250514",
        temperature: 0.1,             // Very deterministic
    },
}
```

## Component Selection AI

### Intelligent Part Selection

```rust
ComponentSelectionAI {
    // Selection criteria
    criteria: SelectionCriteria {
        // Electrical requirements
        electrical: ElectricalCriteria {
            voltage_margin: 1.5,      // 50% derating
            current_margin: 1.3,      // 30% derating
            power_margin: 1.5,
        },
        
        // Availability
        availability: AvailabilityCriteria {
            min_stock: 1000,
            max_lead_time_weeks: 12,
            preferred_distributors: vec!["DigiKey", "Mouser", "LCSC"],
        },
        
        // Cost optimization
        cost: CostCriteria {
            optimize_for: CostTarget::TotalBOM,
            quantity_break: 1000,
        },
        
        // Quality
        quality: QualityCriteria {
            automotive_grade: false,
            rohs_compliant: true,
            aec_q100: false,
        },
    },
    
    // Learning from history
    learning: LearningConfig {
        use_project_history: true,
        use_community_data: true,
        prefer_proven_parts: true,
    },
}
```

### Component Recommendation UI

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Component Recommendation                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Requirement: 100µF, 25V, X5R, 0805, low ESR                    │
│                                                                 │
│ Top Recommendations:                                            │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 1. Murata GRM21BR61E106KA73L                    ⭐ Best     │ │
│ │    100µF, 25V, X5R, 0805                                    │ │
│ │    ESR: 3mΩ | Stock: 45,000 | Price: $0.12 @ 1k            │ │
│ │    ✓ Used in 23 similar projects                           │ │
│ │    ✓ Automotive grade available                            │ │
│ │    [Select] [Datasheet] [Compare]                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 2. Samsung CL21A106KPFNNNE                      💰 Budget   │ │
│ │    100µF, 25V, X5R, 0805                                    │ │
│ │    ESR: 5mΩ | Stock: 120,000 | Price: $0.08 @ 1k           │ │
│ │    ✓ Used in 15 similar projects                           │ │
│ │    [Select] [Datasheet] [Compare]                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 3. TDK C2012X5R1E107M125AC                      📦 In Stock │ │
│ │    100µF, 25V, X5R, 0805                                    │ │
│ │    ESR: 4mΩ | Stock: 200,000 | Price: $0.10 @ 1k           │ │
│ │    [Select] [Datasheet] [Compare]                          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ AI Reasoning:                                                   │
│ "Selected Murata as top choice due to lowest ESR (3mΩ),        │
│  proven reliability in similar buck converter applications,    │
│  and availability of automotive-grade variant for future       │
│  product line extension."                                       │
│                                                                 │
│ [Accept Top] [Show More] [Custom Search] [Close]                │
└─────────────────────────────────────────────────────────────────┘
```

## Schematic Generation

### AI Schematic Synthesis

```rust
SchematicGenerator {
    // Generation strategy
    strategy: GenerationStrategy {
        // Start from topology
        topology_first: true,
        
        // Use proven subcircuits
        use_subcircuit_library: true,
        
        // Optimization passes
        optimization: vec![
            Optimization::MinimizeComponents,
            Optimization::MaximizeReliability,
            Optimization::OptimizeCost,
        ],
    },
    
    // Subcircuit library
    subcircuits: SubcircuitLibrary {
        sources: vec![
            SubcircuitSource::BuiltIn,
            SubcircuitSource::Manufacturer("TI"),
            SubcircuitSource::Manufacturer("Analog Devices"),
            SubcircuitSource::Community,
        ],
    },
    
    // Validation
    validation: ValidationConfig {
        run_erc: true,
        run_simulation: true,
        check_component_ratings: true,
    },
}
```

### Generated Schematic Example

```
┌─────────────────────────────────────────────────────────────────┐
│ AI-Generated Schematic: 48V→5V Buck Converter                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                    ┌─────────────────────────────────┐          │
│                    │         TPS548B22               │          │
│     VIN ──┬───────┤VIN                         SW├──┬──L1──┬── VOUT
│     48V   │       │                               │  │ 4.7µH│   5V
│           │       │                               │  │      │   10A
│          ─┴─      │                          BST├──┤       │
│          C1       │                               │ C_BST   │
│         100µF     │                               │         │
│           │       │                          VCC├──┤        │
│           │       │                               │ C_VCC  ─┴─
│           │       │FB                             │        C_OUT
│           │       ├───┬── R1 ──┬── R2 ──┐        │        100µF
│           │       │   │  10k   │  2k    │        │         │
│           │       │   │        │       ─┴─       │         │
│           │       │  ─┴─      ─┴─      GND       │         │
│           │       │  C_FB     GND                │         │
│           │       │  100pF                       │         │
│          ─┴─      │                             ─┴─       ─┴─
│          GND      │GND                          GND       GND
│                   └─────────────────────────────────┘          │
│                                                                 │
│ AI Notes:                                                       │
│ • TPS548B22 selected for 48V input, integrated FETs, 10A       │
│ • Feedback divider: R1=10k, R2=2k → Vout = 0.6V × (1+10k/2k)  │
│ • L1 = 4.7µH for 400kHz, 30% ripple current                   │
│ • Output caps: 2×100µF for <50mV ripple                        │
│                                                                 │
│ [Edit] [Simulate] [Generate PCB] [Export] [Regenerate]          │
└─────────────────────────────────────────────────────────────────┘
```

## Layout Generation

### AI-Powered PCB Layout

```rust
LayoutGenerator {
    // Placement strategy
    placement: PlacementStrategy {
        // Thermal-aware placement
        thermal_aware: true,
        
        // Signal integrity aware
        si_aware: true,
        
        // Grouping
        group_by_function: true,
        
        // Optimization
        optimization: PlacementOptimization {
            minimize_trace_length: true,
            minimize_vias: true,
            maximize_copper_pour: true,
        },
    },
    
    // Routing strategy
    routing: RoutingStrategy {
        // High-speed nets first
        priority_nets: vec!["CLK", "DATA", "DIFF_PAIR"],
        
        // Power routing
        power_routing: PowerRouting::PolygonPour,
        
        // Via strategy
        via_strategy: ViaStrategy::MinimizeCount,
    },
    
    // Constraints
    constraints: LayoutConstraints {
        board_outline: BoardOutline::Rectangle { width: 50.0, height: 30.0 },
        layer_count: 4,
        min_trace: 0.15,              // mm
        min_space: 0.15,              // mm
        min_via: 0.3,                 // mm
    },
}
```

### Layout Generation Process

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Layout Generation                                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Progress:                                                       │
│                                                                 │
│ ✓ Analyzing netlist (45 nets, 23 components)                   │
│ ✓ Identifying critical paths (3 high-speed, 2 power)           │
│ ✓ Generating placement candidates (1,000 evaluated)            │
│ ✓ Optimizing placement (thermal + SI score: 94/100)            │
│ ● Routing power nets...                              [████░░] 67%
│ ○ Routing signal nets                                           │
│ ○ Copper pour generation                                        │
│ ○ DRC verification                                              │
│ ○ SI/PI verification                                            │
│                                                                 │
│ Current Layout Preview:                                         │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │  ┌───┐                                           ┌───┐      │ │
│ │  │C1 │    ┌─────────────────┐                   │C5 │      │ │
│ │  └───┘    │                 │    ┌───┐          └───┘      │ │
│ │           │      U1         │    │L1 │                      │ │
│ │  ┌───┐    │   TPS548B22     │    └───┘          ┌───┐      │ │
│ │  │C2 │    │                 │                   │C6 │      │ │
│ │  └───┘    └─────────────────┘    ┌───┐          └───┘      │ │
│ │                                  │C3 │                      │ │
│ │  ┌───┐                           └───┘          ┌───┐      │ │
│ │  │R1 │    ┌───┐                                 │C7 │      │ │
│ │  └───┘    │R2 │                                 └───┘      │ │
│ │           └───┘                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Pause] [Cancel] [View 3D] [Settings]                           │
└─────────────────────────────────────────────────────────────────┘
```

## Design Optimization Loop

### Autonomous Optimization

```rust
DesignOptimization {
    // Optimization goals (Pareto multi-objective)
    goals: OptimizationGoals {
        minimize: vec![
            Goal::Cost { weight: 0.3 },
            Goal::Area { weight: 0.2 },
            Goal::EMI { weight: 0.2 },
        ],
        maximize: vec![
            Goal::Efficiency { weight: 0.2 },
            Goal::ThermalMargin { weight: 0.1 },
        ],
    },
    
    // Constraints (hard limits)
    constraints: vec![
        Constraint::MaxCost(10.0),
        Constraint::MaxArea(1500.0),    // mm²
        Constraint::MinEfficiency(0.90),
        Constraint::MaxTemperature(85.0),
        Constraint::PassEMC(Standard::CISPR32_ClassB),
    ],
    
    // Optimization algorithm
    algorithm: OptimizationAlgorithm::NSGA_III {
        population: 200,
        generations: 100,
        crossover_rate: 0.9,
        mutation_rate: 0.1,
    },
    
    // Design variables
    variables: vec![
        Variable::ComponentSelection { alternatives: true },
        Variable::SwitchingFrequency { range: (200e3, 1e6) },
        Variable::InductorValue { range: (1e-6, 22e-6) },
        Variable::CapacitorCount { range: (1, 6) },
        Variable::PCBLayers { options: vec![2, 4, 6] },
        Variable::CopperWeight { options: vec![1.0, 2.0] },
    ],
}
```

### Pareto Front Visualization

```
┌─────────────────────────────────────────────────────────────────┐
│ Design Optimization Results                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Pareto Front (Cost vs Efficiency):                              │
│                                                                 │
│ Efficiency                                                      │
│    96%├─────────────────────────────────────────────────────── │
│       │                                              ●          │
│    94%├─────────────────────────────────────────●────────────  │
│       │                                    ●                    │
│    92%├───────────────────────────────●────────────────────── │
│       │                          ●                              │
│    90%├─────────────────────●──────────────────────────────── │
│       │                ●                                        │
│    88%├────────────●───────────────────────────────────────── │
│       │       ●                                                 │
│    86%├───●────────────────────────────────────────────────── │
│       └──┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────  │
│         $3   $4   $5   $6   $7   $8   $9   $10  $11  $12      │
│                           Cost                                  │
│                                                                 │
│ Selected Designs:                                               │
│                                                                 │
│ ● Design A: $4.50, 91% eff, 45°C rise (Best value)             │
│ ● Design B: $6.20, 94% eff, 35°C rise (Balanced)               │
│ ● Design C: $9.80, 96% eff, 25°C rise (Premium)                │
│                                                                 │
│ [Compare A/B/C] [Select Design] [Export All] [Close]            │
└─────────────────────────────────────────────────────────────────┘
```

## Natural Language Interface

### Conversational Design

```rust
ConversationalDesign {
    // Enable natural language
    enabled: true,
    
    // Context retention
    context: ContextConfig {
        retain_history: true,
        max_turns: 50,
        remember_preferences: true,
    },
    
    // Capabilities
    capabilities: vec![
        Capability::DesignFromSpec,
        Capability::ModifyDesign,
        Capability::ExplainDecisions,
        Capability::SuggestImprovements,
        Capability::AnswerQuestions,
        Capability::DebugIssues,
    ],
}
```

### Conversation Example

```
┌─────────────────────────────────────────────────────────────────┐
│ Hardware Copilot                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ You: Design a battery charger for 4S LiPo, 5A max              │
│                                                                 │
│ AI: I'll design a 4S LiPo charger (16.8V max, 5A). Here's my   │
│     approach:                                                   │
│                                                                 │
│     Topology: Synchronous buck with CC/CV control               │
│     IC: BQ25713 (TI) - integrated charger with power path      │
│     Input: 24V DC (allows headroom for 16.8V output)           │
│                                                                 │
│     Key specs:                                                  │
│     • Charge current: 5A (CC mode)                             │
│     • Charge voltage: 16.8V ±0.5% (CV mode)                    │
│     • Cell balancing: External balancer recommended            │
│     • Efficiency: ~93% at full load                            │
│                                                                 │
│     [View Schematic] [View BOM: $8.45]                         │
│                                                                 │
│ You: Can you add cell balancing?                               │
│                                                                 │
│ AI: Adding active cell balancing using BQ76940 (4-cell AFE     │
│     with integrated balancing FETs). This adds:                │
│                                                                 │
│     • Per-cell voltage monitoring (±2mV accuracy)              │
│     • 50mA balance current per cell                            │
│     • Over/under voltage protection                            │
│     • Temperature monitoring (2 thermistors)                   │
│                                                                 │
│     Updated BOM: $12.30 (+$3.85)                               │
│                                                                 │
│     [View Updated Schematic] [Compare Versions]                │
│                                                                 │
│ You: What's the thermal situation at 5A?                       │
│                                                                 │
│ AI: Running thermal simulation...                              │
│                                                                 │
│     At 5A continuous (24V in, 16.8V out):                      │
│     • BQ25713: 72°C (Tj max: 125°C) ✓                         │
│     • Inductor: 65°C (rated 125°C) ✓                          │
│     • Input FETs: 58°C ✓                                       │
│                                                                 │
│     Board runs safely without heatsink in still air.           │
│     Recommend 2oz copper on power layers for margin.           │
│                                                                 │
│     [View Thermal Map] [Add Heatsink] [Increase Copper]        │
│                                                                 │
│ ─────────────────────────────────────────────────────────────── │
│ Type your message...                                    [Send]  │
└─────────────────────────────────────────────────────────────────┘
```

## Design Verification

### Automated Verification Suite

```rust
VerificationSuite {
    // Electrical verification
    electrical: ElectricalVerification {
        erc: true,
        simulation: SimulationVerification {
            dc_operating_point: true,
            transient: true,
            ac_analysis: true,
            monte_carlo: true,
        },
    },
    
    // Physical verification
    physical: PhysicalVerification {
        drc: true,
        dfm: true,
        thermal: true,
        mechanical: true,
    },
    
    // Signal/Power integrity
    integrity: IntegrityVerification {
        signal_integrity: true,
        power_integrity: true,
        emc_pre_compliance: true,
    },
    
    // Requirements traceability
    requirements: RequirementsVerification {
        trace_to_spec: true,
        coverage_report: true,
    },
}
```

### Verification Report

```
┌─────────────────────────────────────────────────────────────────┐
│ AI Design Verification Report                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Design: 48V Buck Converter (AI-generated)                       │
│ Generated: 2026-01-19 17:45:00                                  │
│                                                                 │
│ REQUIREMENTS TRACEABILITY                                       │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Requirement                    │ Status │ Verified By           │
│ ───────────────────────────────┼────────┼────────────────────── │
│ Vin: 48V nominal               │ ✓ Pass │ Component ratings     │
│ Vout: 5V ±2%                   │ ✓ Pass │ Simulation            │
│ Iout: 10A max                  │ ✓ Pass │ Thermal + simulation  │
│ Efficiency: >90%               │ ✓ Pass │ Simulation (93.2%)    │
│ Size: 50×30mm                  │ ✓ Pass │ Layout (48×28mm)      │
│ Temp: -20°C to +85°C           │ ✓ Pass │ Component ratings     │
│                                                                 │
│ VERIFICATION RESULTS                                            │
│ ═══════════════════════════════════════════════════════════════ │
│                                                                 │
│ Check                          │ Result │ Details               │
│ ───────────────────────────────┼────────┼────────────────────── │
│ ERC (Electrical Rules)         │ ✓ Pass │ 0 errors, 0 warnings  │
│ DRC (Design Rules)             │ ✓ Pass │ 0 violations          │
│ DFM (Manufacturability)        │ ✓ Pass │ Score: 98/100         │
│ Thermal Simulation             │ ✓ Pass │ Max 72°C (limit 85°C) │
│ Signal Integrity               │ ✓ Pass │ All nets within spec  │
│ Power Integrity                │ ✓ Pass │ Ripple <50mV          │
│ EMC Pre-compliance             │ ✓ Pass │ 6dB margin            │
│                                                                 │
│ CONFIDENCE SCORE: 94%                                           │
│                                                                 │
│ AI Reasoning:                                                   │
│ "Design meets all requirements with comfortable margins.        │
│  TPS548B22 is a proven solution for this application with      │
│  extensive field history. Layout follows TI reference design   │
│  guidelines. Recommend prototype validation before production." │
│                                                                 │
│ [Export Report] [Order Prototype] [Modify Design] [Close]       │
└─────────────────────────────────────────────────────────────────┘
```

## API Usage

```rust
// Generate design from natural language
let design = ai.generate_design(
    "USB-C PD charger, 65W output, GaN topology, compact"
)?;

// Review and modify
design.show_schematic()?;
design.show_pcb()?;

// Ask questions
let response = ai.ask("Why did you choose this inductor value?")?;
println!("{}", response);

// Optimize for different goals
let optimized = ai.optimize(design, OptimizationGoals {
    minimize: vec![Goal::Cost],
    constraints: vec![Constraint::MinEfficiency(0.92)],
})?;

// Compare designs
ai.compare(vec![design, optimized])?;

// Generate manufacturing files
design.export_gerbers("./output/")?;
design.export_bom("./output/bom.csv")?;

// Order prototype
design.order_prototype(Manufacturer::JLCPCB, Quantity::Qty5)?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+G` | Open generative design dialog |
| `Ctrl+Shift+G` | Generate from clipboard spec |
| `Alt+G` | AI chat panel |
| `Ctrl+Alt+G` | Optimize current design |

## Related Topics

- [AI-Powered Routing](../ai-integration/ai-routing-optimization.md) - AI routing optimization
- [Native AI Assistant](../ai-integration/native-ai-assistant.md) - AI assistant features
- [Programmatic Design](../core-architecture/programmatic-design.md) - Code-first design
- [Calculator Tools](./calculator-tools.md) - Design calculators
- [Variant Manager](./variant-manager.md) - Design variants
