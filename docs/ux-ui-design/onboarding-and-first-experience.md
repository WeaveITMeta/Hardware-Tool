# Onboarding & First Experience

## Overview

Hardware Tool's onboarding is designed to get users productive quickly while showcasing the tool's innovative capabilities. The experience adapts based on user expertise level and prior EDA experience.

## First Launch Experience

### Welcome Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│                    ⚡ Welcome to Hardware Tool                  │
│                                                                 │
│         The next-generation open-source EDA experience          │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │   Tell us about yourself:                               │   │
│  │                                                         │   │
│  │   ○ New to PCB design                                   │   │
│  │   ○ Hobbyist / Maker                                    │   │
│  │   ○ Professional engineer                               │   │
│  │   ○ Coming from KiCAD                                   │   │
│  │   ○ Coming from Altium / Eagle / Other                  │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│                        [Get Started →]                          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Adaptive Onboarding

```rust
OnboardingConfig {
    // User profile
    profile: UserProfile {
        experience_level: ExperienceLevel::Intermediate,
        previous_tools: vec!["KiCAD"],
        interests: vec!["IoT", "RF"],
    },
    
    // Adapted content
    show_basics: false,           // Skip for experienced users
    highlight_differences: true,  // Show KiCAD → HWT differences
    suggest_shortcuts: true,      // Offer KiCAD-compatible shortcuts
}
```

## Interactive Tutorial

### Guided First Project

```
Step 1 of 8: Create Your First Schematic
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Let's place your first component!

1. Press [A] or click the component icon
2. Search for "LED"
3. Click to place it on the canvas

┌─────────────────────────────────────────┐
│                                         │
│         Try it now! ──────────►  💡     │
│                                         │
│                                         │
└─────────────────────────────────────────┘

[← Back]  [Skip Tutorial]  [Next →]
```

### Tutorial Topics

| Step | Topic | Duration |
|------|-------|----------|
| 1 | Place components | 2 min |
| 2 | Wire connections | 2 min |
| 3 | Add power symbols | 1 min |
| 4 | Run ERC | 1 min |
| 5 | Create PCB | 2 min |
| 6 | Place & route | 3 min |
| 7 | Run DRC | 1 min |
| 8 | Export Gerbers | 2 min |

### Progress Tracking

```rust
TutorialProgress {
    completed_steps: vec![1, 2, 3],
    current_step: 4,
    total_steps: 8,
    
    // Achievements
    achievements: vec![
        Achievement::FirstComponent,
        Achievement::FirstConnection,
    ],
    
    // Can resume later
    save_progress: true,
}
```

## Feature Discovery

### Contextual Tips

```rust
ContextualTips {
    enabled: true,
    
    // Show tips based on context
    triggers: vec![
        Trigger::FirstTimeFeature,
        Trigger::IdleInArea { seconds: 5 },
        Trigger::RepeatedAction { count: 3 },
    ],
    
    // Tip display
    position: TipPosition::NearCursor,
    duration: Duration::UntilDismissed,
    max_per_session: 5,
    
    // Don't annoy users
    respect_dismiss: true,
    cooldown_hours: 24,
}
```

### Feature Spotlight

```
┌─────────────────────────────────────────┐
│ ✨ Did you know?                        │
├─────────────────────────────────────────┤
│                                         │
│ Hold [Alt] while dragging to see        │
│ live DRC feedback!                      │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │  [Component]  ←→  0.15mm clearance  │ │
│ │      ↓                              │ │
│ │   ✓ Valid placement                 │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ [Try it now]  [Got it]  [Don't show]    │
└─────────────────────────────────────────┘
```

## Sample Projects

### Built-in Examples

| Project | Complexity | Teaches |
|---------|------------|---------|
| LED Blinker | Beginner | Basic schematic, simple PCB |
| Arduino Shield | Intermediate | Multi-sheet, connectors |
| USB-C PD Sink | Intermediate | Power design, USB |
| STM32 Dev Board | Advanced | BGA, multi-layer, high-speed |
| RF Transceiver | Expert | Impedance control, RF layout |

### Project Templates

```rust
Templates {
    categories: vec![
        Category::Microcontroller,
        Category::PowerSupply,
        Category::Sensor,
        Category::Communication,
        Category::Audio,
        Category::Motor,
    ],
    
    // Each template includes
    template_contents: TemplateContents {
        schematic: true,
        pcb_outline: true,
        stackup: true,
        design_rules: true,
        bom_template: true,
        readme: true,
    },
}
```

## Help System

### Integrated Documentation

```rust
HelpSystem {
    // Quick help
    tooltip_help: true,
    extended_tooltips: true,  // Show on hover delay
    
    // Contextual help
    f1_context_help: true,    // F1 shows help for current tool
    
    // Search
    help_search: true,
    search_shortcut: "Ctrl+Shift+?",
    
    // Online resources
    link_to_docs: true,
    link_to_community: true,
}
```

### Help Panel

```
┌─────────────────────────────────────────┐
│ 🔍 Search help...                       │
├─────────────────────────────────────────┤
│                                         │
│ ▼ Getting Started                       │
│   • Quick Start Guide                   │
│   • Video Tutorials                     │
│   • Sample Projects                     │
│                                         │
│ ▼ Current Tool: Route Track             │
│   • Basic routing                       │
│   • Push-and-shove mode                 │
│   • Via insertion                       │
│   • Layer switching                     │
│                                         │
│ ▼ Keyboard Shortcuts                    │
│   • [Ctrl+?] Show all shortcuts         │
│                                         │
│ ─────────────────────────────────────── │
│ 💬 Community Forum                      │
│ 📖 Full Documentation                   │
│ 🐛 Report Issue                         │
└─────────────────────────────────────────┘
```

## Migration Assistance

### KiCAD Migration

```rust
KicadMigration {
    // Import projects
    import_projects: true,
    import_libraries: true,
    import_settings: true,
    
    // Shortcut compatibility
    offer_kicad_shortcuts: true,
    
    // Guidance
    show_differences: true,
    migration_tips: true,
}
```

### Migration Guide

```
┌─────────────────────────────────────────────────────────────────┐
│ 🔄 Migrating from KiCAD                                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Your KiCAD skills transfer directly! Here's what's different:  │
│                                                                 │
│ ┌─────────────────────┬─────────────────────────────────────┐  │
│ │ KiCAD               │ Hardware Tool                       │  │
│ ├─────────────────────┼─────────────────────────────────────┤  │
│ │ Eeschema + Pcbnew   │ Unified workspace                   │  │
│ │ .kicad_pro          │ .hwt (TOML-based)                   │  │
│ │ Python scripting    │ Rust + Code-first design            │  │
│ │ Manual zone refill  │ Live zone preview                   │  │
│ └─────────────────────┴─────────────────────────────────────┘  │
│                                                                 │
│ [Import KiCAD Project]  [Use KiCAD Shortcuts]  [Learn More]     │
└─────────────────────────────────────────────────────────────────┘
```

## Preferences Setup

### Initial Configuration

```rust
InitialSetup {
    steps: vec![
        SetupStep::Theme,           // Dark/Light/System
        SetupStep::Units,           // mm/mil/inch
        SetupStep::GridDefaults,    // Grid size preferences
        SetupStep::LibraryPaths,    // Where to find libraries
        SetupStep::AiSetup,         // Optional AI configuration
    ],
    
    // Can skip and configure later
    skippable: true,
    
    // Sensible defaults
    use_smart_defaults: true,
}
```

## Gamification (Optional)

### Achievement System

```rust
Achievements {
    enabled: true,  // Can disable in preferences
    
    achievements: vec![
        Achievement {
            id: "first_board",
            name: "First Board",
            description: "Complete your first PCB design",
            icon: "🎯",
        },
        Achievement {
            id: "zero_drc",
            name: "Clean Sweep",
            description: "Pass DRC with zero errors",
            icon: "✨",
        },
        Achievement {
            id: "speed_router",
            name: "Speed Router",
            description: "Route 100 connections in one session",
            icon: "⚡",
        },
    ],
    
    // Non-intrusive
    notification_style: NotificationStyle::Subtle,
}
```

## Related Topics

- [Main Window Layout](./main-window-layout.md)
- [Keyboard Shortcuts Reference](./keyboard-shortcuts-reference.md)
- [Accessibility & Theming](./accessibility-and-theming.md)
