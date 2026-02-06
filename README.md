# workspace-cli

[中文 README](README.zh.md)

Manage multiple repos as a virtual workspace via symlinks, streamline multi-repo dev workflows easily.

## Implementation Details

### Tech Stack
- **Language**: Rust
- **CLI Framework**: `clap`
- **TUI Framework**: `ratatui` + `crossterm`
- **Error Handling**: `anyhow`

### Storage Layout
- Default location: `~/.workspaces/<name>/`
- **No central registry file**; the filesystem directory structure is the source of truth.
- Symlinks are created directly under each workspace directory.

```
  └── <workspace_name>/   # Workspace directory, holds symlinks
      └── <link_name> -> /path/to/repo
```

## Installation

### Install via Cargo (Recommended)

```bash
cargo install workspace-cli
```

### Build from Source

```bash
# Option 1: use cargo directly
# cargo build --release

# Option 2: use the build script wrapper
./build.sh
# Binary located at ./target/release/workspace-cli
# Optional: install to a directory on your PATH, e.g. /usr/local/bin
sudo cp ./target/release/workspace-cli /usr/local/bin/workspace
# You can now use the `workspace` command anywhere
```

## Usage

### Create a Workspace
```bash
workspace create my-project -r ~/repos/backend ~/repos/frontend
# Or use the -n flag
workspace create -n my-project ...
```
**Sample output:**
```text
Created workspace directory: "/Users/user/.workspaces/my-project"
Linked "/Users/user/.workspaces/my-project/backend" -> "/Users/user/repos/backend"
Linked "/Users/user/.workspaces/my-project/frontend" -> "/Users/user/repos/frontend"
```

### List Workspaces
```bash
workspace list
workspace list --detail
```
**Sample output (default):**
```text
my-project
  backend; frontend
other-workspace
  api; web
```
**Sample output (detail):**
```text
my-project
  backend -> "/Users/user/repos/backend"
  frontend -> "/Users/user/repos/frontend"
other-workspace
  api -> "/Users/user/repos/other/api"
  web -> "/Users/user/repos/other/web"
```

### Update a Workspace

**Add a repo:**
```bash
workspace update my-project --add ~/repos/docs
```
**Sample output:**
```text
Linked "/Users/user/.workspaces/my-project/docs" -> "/Users/user/repos/docs"
```

**Remove a link:**
```bash
workspace update my-project --remove frontend
```
**Sample output:**
```text
Removed link: frontend
```

### Activate a Workspace
```bash
# Activate a specific workspace
workspace activate my-project
# or
# Interactive selection
workspace activate
```
**Interactive UI example:**
```text
┌ Select Workspace ──────────────┐
│>> my-project                   │
│   other-workspace              │
│                                │
└────────────────────────────────┘
Use ↑/↓ to move, Enter to select, q/Esc to quit
```
**After selection:**
```text
Activating workspace: my-project
Entering sub-shell at "/Users/user/.workspaces/workspaces/my-project"
# (You are now in a new shell; current directory is the workspace root)
```

### Remove a Workspace
```bash
workspace remove my-project
```
**Sample output:**
```text
Removed workspace: my-project
```

## Configuration
You can customize the storage location via environment variable:
- `WORKSPACE_ROOT`: workspace root directory (default: `~/.workspaces`).

```bash
export WORKSPACE_ROOT="/path/to/my/workspaces"
```
