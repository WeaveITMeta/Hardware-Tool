# GDSII Export

## Overview

Hardware Tool exports IC layouts in GDSII Stream format (GDSII II), the industry-standard binary format for IC mask data. Full support for hierarchical designs, parameterized cells, and foundry-specific layer mapping.

## GDSII Format

```rust
GDSIIExport {
    // Format settings
    format: GDSIIFormat {
        version: 6,                    // GDSII version
        units: GDSUnits {
            user_unit: 1e-6,           // 1 µm
            database_unit: 1e-9,       // 1 nm
        },
    },
    
    // Export options
    options: ExportOptions {
        flatten: false,                // Preserve hierarchy
        merge_cells: false,            // Keep cell boundaries
        snap_to_grid: true,            // Snap to database grid
        remove_empty_cells: true,
    },
    
    // Layer mapping
    layer_map: LayerMap {
        source: LayerMapSource::PDK,   // Use PDK layer map
        custom_overrides: vec![],
    },
    
    // Cell filtering
    cells: CellFilter {
        include_all: true,
        exclude_patterns: vec!["test_*", "debug_*"],
    },
}
```

## Layer Mapping

```
┌─────────────────────────────────────────────────────────────────┐
│ Layer Mapping: SKY130                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Internal Layer    │ GDS Layer │ GDS Datatype │ Description     │
│ ──────────────────┼───────────┼──────────────┼────────────     │
│ nwell             │    64     │      20      │ N-well          │
│ diff              │    65     │      20      │ Diffusion       │
│ tap               │    65     │      44      │ Substrate tap   │
│ poly              │    66     │      20      │ Polysilicon     │
│ licon             │    66     │      44      │ Local interconn │
│ li1               │    67     │      20      │ Local intercon  │
│ mcon              │    67     │      44      │ Metal1 contact  │
│ met1              │    68     │      20      │ Metal 1         │
│ via               │    68     │      44      │ Via 1-2         │
│ met2              │    69     │      20      │ Metal 2         │
│ via2              │    69     │      44      │ Via 2-3         │
│ met3              │    70     │      20      │ Metal 3         │
│ via3              │    70     │      44      │ Via 3-4         │
│ met4              │    71     │      20      │ Metal 4         │
│ via4              │    71     │      44      │ Via 4-5         │
│ met5              │    72     │      20      │ Metal 5         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Export UI

```
┌─────────────────────────────────────────────────────────────────┐
│ GDSII Export                                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Output File: [my_chip.gds                              ] [...]  │
│                                                                 │
│ Top Cell: [top                                    ▼]            │
│                                                                 │
│ Options:                                                        │
│ ☑ Preserve hierarchy                                           │
│ ☑ Snap to grid (1 nm)                                          │
│ ☐ Flatten design                                               │
│ ☑ Remove empty cells                                           │
│ ☑ Include text labels                                          │
│                                                                 │
│ Layer Mapping:                                                  │
│ ● Use PDK layer map                                            │
│ ○ Custom layer map: [                              ] [...]     │
│                                                                 │
│ Cell Filter:                                                    │
│ ☑ Include all cells                                            │
│ ☐ Exclude patterns: [test_*, debug_*              ]            │
│                                                                 │
│ Statistics:                                                     │
│   Cells: 1,234                                                 │
│   Polygons: 4,567,890                                          │
│   Estimated file size: 125 MB                                  │
│                                                                 │
│ [Cancel]                                    [Export GDSII]      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Rust API

```rust
// Export layout to GDSII
let layout = project.get_layout("top")?;

layout.export_gdsii(GDSIIConfig {
    path: "output/my_chip.gds",
    top_cell: "top",
    flatten: false,
    layer_map: LayerMap::from_pdk(&pdk),
})?;

// Verify export
let stats = layout.gdsii_stats()?;
println!("Cells: {}", stats.cell_count);
println!("Polygons: {}", stats.polygon_count);
println!("File size: {} MB", stats.file_size / 1024 / 1024);
```

## Related Topics

- [OASIS Export](./oasis-export.md) - Compressed format
- [LEF/DEF Export](./lef-def-export.md) - Library exchange
- [Tapeout Checklist](./tapeout-checklist-generation.md) - Signoff
