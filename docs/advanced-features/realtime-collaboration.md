# Real-Time Collaboration

## Overview

Hardware Tool enables Google Docs-style real-time collaboration for hardware design teams. Multiple engineers can simultaneously edit schematics, PCB layouts, and documentation with instant synchronization, conflict resolution, and integrated review workflows.

### Why Real-Time Collaboration?

| Traditional Workflow | Real-Time Collaboration |
|---------------------|------------------------|
| File locking, one editor at a time | Multiple simultaneous editors |
| Email file attachments | Live shared workspace |
| Manual merge conflicts | Automatic conflict resolution |
| Delayed feedback | Instant review and comments |
| Version confusion | Single source of truth |

## Architecture

### Collaboration Infrastructure

```
┌─────────────────────────────────────────────────────────────────┐
│                    Collaboration Architecture                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Client A  │  │   Client B  │  │   Client C  │             │
│  │  (Designer) │  │  (Reviewer) │  │  (Layout)   │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          │                                      │
│                          ▼                                      │
│              ┌───────────────────────┐                         │
│              │   WebSocket Server    │                         │
│              │   (Real-time sync)    │                         │
│              └───────────┬───────────┘                         │
│                          │                                      │
│         ┌────────────────┼────────────────┐                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │    CRDT     │  │  Presence   │  │   History   │             │
│  │   Engine    │  │   Service   │  │   Service   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│         │                │                │                     │
│         └────────────────┼────────────────┘                     │
│                          ▼                                      │
│              ┌───────────────────────┐                         │
│              │   Persistent Storage  │                         │
│              │   (Git-compatible)    │                         │
│              └───────────────────────┘                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### CRDT-Based Synchronization

Conflict-free Replicated Data Types ensure consistency without locks:

```rust
CRDTEngine {
    // Document model
    document: CRDTDocument {
        // Schematic elements
        schematic: CRDTMap<ElementId, SchematicElement>,
        
        // PCB elements
        pcb: CRDTMap<ElementId, PCBElement>,
        
        // Connections
        connections: CRDTSet<Connection>,
        
        // Properties
        properties: CRDTMap<PropertyKey, PropertyValue>,
    },
    
    // Conflict resolution
    resolution: ConflictResolution {
        strategy: Strategy::LastWriterWins,
        
        // Per-element locking for critical operations
        element_locks: true,
        lock_timeout: Duration::seconds(30),
    },
    
    // Synchronization
    sync: SyncConfig {
        interval: Duration::milliseconds(50),  // 20 Hz
        batch_operations: true,
        compression: true,
    },
}
```

## Presence Awareness

### Live Cursors and Selections

See where teammates are working in real-time:

```rust
PresenceSystem {
    // Cursor tracking
    cursors: CursorTracking {
        enabled: true,
        update_rate: 60,              // Hz
        show_names: true,
        fade_inactive: Duration::seconds(30),
    },
    
    // Selection highlighting
    selections: SelectionTracking {
        enabled: true,
        show_component_selections: true,
        show_net_selections: true,
        color_per_user: true,
    },
    
    // Activity indicators
    activity: ActivityIndicators {
        show_editing_regions: true,
        show_view_regions: true,
        typing_indicator: true,
    },
}
```

### Presence UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Schematic Editor                              [3 collaborators] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │      ┌─────┐                    ┌─────┐                    ││
│  │      │ U1  │────────────────────│ U2  │                    ││
│  │      │     │      ▲             │     │                    ││
│  │      └─────┘      │             └─────┘                    ││
│  │                   │                                         ││
│  │              ┌────┴────┐                                   ││
│  │              │ 🔵 Alex │  ← Alex's cursor                  ││
│  │              └─────────┘                                   ││
│  │                                                             ││
│  │  ┌───────────────────┐                                     ││
│  │  │ 🟢 Sarah editing  │  ← Sarah's selection               ││
│  │  │ ┌─────┐  ┌─────┐  │                                     ││
│  │  │ │ R1  │──│ R2  │  │                                     ││
│  │  │ └─────┘  └─────┘  │                                     ││
│  │  └───────────────────┘                                     ││
│  │                                                             ││
│  │                              ┌─────────┐                   ││
│  │                              │ 🟡 Mike │  ← Mike viewing   ││
│  │                              │ viewing │                   ││
│  │                              └─────────┘                   ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Collaborators:                                                  │
│ 🔵 Alex Chen (editing schematic)                               │
│ 🟢 Sarah Kim (editing power section)                           │
│ 🟡 Mike Johnson (viewing)                                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Collaborative Editing

### Simultaneous Editing Modes

```rust
CollaborativeEditing {
    // Editing modes
    modes: EditingModes {
        // Free editing (all can edit anywhere)
        free: FreeEditing {
            enabled: true,
            conflict_highlight: true,
        },
        
        // Region-based (claim areas)
        region_based: RegionEditing {
            enabled: false,
            auto_claim: true,
            claim_timeout: Duration::minutes(5),
        },
        
        // Role-based (permissions per area)
        role_based: RoleEditing {
            enabled: false,
            roles: vec!["schematic_lead", "layout_lead", "reviewer"],
        },
    },
    
    // Operation types
    operations: OperationConfig {
        // Atomic operations
        atomic: vec![
            Operation::MoveComponent,
            Operation::RouteTrace,
            Operation::PlaceVia,
        ],
        
        // Compound operations (can be interrupted)
        compound: vec![
            Operation::AutoRoute,
            Operation::CopperPour,
        ],
    },
}
```

### Conflict Resolution

```rust
ConflictResolution {
    // Detection
    detection: ConflictDetection {
        // Spatial conflicts
        spatial: true,
        spatial_threshold: 1.0,       // mm
        
        // Logical conflicts
        logical: true,                // Same net, component, etc.
        
        // Timing conflicts
        timing_window: Duration::milliseconds(100),
    },
    
    // Resolution strategies
    strategies: vec![
        // Auto-resolve simple conflicts
        Strategy::AutoMerge {
            for_operations: vec![Operation::Move, Operation::Rotate],
        },
        
        // Prompt for complex conflicts
        Strategy::UserPrompt {
            for_operations: vec![Operation::Delete, Operation::Reconnect],
            timeout: Duration::seconds(30),
            default: DefaultAction::KeepBoth,
        },
        
        // Lock for critical operations
        Strategy::Lock {
            for_operations: vec![Operation::SchematicRestructure],
        },
    ],
}
```

### Conflict Resolution UI

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠ Conflict Detected                                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Both you and Sarah modified component R5:                       │
│                                                                 │
│ ┌─────────────────────────┐  ┌─────────────────────────┐       │
│ │ Your change:            │  │ Sarah's change:         │       │
│ │                         │  │                         │       │
│ │ R5: 10kΩ → 12kΩ        │  │ R5: 10kΩ → 15kΩ        │       │
│ │ Position: (25, 30)      │  │ Position: (25, 30)      │       │
│ │                         │  │                         │       │
│ └─────────────────────────┘  └─────────────────────────┘       │
│                                                                 │
│ Resolution options:                                             │
│                                                                 │
│ [Keep Mine (12kΩ)]  [Keep Sarah's (15kΩ)]  [Discuss in Chat]  │
│                                                                 │
│ ○ Always prefer my changes for value edits                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Design Review

### Inline Comments and Annotations

```rust
ReviewSystem {
    // Comment types
    comments: CommentConfig {
        // Point comments (on specific location)
        point_comments: true,
        
        // Element comments (on component/net)
        element_comments: true,
        
        // Region comments (on area)
        region_comments: true,
        
        // Thread replies
        threaded: true,
        max_depth: 10,
    },
    
    // Annotations
    annotations: AnnotationConfig {
        // Drawing tools
        tools: vec![
            AnnotationTool::Arrow,
            AnnotationTool::Circle,
            AnnotationTool::Rectangle,
            AnnotationTool::Freehand,
            AnnotationTool::Text,
        ],
        
        // Visibility
        visibility: AnnotationVisibility::AllCollaborators,
        
        // Persistence
        persist: true,
    },
    
    // Review workflow
    workflow: ReviewWorkflow {
        // Review states
        states: vec!["pending", "in_review", "approved", "changes_requested"],
        
        // Approvers
        require_approval: true,
        min_approvers: 1,
        
        // Sign-off
        sign_off_required: true,
    },
}
```

### Review UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Design Review: Power Supply Section                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                                                             ││
│  │      ┌─────┐                                               ││
│  │      │ U1  │──────┬──────────────────                      ││
│  │      │     │      │                                        ││
│  │      └─────┘      │    💬 ← Comment marker                 ││
│  │                   │    ┌─────────────────────────────┐     ││
│  │              ┌────┴────│ Alex: Should we add a TVS   │     ││
│  │              │ C1      │ here for ESD protection?    │     ││
│  │              │ 100µF   │                             │     ││
│  │              └─────────│ Sarah: Good catch! Adding   │     ││
│  │                        │ PESD5V0S1BL to schematic.   │     ││
│  │                        │ ✓ Resolved                  │     ││
│  │                        └─────────────────────────────┘     ││
│  │                                                             ││
│  │  ┌─────────────────────────────────────────────────────┐   ││
│  │  │ 🔴 Review annotation                                │   ││
│  │  │ ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐ │   ││
│  │  │ │  Thermal concern: Add copper pour here        │ │   ││
│  │  │ └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘ │   ││
│  │  └─────────────────────────────────────────────────────┘   ││
│  │                                                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Review Status: 2/3 sections approved                            │
│ Comments: 5 total, 3 resolved, 2 open                          │
│                                                                 │
│ [Add Comment] [Approve Section] [Request Changes] [Complete]    │
└─────────────────────────────────────────────────────────────────┘
```

## Version Control Integration

### Git-Compatible Workflow

```rust
VersionControl {
    // Backend
    backend: VersionControlBackend::Git,
    
    // Auto-commit
    auto_commit: AutoCommitConfig {
        enabled: true,
        interval: Duration::minutes(5),
        on_significant_change: true,
        message_template: "Auto-save: {changes_summary}",
    },
    
    // Branching
    branching: BranchingConfig {
        // Feature branches
        feature_branches: true,
        branch_prefix: "feature/",
        
        // Review branches
        review_branches: true,
        review_prefix: "review/",
        
        // Auto-merge
        auto_merge: AutoMergeConfig {
            enabled: true,
            require_review: true,
            require_ci_pass: true,
        },
    },
    
    // Diff visualization
    diff: DiffConfig {
        visual_diff: true,
        show_component_changes: true,
        show_routing_changes: true,
        show_property_changes: true,
    },
}
```

### Visual Diff

```
┌─────────────────────────────────────────────────────────────────┐
│ Visual Diff: main ← feature/add-usb-c                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Changes Summary:                                                │
│ +12 components, -2 components, ~5 modified, +45 traces         │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │  ┌─────┐                    ┌─────┐                        │ │
│ │  │ U1  │────────────────────│ U2  │  (unchanged)           │ │
│ │  └─────┘                    └─────┘                        │ │
│ │                                                             │ │
│ │  🟢 ┌─────┐                                                │ │
│ │     │ U5  │  (added: USB-C controller)                     │ │
│ │     │     │                                                 │ │
│ │     └─────┘                                                 │ │
│ │         │                                                   │ │
│ │  🟢 ════╪════════════════════  (added: USB traces)         │ │
│ │         │                                                   │ │
│ │  🟡 ┌─────┐                                                │ │
│ │     │ R3  │  (modified: 10k → 5.1k)                        │ │
│ │     └─────┘                                                 │ │
│ │                                                             │ │
│ │  🔴 ┌─────┐                                                │ │
│ │     │ J2  │  (removed: old connector)                      │ │
│ │     └─────┘                                                 │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ Legend: 🟢 Added  🟡 Modified  🔴 Removed                      │
│                                                                 │
│ [Merge] [Request Review] [View Full Diff] [Close]               │
└─────────────────────────────────────────────────────────────────┘
```

## Team Communication

### Integrated Chat

```rust
TeamChat {
    // Chat channels
    channels: ChannelConfig {
        // Project channel
        project_channel: true,
        
        // Per-sheet channels
        sheet_channels: true,
        
        // Direct messages
        direct_messages: true,
    },
    
    // Features
    features: ChatFeatures {
        // Rich content
        code_snippets: true,
        file_attachments: true,
        screen_shares: true,
        
        // Design references
        component_mentions: true,    // @U1, @R5
        net_mentions: true,          // #VCC, #GND
        location_links: true,        // Link to specific location
        
        // Notifications
        mention_notifications: true,
        review_notifications: true,
    },
}
```

### Chat UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Team Chat: Power Supply Design                          [─][□][×]│
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Alex Chen (2:34 PM):                                           │
│ Hey team, I'm seeing some noise on #VDD_CORE. Anyone else?     │
│                                                                 │
│ Sarah Kim (2:35 PM):                                           │
│ Yes! I noticed that too. Check @C15 - I think we need more     │
│ decoupling near @U1.                                           │
│ 📍 [View Location]                                             │
│                                                                 │
│ Mike Johnson (2:36 PM):                                        │
│ Running PI analysis now... Results:                            │
│ ┌─────────────────────────────────────────────────────────────┐│
│ │ PDN Impedance @ 10MHz: 45mΩ (target: 20mΩ)                 ││
│ │ Recommendation: Add 2x 10µF near U1                        ││
│ └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│ Alex Chen (2:38 PM):                                           │
│ Great catch! I'll add those caps now.                          │
│ ✓ Added C16, C17 (10µF) near U1                               │
│ 📍 [View Changes]                                              │
│                                                                 │
│ Sarah Kim (2:39 PM):                                           │
│ 👍 Looks good! Re-running PI...                                │
│ ✓ PDN Impedance @ 10MHz: 18mΩ - PASS                          │
│                                                                 │
│ ─────────────────────────────────────────────────────────────── │
│ Type a message... @mention #net 📎 📍                   [Send] │
└─────────────────────────────────────────────────────────────────┘
```

## Access Control

### Role-Based Permissions

```rust
AccessControl {
    // Roles
    roles: vec![
        Role {
            name: "owner",
            permissions: Permissions::all(),
        },
        Role {
            name: "admin",
            permissions: Permissions {
                edit_schematic: true,
                edit_pcb: true,
                manage_users: true,
                delete_project: false,
                export: true,
                review: true,
            },
        },
        Role {
            name: "designer",
            permissions: Permissions {
                edit_schematic: true,
                edit_pcb: true,
                manage_users: false,
                delete_project: false,
                export: true,
                review: false,
            },
        },
        Role {
            name: "reviewer",
            permissions: Permissions {
                edit_schematic: false,
                edit_pcb: false,
                manage_users: false,
                delete_project: false,
                export: false,
                review: true,
            },
        },
        Role {
            name: "viewer",
            permissions: Permissions::view_only(),
        },
    ],
    
    // Sharing
    sharing: SharingConfig {
        link_sharing: true,
        link_expiration: Some(Duration::days(30)),
        password_protection: true,
    },
}
```

### Team Management UI

```
┌─────────────────────────────────────────────────────────────────┐
│ Team Management: Power Supply Project                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Team Members (4):                                               │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 👤 Alex Chen                                    Owner       │ │
│ │    alex@company.com                             [Manage ▼]  │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ 👤 Sarah Kim                                    Designer    │ │
│ │    sarah@company.com                  Online 🟢 [Manage ▼]  │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ 👤 Mike Johnson                                 Designer    │ │
│ │    mike@company.com                   Online 🟢 [Manage ▼]  │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ 👤 Lisa Wang                                    Reviewer    │ │
│ │    lisa@company.com                   Offline 🔴 [Manage ▼] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [+ Invite Member]  [Manage Roles]  [Share Link]                 │
│                                                                 │
│ Sharing:                                                        │
│ 🔗 https://hwt.io/p/abc123 (expires in 25 days)                │
│ [Copy Link] [Regenerate] [Disable]                              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Offline Support

### Offline-First Architecture

```rust
OfflineSupport {
    // Local storage
    local_storage: LocalStorageConfig {
        enabled: true,
        max_size: 1024 * 1024 * 1024,  // 1 GB
        sync_on_connect: true,
    },
    
    // Offline editing
    offline_editing: OfflineEditingConfig {
        enabled: true,
        queue_operations: true,
        max_queue_size: 10000,
    },
    
    // Sync on reconnect
    reconnect: ReconnectConfig {
        auto_sync: true,
        conflict_resolution: ConflictResolution::Prompt,
        merge_strategy: MergeStrategy::ThreeWay,
    },
}
```

### Offline Indicator

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠ Working Offline                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ You're currently offline. Changes are being saved locally.     │
│                                                                 │
│ Pending changes: 23 operations                                  │
│ Last sync: 15 minutes ago                                       │
│                                                                 │
│ When you reconnect:                                             │
│ • Your changes will be automatically synced                    │
│ • Any conflicts will be highlighted for review                 │
│ • Team members will see your updates                           │
│                                                                 │
│ [Work Offline] [Try Reconnect]                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Rust Implementation

### CRDT Core

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════
// CRDT Document
// ═══════════════════════════════════════════════════════════════

struct CRDTDocument {
    // Lamport timestamp for ordering
    clock: LamportClock,
    
    // Site ID (unique per client)
    site_id: SiteId,
    
    // Document state
    elements: HashMap<ElementId, CRDTElement>,
    
    // Operation log
    operations: Vec<Operation>,
}

impl CRDTDocument {
    fn apply_operation(&mut self, op: Operation) -> Result<(), CRDTError> {
        // Check causality
        if !self.can_apply(&op) {
            return Err(CRDTError::CausalityViolation);
        }
        
        match op.kind {
            OpKind::Insert { id, element } => {
                self.elements.insert(id, CRDTElement {
                    value: element,
                    timestamp: op.timestamp,
                    site_id: op.site_id,
                    deleted: false,
                });
            }
            OpKind::Update { id, property, value } => {
                if let Some(elem) = self.elements.get_mut(&id) {
                    // Last-writer-wins for updates
                    if op.timestamp > elem.timestamp {
                        elem.set_property(property, value);
                        elem.timestamp = op.timestamp;
                    }
                }
            }
            OpKind::Delete { id } => {
                if let Some(elem) = self.elements.get_mut(&id) {
                    elem.deleted = true;
                    elem.timestamp = op.timestamp;
                }
            }
            OpKind::Move { id, position } => {
                if let Some(elem) = self.elements.get_mut(&id) {
                    if op.timestamp > elem.timestamp {
                        elem.set_position(position);
                        elem.timestamp = op.timestamp;
                    }
                }
            }
        }
        
        self.operations.push(op);
        self.clock.tick();
        
        Ok(())
    }
    
    fn merge(&mut self, other: &CRDTDocument) {
        for op in &other.operations {
            if !self.has_operation(op) {
                let _ = self.apply_operation(op.clone());
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// WebSocket Sync
// ═══════════════════════════════════════════════════════════════

struct CollaborationClient {
    document: Arc<RwLock<CRDTDocument>>,
    websocket: WebSocketConnection,
    presence: PresenceState,
}

impl CollaborationClient {
    async fn send_operation(&self, op: Operation) -> Result<(), SyncError> {
        // Apply locally first
        {
            let mut doc = self.document.write().await;
            doc.apply_operation(op.clone())?;
        }
        
        // Send to server
        let message = SyncMessage::Operation(op);
        self.websocket.send(message).await?;
        
        Ok(())
    }
    
    async fn receive_loop(&self) {
        while let Some(message) = self.websocket.receive().await {
            match message {
                SyncMessage::Operation(op) => {
                    let mut doc = self.document.write().await;
                    let _ = doc.apply_operation(op);
                }
                SyncMessage::Presence(presence) => {
                    self.update_presence(presence).await;
                }
                SyncMessage::Sync(full_doc) => {
                    let mut doc = self.document.write().await;
                    doc.merge(&full_doc);
                }
            }
        }
    }
    
    async fn update_cursor(&self, position: Position) {
        let message = SyncMessage::Presence(PresenceUpdate {
            site_id: self.document.read().await.site_id,
            cursor: Some(position),
            selection: None,
            activity: Activity::Editing,
        });
        let _ = self.websocket.send(message).await;
    }
}

// ═══════════════════════════════════════════════════════════════
// Presence System
// ═══════════════════════════════════════════════════════════════

struct PresenceManager {
    collaborators: HashMap<SiteId, Collaborator>,
    update_callback: Box<dyn Fn(&HashMap<SiteId, Collaborator>)>,
}

impl PresenceManager {
    fn update(&mut self, site_id: SiteId, update: PresenceUpdate) {
        let collaborator = self.collaborators.entry(site_id).or_insert_with(|| {
            Collaborator::new(site_id)
        });
        
        if let Some(cursor) = update.cursor {
            collaborator.cursor = cursor;
        }
        if let Some(selection) = update.selection {
            collaborator.selection = selection;
        }
        collaborator.activity = update.activity;
        collaborator.last_seen = Instant::now();
        
        (self.update_callback)(&self.collaborators);
    }
    
    fn remove_inactive(&mut self, timeout: Duration) {
        let now = Instant::now();
        self.collaborators.retain(|_, c| {
            now.duration_since(c.last_seen) < timeout
        });
    }
}
```

## API Usage

```rust
// Connect to collaboration session
let session = CollaborationSession::join(
    "project-id",
    "user-token",
)?;

// Get real-time document
let document = session.document();

// Make changes (automatically synced)
document.add_component(Component::new("R1", "Resistor"))?;
document.move_component("R1", Position::new(100.0, 50.0))?;

// See other collaborators
for collaborator in session.collaborators() {
    println!("{} is at {:?}", collaborator.name, collaborator.cursor);
}

// Add comment
session.add_comment(Comment {
    location: Location::Component("U1"),
    text: "Should we use a different package?",
    mentions: vec!["@sarah"],
})?;

// Start review
let review = session.start_review(ReviewConfig {
    sections: vec!["power", "digital"],
    reviewers: vec!["mike@company.com"],
})?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+C` | Open collaboration panel |
| `Ctrl+/` | Add comment at cursor |
| `Ctrl+Shift+/` | View all comments |
| `Alt+P` | Toggle presence indicators |
| `Ctrl+Shift+S` | Share project |

## Related Topics

- [Project Structure](../core-architecture/project-structure-management.md) - Project organization
- [Undo/Redo & Versioning](./undo-redo-versioning.md) - Version history
- [CLI](./cli.md) - Command-line collaboration
