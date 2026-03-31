# opencode-multi

[![Crates.io](https://img.shields.io/crates/v/opencode-multi.svg)](https://crates.io/crates/opencode-multi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A Rust CLI tool for managing multiple OpenCode profiles through isolated config and data directories.

## Overview

`opencode-multi` enables **multi-profile support for OpenCode** by managing separate config and data environments. Each profile gets:

- Isolated configuration directory (`~/.config/opencode-multi/profiles/{name}/`)
- Isolated data directory (`~/.local/share/opencode-multi/profiles/{name}/`)
- Separate authentication state (separate `auth.json` per profile)

This allows you to:
- Use different OpenCode accounts/providers per profile
- Maintain separate plugin configurations
- Keep work and personal environments completely isolated

## Installation

### From crates.io (recommended)

```bash
cargo install opencode-multi
```

### From source

```bash
git clone https://github.com/dominic-codespoti/opencode-multi.git
cd opencode-multi
cargo install --path .
```

## Quick Start

```bash
# Create two profiles
opencode-multi create work
opencode-multi create personal

# List profiles
opencode-multi list

# Run OpenCode with a profile
opencode-multi run work

# In another terminal, switch to personal profile
opencode-multi run personal
```

## Commands

### `create <name>`
Create a new profile with scaffolded directory structure.

```bash
opencode-multi create my-profile
```

Creates:
- `~/.config/opencode-multi/profiles/{name}/`
- `~/.config/opencode-multi/profiles/{name}/plugins/`
- `~/.config/opencode-multi/profiles/{name}/commands/`
- `~/.config/opencode-multi/profiles/{name}/agents/`
- `~/.config/opencode-multi/profiles/{name}/modes/`
- `~/.local/share/opencode-multi/profiles/{name}/`

### `list`
Display all profiles with status information.

```bash
opencode-multi list
```

Output:
```
NAME       CONFIG   AUTH   STATUS
work       yes      yes    healthy
personal   yes      no     needs-auth
```

### `show <name>`
Display detailed information about a profile.

```bash
opencode-multi show work
```

### `run <name> [-- <opencode-args>]`
Launch OpenCode with the specified profile.

```bash
# Basic usage
opencode-multi run work

# With additional OpenCode arguments
opencode-multi run work -- --model openai/gpt-5
```

### `clone <source> <destination>`
Copy an existing profile to a new profile.

```bash
opencode-multi clone work work-backup
```

### `remove <name> [--yes]`
Delete a profile and all its data.

```bash
# With confirmation prompt
opencode-multi remove old-profile

# Skip confirmation
opencode-multi remove old-profile --yes
```

### `doctor`
Check system health and profile status.

```bash
opencode-multi doctor
```

Output:
```
[ok] opencode found at /usr/local/bin/opencode
[ok] Config root exists
[ok] Data root exists
[ok] Profile 'work' healthy
[warn] Profile 'personal' needs authentication
```

## How It Works

OpenCode reads configuration from a config directory and stores runtime data (including `auth.json`) in a data directory. `opencode-multi` works by:

1. Managing separate config/data directories per profile
2. Setting environment variables when launching OpenCode:
   - `OPENCODE_CONFIG_DIR` → profile's config directory
   - `XDG_DATA_HOME` → profile's data parent directory
   - `OPENCODE_PROFILE` → profile name (for reference)

This approach requires **no modifications to OpenCode itself**.

## Directory Structure

```
~/.config/opencode-multi/
└── profiles/
    ├── work/
    │   ├── opencode.json
    │   ├── plugins/
    │   ├── commands/
    │   ├── agents/
    │   └── modes/
    └── personal/
        ├── opencode.json
        ├── plugins/
        ├── commands/
        ├── agents/
        └── modes/

~/.local/share/opencode-multi/
└── profiles/
    ├── work/
    │   └── opencode/
    │       └── auth.json
    └── personal/
        └── opencode/
            └── auth.json
```

## Profile Lifecycle Example

```bash
# 1. Create work profile
opencode-multi create work

# 2. Run OpenCode and authenticate with work account
opencode-multi run work
# Inside OpenCode: /connect (authenticate with work provider)

# 3. Create personal profile
opencode-multi create personal

# 4. Run OpenCode and authenticate with personal account
opencode-multi run personal
# Inside OpenCode: /connect (authenticate with personal provider)

# 5. Switch between profiles anytime
opencode-multi run work      # Uses work auth
opencode-multi run personal  # Uses personal auth
```

## Requirements

- Rust 1.70+ (for building from source)
- OpenCode installed and available in PATH

## License

MIT © Dominic Codespoti
