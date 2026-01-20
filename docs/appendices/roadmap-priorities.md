# Roadmap & Priorities

## Overview

Hardware Tool development follows a phased approach, prioritizing core functionality first, then advanced features, and finally ecosystem expansion. This roadmap reflects the vision of creating a **delightful, fast, fluid, and modern** EDA experience.

## Development Phases

### Phase 1: Foundation (Months 1-6)

**Goal:** Core editing capabilities with basic import/export

| Feature | Priority | Status |
|---------|----------|--------|
| Project file format (.hwt) | P0 | 🔄 In Progress |
| Schematic editor (basic) | P0 | 🔄 In Progress |
| Symbol library system | P0 | 📋 Planned |
| PCB editor (basic) | P0 | 📋 Planned |
| Footprint library system | P0 | 📋 Planned |
| Gerber RS-274X export | P0 | 📋 Planned |
| Basic DRC | P0 | 📋 Planned |
| Basic ERC | P0 | 📋 Planned |
| KiCAD import | P1 | 📋 Planned |
| Dark/Light themes | P1 | 📋 Planned |

### Phase 2: Professional Features (Months 7-12)

**Goal:** Feature parity with professional tools

| Feature | Priority | Status |
|---------|----------|--------|
| Hierarchical schematics | P0 | 📋 Planned |
| Multi-layer PCB (up to 32) | P0 | 📋 Planned |
| Interactive push-and-shove router | P0 | 📋 Planned |
| Differential pair routing | P0 | 📋 Planned |
| Copper zones with thermals | P0 | 📋 Planned |
| IPC-2581 export | P1 | 📋 Planned |
| ODB++ export | P1 | 📋 Planned |
| 3D viewer (Bevy) | P1 | 📋 Planned |
| STEP export | P1 | 📋 Planned |
| BOM generation | P1 | 📋 Planned |
| Pick-and-place export | P1 | 📋 Planned |
| SPICE simulation integration | P2 | 📋 Planned |

### Phase 3: Innovation (Months 13-18)

**Goal:** Differentiated features that set Hardware Tool apart

| Feature | Priority | Status |
|---------|----------|--------|
| Programmatic/code-first design | P0 | 📋 Planned |
| Circuit JSON IR | P0 | 📋 Planned |
| Real-time schematic-PCB sync | P0 | 📋 Planned |
| Magnet cursor | P1 | 📋 Planned |
| Gesture router | P1 | 📋 Planned |
| Shadow nudge | P1 | 📋 Planned |
| Live zone pour preview | P1 | 📋 Planned |
| Visual net spy | P1 | 📋 Planned |
| Command palette 2.0 | P1 | 📋 Planned |
| Temporal history scrubber | P2 | 📋 Planned |
| Confidence heatmaps | P2 | 📋 Planned |

### Phase 4: AI Integration (Months 19-24)

**Goal:** Native AI-powered design assistance

| Feature | Priority | Status |
|---------|----------|--------|
| AI API integration framework | P0 | 📋 Planned |
| Natural language commands | P0 | 📋 Planned |
| AI-assisted component placement | P1 | 📋 Planned |
| AI-assisted routing | P1 | 📋 Planned |
| Design review AI | P1 | 📋 Planned |
| First-principles optimization | P1 | 📋 Planned |
| Real-time benchmarking | P1 | 📋 Planned |
| AI-powered DFM analysis | P2 | 📋 Planned |
| Predictive design suggestions | P2 | 📋 Planned |

### Phase 5: Ecosystem (Months 25-30)

**Goal:** Complete ecosystem and community

| Feature | Priority | Status |
|---------|----------|--------|
| Plugin/extension system | P0 | 📋 Planned |
| Cloud library hosting | P1 | 📋 Planned |
| Collaboration features | P1 | 📋 Planned |
| Manufacturer integrations | P1 | 📋 Planned |
| Component sourcing integration | P2 | 📋 Planned |
| Version control integration | P2 | 📋 Planned |
| Mobile companion app | P3 | 📋 Planned |

## Priority Definitions

| Priority | Definition |
|----------|------------|
| **P0** | Critical - Must have for release |
| **P1** | High - Important for user experience |
| **P2** | Medium - Nice to have |
| **P3** | Low - Future consideration |

## Status Definitions

| Status | Meaning |
|--------|---------|
| ✅ Complete | Feature is done and tested |
| 🔄 In Progress | Currently being developed |
| 📋 Planned | Scheduled for development |
| 💡 Proposed | Under consideration |
| ❌ Declined | Not planned |

## Key Milestones

### Alpha Release (Month 6)

- Basic schematic and PCB editing
- Gerber export
- KiCAD project import
- Core DRC/ERC

### Beta Release (Month 12)

- Full professional feature set
- 3D visualization
- All export formats
- Stable file format

### 1.0 Release (Month 18)

- Innovative UX features
- Code-first design
- Real-time sync
- Production ready

### 2.0 Release (Month 24)

- Full AI integration
- Native benchmarking
- First-principles optimization
- Enterprise features

## Technical Priorities

### Performance

1. 60+ FPS on large designs
2. < 2s project open time
3. < 100ms incremental DRC
4. Minimal memory footprint

### Quality

1. Zero data loss
2. Comprehensive test coverage
3. Accessibility compliance
4. Cross-platform consistency

### Developer Experience

1. Clean, documented API
2. Extensible architecture
3. Comprehensive CLI
4. Active community

## Community Involvement

### Open Source Strategy

- Core application: Open source (MIT/Apache 2.0)
- Standard libraries: Open source
- Premium features: Dual license
- Enterprise: Commercial license

### Contribution Areas

| Area | Difficulty | Impact |
|------|------------|--------|
| Symbol libraries | Easy | High |
| Footprint libraries | Easy | High |
| Documentation | Easy | Medium |
| Bug fixes | Medium | High |
| New features | Hard | High |
| Core architecture | Expert | Critical |

## Related Topics

- [Comparison with KiCAD and TSCircuit](./comparison-with-kicad-and-tscircuit.md)
- [Performance Targets](./performance-targets.md)
- [Project Structure](../core-architecture/project-structure-management.md)
