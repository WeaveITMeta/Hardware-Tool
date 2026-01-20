# Undo/Redo & Versioning

## Overview

Hardware Tool implements a command-based history system for safe editing with unlimited undo/redo capability. Combined with Git-friendly file formats, this enables comprehensive version control and design history tracking.

## Command-Based History

### Architecture

```
┌─────────────────────────────────────────┐
│              Command Stack              │
├─────────────────────────────────────────┤
│  [Undo Stack]         [Redo Stack]      │
│                                         │
│  ┌─────────────┐     ┌─────────────┐   │
│  │ Move R1     │     │ Delete C5   │   │
│  ├─────────────┤     └─────────────┘   │
│  │ Add trace   │                        │
│  ├─────────────┤                        │
│  │ Route net   │                        │
│  ├─────────────┤                        │
│  │ Place U1    │                        │
│  └─────────────┘                        │
│        ▲                                │
│        │                                │
│   Current State                         │
└─────────────────────────────────────────┘
```

### Command Interface

```rust
pub trait Command {
    /// Execute the command
    fn execute(&mut self, context: &mut EditorContext) -> Result<()>;
    
    /// Undo the command
    fn undo(&mut self, context: &mut EditorContext) -> Result<()>;
    
    /// Command description for history display
    fn description(&self) -> String;
    
    /// Can this command be merged with previous?
    fn can_merge(&self, other: &dyn Command) -> bool;
}
```

### Example Commands

```rust
// Move component command
struct MoveComponentCommand {
    component_ref: String,
    old_position: Point,
    new_position: Point,
}

impl Command for MoveComponentCommand {
    fn execute(&mut self, ctx: &mut EditorContext) -> Result<()> {
        ctx.pcb.move_component(&self.component_ref, self.new_position)
    }
    
    fn undo(&mut self, ctx: &mut EditorContext) -> Result<()> {
        ctx.pcb.move_component(&self.component_ref, self.old_position)
    }
    
    fn description(&self) -> String {
        format!("Move {}", self.component_ref)
    }
    
    fn can_merge(&self, other: &dyn Command) -> bool {
        // Merge consecutive moves of same component
        if let Some(other_move) = other.downcast_ref::<MoveComponentCommand>() {
            return other_move.component_ref == self.component_ref;
        }
        false
    }
}
```

## Undo/Redo Operations

### Basic Operations

```rust
// Undo last action
editor.undo()?;

// Redo last undone action
editor.redo()?;

// Undo multiple steps
editor.undo_n(5)?;

// Undo to specific point
editor.undo_to(&checkpoint_id)?;
```

### Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Undo | `Ctrl+Z` |
| Redo | `Ctrl+Y` or `Ctrl+Shift+Z` |
| Undo 10 steps | `Ctrl+Alt+Z` |
| History panel | `Ctrl+H` |

### History Panel

```
History
═══════════════════════════════

  ● Current state
  │
  ├─ Route NET_CLK (2 min ago)
  │
  ├─ Move U1 (3 min ago)
  │
  ├─ Add via (5 min ago)
  │
  ├─ Place C1, C2, C3 (8 min ago)
  │
  ├─ Import netlist (15 min ago)
  │
  └─ Create board (20 min ago)

[Undo to selected] [Clear future]
```

## Command Merging

### Automatic Merging

Consecutive similar operations are merged:

```rust
// These become one undo step:
// - Move R1 to (10, 10)
// - Move R1 to (12, 10)
// - Move R1 to (15, 10)
// Result: Single "Move R1" command

MergeConfig {
    // Time window for merging
    merge_window_ms: 500,
    
    // Merge same-type commands
    merge_moves: true,
    merge_rotations: true,
    merge_property_edits: true,
}
```

### Compound Commands

Group multiple operations as single undo step:

```rust
editor.begin_compound("Place power section");

editor.place_component("U1", pos1);
editor.place_component("C1", pos2);
editor.place_component("C2", pos3);
editor.route_net("VCC");

editor.end_compound();

// Single undo reverts all four operations
```

## Checkpoints

### Creating Checkpoints

```rust
// Named checkpoint
let checkpoint = editor.create_checkpoint("Before routing");

// Auto-checkpoint on save
editor.save();  // Creates implicit checkpoint

// Milestone checkpoint
editor.create_milestone("v1.0 release candidate");
```

### Checkpoint Management

```rust
// List checkpoints
let checkpoints = editor.list_checkpoints();

// Restore checkpoint
editor.restore_checkpoint(&checkpoint_id)?;

// Compare with checkpoint
let diff = editor.diff_from_checkpoint(&checkpoint_id)?;
```

## File Versioning

### Git-Friendly Formats

Hardware Tool uses text-based formats optimized for version control:

```
project.hwt              # TOML - project config
schematic.hwt_sch        # JSON - schematic data
board.hwt_pcb            # JSON - PCB layout
```

### Diff-Friendly Structure

```json
{
  "components": [
    {
      "ref": "R1",
      "value": "10k",
      "position": { "x": 10.0, "y": 15.0 }
    }
  ],
  "traces": [
    {
      "net": "VCC",
      "points": [[10, 15], [20, 15], [20, 25]]
    }
  ]
}
```

### .gitignore Template

```gitignore
# Hardware Tool generated files
*.hwt_backup
*.hwt_autosave
/output/
/cache/

# Keep these in version control
!*.hwt
!*.hwt_sch
!*.hwt_pcb
!*.hwt_lib
```

## Auto-Save & Recovery

### Auto-Save Configuration

```rust
AutoSaveConfig {
    enabled: true,
    interval_seconds: 60,
    
    // Backup location
    backup_dir: ".hwt_backup",
    max_backups: 10,
    
    // Recovery
    keep_crash_recovery: true,
    
    // Integrity
    verify_on_save: true,
    create_checksum: true,
}
```

### Crash Recovery

Hardware Tool automatically detects crashes and offers recovery:

```
┌─────────────────────────────────────────┐
│         Recover Unsaved Work?           │
├─────────────────────────────────────────┤
│                                         │
│  Hardware Tool found unsaved changes    │
│  from a previous session:               │
│                                         │
│  File: my_board.hwt_pcb                 │
│  Last saved: 2026-01-19 16:30:00        │
│  Recovery: 2026-01-19 16:45:23          │
│                                         │
│  Changes since last save:               │
│  - 12 components moved                  │
│  - 5 traces added                       │
│  - 2 zones modified                     │
│                                         │
│  [Recover] [Discard] [Compare]          │
│                                         │
└─────────────────────────────────────────┘
```

### Recovery System Architecture

```rust
CrashRecovery {
    // Detection
    detection: CrashDetection {
        lock_file: true,           // Detect unclean shutdown
        heartbeat: true,           // Periodic heartbeat file
        heartbeat_interval: 5,     // seconds
    },
    
    // Recovery files
    recovery: RecoveryFiles {
        journal: ".hwt_journal",   // Transaction journal
        autosave: ".hwt_autosave", // Periodic snapshots
        wal: ".hwt_wal",           // Write-ahead log
    },
    
    // Behavior
    behavior: RecoveryBehavior {
        prompt_on_startup: true,
        auto_recover: false,       // Require user confirmation
        keep_recovery_files: true, // Until explicitly saved
    },
}
```

### Recovery Options

| Option | Description |
|--------|-------------|
| **Recover** | Restore all unsaved changes from recovery file |
| **Discard** | Ignore recovery, open last saved version |
| **Compare** | Side-by-side diff of saved vs. recovered |

### Compare Mode

```
┌─────────────────────────────────────────────────────────────────┐
│ Recovery Comparison                                             │
├────────────────────────────┬────────────────────────────────────┤
│ Last Saved (16:30:00)      │ Recovered (16:45:23)               │
├────────────────────────────┼────────────────────────────────────┤
│                            │                                    │
│  [PCB view - old state]    │  [PCB view - recovered state]      │
│                            │                                    │
│  Components: 45            │  Components: 45                    │
│  Traces: 120               │  Traces: 125 (+5)                  │
│  Vias: 34                  │  Vias: 36 (+2)                     │
│                            │                                    │
├────────────────────────────┴────────────────────────────────────┤
│ Changes:                                                        │
│   + 5 traces added (NET_DATA0..4)                              │
│   ~ 12 components moved                                         │
│   ~ 2 zones modified                                            │
├─────────────────────────────────────────────────────────────────┤
│ [Use Saved] [Use Recovered] [Merge Selected] [Cancel]           │
└─────────────────────────────────────────────────────────────────┘
```

### Project Integrity Checks

```rust
IntegrityCheck {
    // On open
    on_open: vec![
        Check::FileChecksum,       // Verify file integrity
        Check::SchemaVersion,      // Check format compatibility
        Check::ReferenceIntegrity, // Verify all refs exist
        Check::NetlistConsistency, // Schematic ↔ PCB match
    ],
    
    // On save
    on_save: vec![
        Check::WriteVerify,        // Read back and verify
        Check::BackupCreation,     // Ensure backup exists
    ],
    
    // Periodic
    periodic: PeriodicChecks {
        enabled: true,
        interval_minutes: 15,
        checks: vec![
            Check::MemoryIntegrity,
            Check::UndoStackHealth,
        ],
    },
}
```

### Error Handling

```rust
ErrorHandling {
    // File errors
    file_errors: FileErrorPolicy {
        retry_count: 3,
        retry_delay_ms: 100,
        fallback_to_backup: true,
    },
    
    // Corruption handling
    corruption: CorruptionPolicy {
        attempt_repair: true,
        quarantine_corrupt: true,
        notify_user: true,
    },
    
    // Logging
    logging: ErrorLogging {
        log_file: "hwt_errors.log",
        include_stack_trace: true,
        report_to_telemetry: false,  // Opt-in only
    },
}
```

### Recovery CLI

```bash
# Check for recovery files
hwt recover --check my_project.hwt

# List available recovery points
hwt recover --list my_project.hwt

# Recover to specific point
hwt recover --restore my_project.hwt --point 2026-01-19T16:45:23

# Export recovery as new file
hwt recover --export my_project.hwt --output recovered_project.hwt

# Verify project integrity
hwt verify my_project.hwt --full
```

## Branch & Merge (Experimental)

### Design Branches

```rust
// Create design branch
editor.create_branch("experiment_layout");

// Switch branches
editor.checkout_branch("main");

// List branches
let branches = editor.list_branches();

// Merge branch
editor.merge_branch("experiment_layout")?;
```

### Conflict Resolution

```
Merge Conflict: component R1
══════════════════════════════

Branch 'main':
  Position: (10.0, 15.0)
  Rotation: 0°

Branch 'experiment_layout':
  Position: (25.0, 30.0)
  Rotation: 90°

Resolution:
  ○ Keep main
  ○ Keep experiment_layout
  ● Manual (open editor)

[Resolve] [Skip] [Abort merge]
```

## History Export

### Export History

```rust
// Export command history
editor.export_history("history.json")?;

// Export as script (reproducible)
editor.export_as_script("build_board.rs")?;
```

### Replay History

```rust
// Replay from script
let script = Script::load("build_board.rs")?;
script.replay(&mut editor)?;
```

## Related Topics

- [Project Structure](../core-architecture/project-structure-management.md)
- [CLI Tools](./cli.md)
- [Programmatic Design](../core-architecture/programmatic-design.md)
