use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "workspace")]
#[command(about = "Manage local development workspaces", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new workspace
    Create {
        /// Name of the workspace (positional or via flag)
        #[arg(index = 1)]
        name_pos: Option<String>,

        /// Name of the workspace (flag)
        #[arg(short, long)]
        name: Option<String>,

        /// Initial repositories to add (paths)
        #[arg(short, long, num_args = 1..)]
        repos: Option<Vec<PathBuf>>,

        /// Output activation command immediately
        #[arg(short, long)]
        activate: bool,
    },

    /// List all workspaces
    List {
        /// Show detailed symlink targets
        #[arg(long)]
        detail: bool,
    },

    /// Update a workspace (add/remove repos)
    Update {
        /// Name of the workspace (positional or via flag)
        #[arg(index = 1)]
        name_pos: Option<String>,

        /// Name of the workspace (flag)
        #[arg(short, long)]
        name: Option<String>,

        /// Add repositories (paths)
        #[arg(long, num_args = 1..)]
        add: Option<Vec<PathBuf>>,

        /// Remove links (by name or path)
        #[arg(long, num_args = 1..)]
        remove: Option<Vec<String>>,
    },

    /// Remove a workspace
    Remove {
        /// Name of the workspace (positional or via flag)
        #[arg(index = 1)]
        name_pos: Option<String>,

        /// Name of the workspace (flag)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Activate a workspace and enter the workspace directory (interactive TUI or direct)
    Activate {
        /// Workspace name to activate directly
        #[arg(index = 1)]
        name: Option<String>,

        /// Start with details expanded (only for TUI mode)
        #[arg(long)]
        detail: bool,
    },
}
