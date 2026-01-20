# Accessibility & Theming

## Overview

Hardware Tool is designed to be usable by everyone, including engineers with visual impairments, motor difficulties, or color vision deficiencies. Accessibility is not an afterthought — it's built into the core design system.

## Theme System

### Built-in Themes

| Theme | Description | Use Case |
|-------|-------------|----------|
| **Dark** (Default) | Low-light optimized | Reduces eye strain, professional preference |
| **Light** | High brightness | Well-lit environments, print preview |
| **High Contrast** | Maximum contrast | Visual impairments, bright sunlight |
| **OLED Dark** | True black backgrounds | OLED displays, battery saving |

### Theme Configuration

```rust
ThemeConfig {
    // Active theme
    active: Theme::Dark,
    
    // Auto-switching
    auto_switch: AutoSwitch {
        enabled: true,
        follow_system: true,
        schedule: Some(Schedule {
            dark_start: "18:00",
            light_start: "06:00",
        }),
    },
    
    // Customization
    allow_custom: true,
    accent_color_override: None,
}
```

### Custom Theme Creation

```rust
CustomTheme::new("My Theme")
    .base(Theme::Dark)
    .override_color("primary", "#FF6B6B")
    .override_color("bg_primary", "#0A0A0A")
    .override_font("primary", "Fira Sans")
    .save("my_theme.hwt_theme");
```

## Color Accessibility

### Color Blindness Support

```rust
ColorBlindMode {
    // Detection (optional)
    auto_detect: false,
    
    // Manual selection
    mode: ColorBlindType::None,  // or Deuteranopia, Protanopia, Tritanopia
    
    // Adaptations
    adaptations: ColorAdaptations {
        // Replace problematic colors
        use_safe_palette: true,
        
        // Add patterns to colored elements
        use_patterns: true,
        
        // Add icons alongside color indicators
        use_icons: true,
        
        // Add text labels
        use_labels: true,
    },
}
```

### Color Contrast Requirements

| Element | Minimum Contrast | Target |
|---------|------------------|--------|
| Body text | 4.5:1 | 7:1 |
| Large text (18px+) | 3:1 | 4.5:1 |
| UI components | 3:1 | 4.5:1 |
| Focus indicators | 3:1 | 4.5:1 |
| Graphical objects | 3:1 | 4.5:1 |

### Contrast Checker

```rust
// Built-in contrast validation
let contrast = color_contrast(foreground, background);
assert!(contrast >= 4.5, "Insufficient contrast for body text");

// Automatic adjustment
let adjusted = ensure_contrast(foreground, background, 4.5);
```

## Keyboard Navigation

### Full Keyboard Support

Every feature is accessible via keyboard:

```rust
KeyboardNavigation {
    // Focus management
    focus_visible: true,
    focus_ring_color: Color::Primary,
    focus_ring_width: 2,
    focus_ring_offset: 2,
    
    // Tab order
    logical_tab_order: true,
    skip_links: true,
    
    // Shortcuts
    customizable_shortcuts: true,
    conflict_detection: true,
}
```

### Focus Indicators

```
┌─────────────────────────────────────┐
│                                     │
│   ┌─────────────────────────────┐   │
│   │  ╔═══════════════════════╗  │   │  ← Focus ring (2px cyan)
│   │  ║   Focused Element     ║  │   │
│   │  ╚═══════════════════════╝  │   │
│   └─────────────────────────────┘   │
│                                     │
└─────────────────────────────────────┘
```

### Navigation Shortcuts

| Action | Shortcut |
|--------|----------|
| Next focusable | `Tab` |
| Previous focusable | `Shift+Tab` |
| Activate | `Enter` / `Space` |
| Cancel/Close | `Escape` |
| Navigate lists | `Arrow keys` |
| Jump to main | `Alt+M` |
| Jump to sidebar | `Alt+S` |
| Open command palette | `Ctrl+K` |

## Screen Reader Support

### ARIA Implementation

```rust
Accessibility {
    // Semantic structure
    use_semantic_html: true,
    aria_labels: true,
    aria_descriptions: true,
    
    // Live regions
    live_announcements: true,
    announcement_politeness: Politeness::Polite,
    
    // Landmarks
    landmarks: vec![
        Landmark::Main,
        Landmark::Navigation,
        Landmark::Complementary,
    ],
}
```

### Announcements

```rust
// Announce important changes
screen_reader.announce("Component R1 placed at 50, 40");
screen_reader.announce("DRC check complete: 3 errors found");
screen_reader.announce("Route completed: 45.2mm");
```

### Element Labels

```rust
Component {
    // Accessible name
    aria_label: "Resistor R1, 10 kilohms, at position 50 by 40 millimeters",
    
    // Description
    aria_description: "Connected to nets VCC and NODE_A",
    
    // Role
    role: "graphics-symbol",
}
```

## Motor Accessibility

### Large Touch Targets

```rust
TouchTargets {
    // Minimum sizes
    min_touch_target: 44,  // pixels (WCAG recommendation)
    min_click_target: 24,  // pixels for mouse
    
    // Spacing
    min_target_spacing: 8,  // pixels between targets
}
```

### Reduced Motion

```rust
ReducedMotion {
    // Respect system preference
    follow_system: true,
    
    // Manual override
    enabled: false,
    
    // When enabled
    adaptations: MotionAdaptations {
        disable_animations: false,  // Keep functional animations
        reduce_duration: true,      // Shorten to <100ms
        disable_parallax: true,
        disable_auto_play: true,
        use_fade_only: true,        // Replace complex animations
    },
}
```

### Sticky Keys & Dwell Click

```rust
MotorAssist {
    // Sticky modifiers
    sticky_keys: true,
    sticky_timeout: 5000,  // ms
    
    // Dwell clicking
    dwell_click: DwellClick {
        enabled: false,
        dwell_time: 1000,  // ms
        visual_feedback: true,
    },
    
    // Large cursor
    large_cursor: false,
    cursor_scale: 1.5,
}
```

## Visual Adjustments

### Text Scaling

```rust
TextScaling {
    // Support system font scaling
    respect_system_scale: true,
    
    // Manual adjustment
    scale_factor: 1.0,  // 0.75 to 2.0
    
    // Minimum readable size
    min_font_size: 12,
    
    // Line height adjustment
    line_height_scale: 1.0,
}
```

### Zoom Support

```rust
ZoomAccessibility {
    // UI zoom (separate from canvas zoom)
    ui_zoom: 1.0,  // 0.5 to 3.0
    
    // Reflow at high zoom
    reflow_layout: true,
    reflow_threshold: 2.0,
    
    // Preserve functionality
    no_loss_of_content: true,
    no_loss_of_functionality: true,
}
```

### Custom Cursors

```rust
CursorOptions {
    // Size
    size: CursorSize::Normal,  // Small, Normal, Large, ExtraLarge
    
    // Color
    color: CursorColor::System,  // or Custom(color)
    
    // Visibility
    highlight_on_move: false,
    trail: false,
}
```

## Cognitive Accessibility

### Simplified Mode

```rust
SimplifiedMode {
    enabled: false,
    
    features: SimplifiedFeatures {
        // Reduce visual complexity
        hide_advanced_panels: true,
        simplified_toolbars: true,
        
        // Clearer feedback
        extended_tooltips: true,
        step_by_step_guidance: true,
        
        // Reduced distractions
        disable_animations: true,
        muted_colors: true,
    },
}
```

### Reading Aids

```rust
ReadingAids {
    // Dyslexia support
    dyslexia_font: false,  // OpenDyslexic
    
    // Focus mode
    focus_mode: false,     // Dims non-active areas
    
    // Reading guide
    reading_guide: false,  // Horizontal line follows cursor
}
```

## Testing & Compliance

### Accessibility Audit

```rust
// Run accessibility audit
let audit = AccessibilityAuditor::audit(&ui);

println!("WCAG 2.1 AA Compliance: {}", audit.wcag_aa_compliant);
println!("Issues found: {}", audit.issues.len());

for issue in audit.issues {
    println!("  [{:?}] {}: {}", 
             issue.severity, 
             issue.element, 
             issue.description);
}
```

### Compliance Targets

| Standard | Level | Status |
|----------|-------|--------|
| WCAG 2.1 | AA | ✓ Target |
| WCAG 2.1 | AAA | Partial |
| Section 508 | Full | ✓ Target |
| EN 301 549 | Full | ✓ Target |

## Related Topics

- [Visual Style Guidelines](./visual-style-guidelines.md)
- [Keyboard Shortcuts Reference](./keyboard-shortcuts-reference.md)
- [Onboarding & First Experience](./onboarding-and-first-experience.md)
