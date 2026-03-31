# opencode-multi Implementation Plan

## Overview

A Rust CLI tool for managing multiple OpenCode profiles through isolated config and data directories.

## Phase 1: Project Setup & Dependencies

### 1.1 Create Cargo Project
```bash
cargo init --name opencode-multi
```

### 1.2 Dependencies (Cargo.toml)
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
dirs = "5"
colored = "2"
tokio = { version = "1", features = ["full"] }
walkdir = "2"
```

## Phase 2: Core Architecture

### 2.1 Module Structure
```
src/
├── main.rs          # Entry point, CLI dispatch
├── cli.rs           # CLI argument definitions (clap derive)
├── config.rs        # Profile paths, directory resolution
├── profile.rs       # Profile struct and methods
├── commands/        # Individual command implementations
│   ├── mod.rs
│   ├── create.rs
│   ├── list.rs
│   ├── show.rs
│   ├── run.rs
│   ├── clone.rs
│   ├── remove.rs
│   └── doctor.rs
├── utils.rs         # File operations, validation
└── errors.rs        # Custom error types
```

### 2.2 Key Data Structures

**Profile struct:**
```rust
pub struct Profile {
    pub name: String,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}
```

**ProfileStatus enum:**
```rust
pub enum ProfileStatus {
    Healthy,      // Config exists, auth exists
    NeedsAuth,    // Config exists, no auth
    Missing,      // Profile doesn't exist
}
```

## Phase 3: Implementation Details

### 3.1 CLI Structure (clap derive)

```rust
#[derive(Parser)]
#[command(name = "opencode-multi")]
#[command(about = "Multi-profile manager for OpenCode")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { name: String },
    List,
    Show { name: String },
    Run { 
        name: String,
        #[arg(trailing_var_arg = true)]
        opencode_args: Vec<String>,
    },
    Clone { source: String, destination: String },
    Remove { 
        name: String,
        #[arg(long)]
        yes: bool,
    },
    Doctor,
}
```

### 3.2 Directory Resolution

**Config root:** `~/.config/opencode-multi/profiles/`
**Data root:** `~/.local/share/opencode-multi/profiles/`

Use `dirs::config_dir()` and `dirs::data_dir()` for cross-platform support.

### 3.3 Command Implementations

#### create <name>
1. Validate profile name (alphanumeric + hyphens only)
2. Check if profile already exists
3. Create directories:
   - `~/.config/opencode-multi/profiles/{name}/`
   - `~/.config/opencode-multi/profiles/{name}/plugins/`
   - `~/.config/opencode-multi/profiles/{name}/commands/`
   - `~/.config/opencode-multi/profiles/{name}/agents/`
   - `~/.config/opencode-multi/profiles/{name}/modes/`
   - `~/.local/share/opencode-multi/profiles/{name}/`
4. Write default `opencode.json` with schema reference

#### list
1. Scan config root for profile directories
2. For each profile, check:
   - Config directory exists
   - Data directory exists  
   - `auth.json` exists in data directory
3. Display table: NAME | CONFIG | AUTH | STATUS

#### show <name>
1. Validate profile exists
2. Display:
   - Config path
   - Data path
   - Auth path (with existence check)
   - Status (healthy/needs-auth)

#### run <name>
1. Validate profile exists
2. Set environment variables:
   - `OPENCODE_CONFIG_DIR=<config_path>`
   - `XDG_DATA_HOME=<data_parent>` (OpenCode uses this + /opencode/)
   - Optionally: `OPENCODE_PROFILE=<name>`
3. Spawn `opencode` process with:
   - Inherit stdin/stdout/stderr
   - Forward all trailing args
4. Wait for process completion

#### clone <src> <dst>
1. Validate source exists
2. Check destination doesn't exist
3. Copy config directory recursively
4. Copy data directory recursively
5. Display success message

#### remove <name>
1. Validate profile exists
2. If not `--yes`, prompt for confirmation
3. Delete config directory
4. Delete data directory
5. Display success message

#### doctor
1. Check `opencode` in PATH
2. Check config root exists
3. Check data root exists
4. For each profile:
   - Check config structure
   - Check auth.json presence
5. Print status with [ok]/[warn] prefixes

### 3.4 Error Handling

Use `anyhow` for error propagation with context:
```rust
use anyhow::{Context, Result};

fs::create_dir_all(&path)
    .with_context(|| format!("Failed to create directory: {:?}", path))?;
```

### 3.5 Output Formatting

Use `colored` crate for terminal colors:
- Green for success/ok
- Yellow for warnings
- Red for errors
- Bold for headers

## Phase 4: Testing Strategy

### 4.1 Unit Tests
- Profile name validation
- Path resolution
- Status detection logic

### 4.2 Integration Tests
- Full command workflows
- Profile lifecycle (create → use → remove)

## Phase 5: Distribution

### 5.1 Cargo.toml metadata
```toml
[package]
name = "opencode-multi"
version = "0.1.0"
edition = "2021"
description = "Multi-profile manager for OpenCode"
license = "MIT"
repository = "..."
```

### 5.2 Installation
```bash
cargo install --path .
```

## Phase 6: Documentation

### 6.1 README.md
- Installation instructions
- Quick start guide
- Command reference
- Examples

### 6.2 --help for all commands

## Implementation Order

1. Setup project + dependencies
2. Implement config.rs (directory resolution)
3. Implement profile.rs (Profile struct)
4. Implement create command
5. Implement list command
6. Implement show command
7. Implement run command (MVP feature)
8. Implement clone command
9. Implement remove command
10. Implement doctor command
11. Add colored output
12. Write tests
13. Write README

## Success Metrics

- [ ] All 7 core commands work
- [ ] Profile isolation verified (auth.json separate)
- [ ] Clean error messages
- [ ] Cross-platform directory support
- [ ] Tests pass
