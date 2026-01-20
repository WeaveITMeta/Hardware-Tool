# Visual Style Guidelines

## Overview

Hardware Tool's visual design creates an experience that feels **delightful, fast, fluid, and modern** — something professional engineers enjoy using daily while remaining discoverable for beginners. Every visual element serves a purpose and provides meaningful feedback.

## Color System

### Primary Palette

| Color | Hex | Usage |
|-------|-----|-------|
| **Primary Accent** | `#00D4FF` | Interactive elements, highlights, focus states |
| **Success** | `#00FF9D` | Confirmations, valid connections, passing checks |
| **Warning** | `#FFB800` | Cautions, near-violations, attention needed |
| **Critical** | `#FF3366` | Errors, violations, destructive actions |
| **Copper** | `#B87333 → #FFD700` | PCB copper (metallic gradient) |

### Semantic Colors

```rust
ColorSystem {
    // Interactive
    primary: "#00D4FF",      // Cyan - energetic but not aggressive
    primary_hover: "#33DDFF",
    primary_pressed: "#00A8CC",
    
    // Feedback
    success: "#00FF9D",      // Mint green
    warning: "#FFB800",      // Amber
    error: "#FF3366",        // Coral red
    info: "#6C7A89",         // Slate
    
    // PCB-specific
    copper: Gradient::linear("#B87333", "#FFD700"),
    soldermask_green: "#006847",
    soldermask_blue: "#00447C",
    soldermask_red: "#8B0000",
    soldermask_black: "#1A1A1A",
    silkscreen: "#FFFFFF",
    substrate: "#C2A366",
    
    // Net highlighting
    net_highlight: "#00D4FF",
    net_dimmed: "#404040",
    ratsnest: "#FFFF00",
}
```

### Dark Theme (Default)

```rust
DarkTheme {
    // Backgrounds
    bg_primary: "#0D0D0D",      // Main canvas
    bg_secondary: "#1A1A1A",    // Panels
    bg_tertiary: "#262626",     // Cards, inputs
    bg_elevated: "#333333",     // Dropdowns, tooltips
    
    // Text
    text_primary: "#FFFFFF",
    text_secondary: "#B3B3B3",
    text_tertiary: "#666666",
    text_disabled: "#404040",
    
    // Borders
    border_subtle: "#333333",
    border_default: "#404040",
    border_strong: "#666666",
}
```

### Light Theme

```rust
LightTheme {
    // Backgrounds
    bg_primary: "#FFFFFF",
    bg_secondary: "#F5F5F5",
    bg_tertiary: "#EBEBEB",
    bg_elevated: "#FFFFFF",
    
    // Text
    text_primary: "#1A1A1A",
    text_secondary: "#666666",
    text_tertiary: "#999999",
    text_disabled: "#CCCCCC",
    
    // Borders
    border_subtle: "#EBEBEB",
    border_default: "#D9D9D9",
    border_strong: "#B3B3B3",
}
```

### High Contrast Mode

```rust
HighContrastTheme {
    // Maximum contrast for accessibility
    bg_primary: "#000000",
    text_primary: "#FFFFFF",
    
    // Distinct colors
    primary: "#00FFFF",
    success: "#00FF00",
    warning: "#FFFF00",
    error: "#FF0000",
    
    // Thick borders
    border_width: 2.0,
}
```

## Color-Blind Friendly Palettes

### Deuteranopia/Protanopia Safe

```rust
ColorBlindSafe {
    // Avoid red-green confusion
    success: "#0077BB",      // Blue instead of green
    error: "#EE7733",        // Orange instead of red
    warning: "#CCBB44",      // Yellow (unchanged)
    
    // Use patterns in addition to color
    use_patterns: true,
    use_icons: true,
}
```

### Palette Variants

| Mode | Success | Warning | Error |
|------|---------|---------|-------|
| Normal | 🟢 #00FF9D | 🟡 #FFB800 | 🔴 #FF3366 |
| Deuteranopia | 🔵 #0077BB | 🟡 #CCBB44 | 🟠 #EE7733 |
| Tritanopia | 🟢 #00FF9D | 🟣 #CC79A7 | 🔴 #D55E00 |

## Typography

### Font Stack

```rust
Typography {
    // Primary UI font
    primary: FontStack::new()
        .add("Inter", FontWeight::Variable(100..900))
        .fallback("system-ui")
        .fallback("sans-serif"),
    
    // Monospace for code/values
    monospace: FontStack::new()
        .add("JetBrains Mono", FontWeight::Variable(100..800))
        .fallback("Iosevka")
        .fallback("Consolas")
        .fallback("monospace"),
    
    // Technical drawings
    technical: FontStack::new()
        .add("ISOCPEUR")  // ISO standard for technical drawings
        .fallback("Inter"),
}
```

### Type Scale

| Name | Size | Weight | Usage |
|------|------|--------|-------|
| Display | 32px | 600 | Welcome screens, empty states |
| Title | 24px | 600 | Panel headers, dialogs |
| Heading | 18px | 600 | Section headers |
| Body | 14px | 400 | Primary content |
| Caption | 12px | 400 | Secondary info, labels |
| Micro | 10px | 500 | Badges, status indicators |

### Code Typography

```rust
CodeTypography {
    font: "JetBrains Mono",
    size: 13,
    line_height: 1.5,
    
    // Syntax highlighting
    keyword: "#FF79C6",
    string: "#F1FA8C",
    number: "#BD93F9",
    comment: "#6272A4",
    function: "#50FA7B",
    type_: "#8BE9FD",
}
```

## Motion & Animation

### Animation Principles

1. **Purposeful** — Every animation conveys meaning
2. **Quick** — Never make users wait (0.15–0.4s)
3. **Natural** — Spring physics, not linear
4. **Subtle** — Enhance, don't distract

### Timing Functions

```rust
Easing {
    // Standard interactions
    default: "cubic-bezier(0.4, 0.0, 0.2, 1)",      // Material standard
    
    // Entering elements
    enter: "cubic-bezier(0.0, 0.0, 0.2, 1)",        // Decelerate
    
    // Exiting elements
    exit: "cubic-bezier(0.4, 0.0, 1, 1)",           // Accelerate
    
    // Elastic/spring
    spring: "spring(1, 80, 10)",                     // Bouncy
    
    // Connection lines
    elastic: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
}
```

### Duration Scale

| Duration | Usage |
|----------|-------|
| 0.1s | Micro-interactions (hover, press) |
| 0.15s | State changes (toggle, select) |
| 0.2s | Small movements (nudge, snap) |
| 0.3s | Medium transitions (panel open) |
| 0.4s | Large transitions (mode switch) |
| 0.5s+ | Emphasis animations (zone fill) |

### Signature Animations

```rust
Animations {
    // Connection made
    connection_success: Animation {
        type_: "pulse_glow",
        color: Color::Success,
        duration: 0.3,
        easing: Easing::Spring,
    },
    
    // Zone fill
    zone_flood: Animation {
        type_: "flood_fill",
        duration: 0.3,
        direction: FloodDirection::FromCenter,
    },
    
    // Wire dragging
    wire_elastic: Animation {
        type_: "elastic_line",
        tension: 0.8,
        damping: 0.3,
    },
    
    // Component placement
    component_drop: Animation {
        type_: "scale_bounce",
        from: 1.2,
        to: 1.0,
        duration: 0.2,
    },
}
```

## Iconography

### Icon Style

- **Style**: Outlined, 1.5px stroke
- **Size**: 16px grid, 20px touch targets
- **Corners**: 2px radius
- **Optical alignment**: Centered in bounding box

### Icon Categories

```rust
Icons {
    // Tools
    select: "cursor-default",
    wire: "vector-line",
    component: "chip",
    zone: "rectangle-fill",
    dimension: "ruler",
    
    // Actions
    rotate: "rotate-cw",
    flip: "flip-horizontal",
    delete: "trash-2",
    duplicate: "copy",
    
    // Status
    success: "check-circle",
    warning: "alert-triangle",
    error: "x-circle",
    info: "info",
    
    // Navigation
    zoom_in: "zoom-in",
    zoom_out: "zoom-out",
    fit: "maximize-2",
    layers: "layers",
}
```

## Shadows & Elevation

### Elevation Scale

```rust
Elevation {
    // Level 0: Flat (canvas elements)
    level_0: Shadow::none(),
    
    // Level 1: Subtle lift (cards, inputs)
    level_1: Shadow {
        offset_y: 1,
        blur: 3,
        color: "rgba(0,0,0,0.12)",
    },
    
    // Level 2: Raised (dropdowns, tooltips)
    level_2: Shadow {
        offset_y: 4,
        blur: 8,
        color: "rgba(0,0,0,0.16)",
    },
    
    // Level 3: Floating (modals, popovers)
    level_3: Shadow {
        offset_y: 8,
        blur: 24,
        color: "rgba(0,0,0,0.24)",
    },
    
    // Level 4: Overlay (command palette)
    level_4: Shadow {
        offset_y: 16,
        blur: 48,
        color: "rgba(0,0,0,0.32)",
    },
}
```

## Component Styling

### Buttons

```rust
Button {
    // Primary
    primary: ButtonStyle {
        background: Color::Primary,
        text: Color::White,
        border: None,
        hover: Color::PrimaryHover,
        pressed: Color::PrimaryPressed,
    },
    
    // Secondary
    secondary: ButtonStyle {
        background: Color::Transparent,
        text: Color::Primary,
        border: Color::Primary,
        hover: Color::Primary.with_alpha(0.1),
    },
    
    // Ghost
    ghost: ButtonStyle {
        background: Color::Transparent,
        text: Color::TextSecondary,
        border: None,
        hover: Color::BgTertiary,
    },
    
    // Sizing
    height_sm: 28,
    height_md: 36,
    height_lg: 44,
    
    // Border radius
    radius: 6,
}
```

### Inputs

```rust
Input {
    background: Color::BgTertiary,
    border: Color::BorderDefault,
    border_focus: Color::Primary,
    text: Color::TextPrimary,
    placeholder: Color::TextTertiary,
    
    height: 36,
    padding_x: 12,
    radius: 6,
    
    // States
    hover_border: Color::BorderStrong,
    error_border: Color::Error,
    disabled_opacity: 0.5,
}
```

## PCB-Specific Visuals

### Layer Colors

```rust
LayerColors {
    f_cu: "#FF0000",      // Top copper - Red
    b_cu: "#0000FF",      // Bottom copper - Blue
    in1_cu: "#FFFF00",    // Inner 1 - Yellow
    in2_cu: "#00FF00",    // Inner 2 - Green
    
    f_mask: "#800080",    // Top mask - Purple
    b_mask: "#008080",    // Bottom mask - Teal
    
    f_silk: "#00FFFF",    // Top silk - Cyan
    b_silk: "#FF00FF",    // Bottom silk - Magenta
    
    edge_cuts: "#FFFF00", // Board outline - Yellow
    
    // Customizable per-user
    customizable: true,
}
```

### Copper Rendering

```rust
CopperStyle {
    // Realistic mode
    realistic: CopperRealistic {
        base_color: "#B87333",
        highlight: "#FFD700",
        roughness: 0.3,
        metallic: 0.9,
    },
    
    // Schematic mode
    schematic: CopperSchematic {
        fill: Color::LayerColor,
        outline: Color::LayerColor.darken(0.2),
        outline_width: 0.1,
    },
}
```

## Related Topics

- [Main Window Layout](./main-window-layout.md)
- [Accessibility & Theming](./accessibility-and-theming.md)
- [Innovative Interaction Patterns](./innovative-interaction-patterns.md)
