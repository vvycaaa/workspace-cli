mod cli;
mod tui;
mod workspace;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use std::env;
use std::process::Command;
use workspace::WorkspaceManager;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manager = WorkspaceManager::new()?;

    match &cli.command {
        Commands::Create {
            name_pos,
            name,
            repos,
            activate,
        } => {
            let workspace_name = name_pos.as_ref().or(name.as_ref()).ok_or_else(|| {
                anyhow::anyhow!("Workspace name is required (provide as argument or via --name/-n)")
            })?;
            manager.create_workspace(workspace_name, repos.clone())?;
            if *activate {
                activate_workspace(&manager, workspace_name)?;
            }
        }
        Commands::List { detail } => {
            let workspaces = manager.list_workspaces()?;
            if workspaces.is_empty() {
                println!("No workspaces found.");
                return Ok(());
            }

            for ws in workspaces {
                println!("{}", ws.bold().green());
                if *detail {
                    match manager.list_links(&ws) {
                        Ok(links) => {
                            for (name, target) in links {
                                println!("  {} -> {:?}", name.cyan(), target);
                            }
                        }
                        Err(e) => eprintln!("  Error listing links: {}", e),
                    }
                } else {
                    match manager.list_links(&ws) {
                        Ok(links) => {
                            let names: Vec<String> = links.into_iter().map(|(n, _)| n).collect();
                            if !names.is_empty() {
                                println!("  {}", names.join("; "));
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
        Commands::Update { name_pos, name, add, remove } => {
            let workspace_name = name_pos.as_ref().or(name.as_ref()).ok_or_else(|| {
                anyhow::anyhow!("Workspace name is required (provide as argument or via --name/-n)")
            })?;
            if let Some(add_paths) = add {
                for path in add_paths {
                    manager.add_link(workspace_name, path)?;
                }
            }
            if let Some(remove_names) = remove {
                for link_name in remove_names {
                    manager.remove_link(workspace_name, link_name)?;
                }
            }
        }
        Commands::Remove { name_pos, name } => {
            let workspace_name = name_pos.as_ref().or(name.as_ref()).ok_or_else(|| {
                anyhow::anyhow!("Workspace name is required (provide as argument or via --name/-n)")
            })?;
            manager.remove_workspace(workspace_name)?;
        }
        Commands::Activate { name, detail } => {
            if let Some(ws_name) = name {
                // Direct activation
                activate_workspace(&manager, ws_name)?;
                return Ok(());
            }

            let workspaces = manager.list_workspaces()?;
            if workspaces.is_empty() {
                println!("No workspaces found.");
                return Ok(());
            }

            let mut items = Vec::new();
            for ws in workspaces {
                let display = if *detail {
                    match manager.list_links(&ws) {
                        Ok(links) => {
                             let link_strs: Vec<String> = links.iter()
                                .map(|(n, t)| format!("{} -> {:?}", n, t))
                                .collect();
                             format!("{} ({})", ws, link_strs.join(", "))
                        },
                        Err(_) => ws.clone()
                    }
                } else {
                    match manager.list_links(&ws) {
                         Ok(links) => {
                             let names: Vec<String> = links.into_iter().map(|(n, _)| n).collect();
                             if !names.is_empty() {
                                 format!("{} ({})", ws, names.join("; "))
                             } else {
                                 ws.clone()
                             }
                         },
                         Err(_) => ws.clone()
                    }
                };
                items.push((display, ws));
            }

            if let Some(selected) = tui::run_tui(items)? {
                activate_workspace(&manager, &selected)?;
            }
        }
    }

    Ok(())
}

fn activate_workspace(manager: &WorkspaceManager, name: &str) -> Result<()> {
    let ws_path = manager.get_workspace_dir(name);
    if !ws_path.exists() {
        anyhow::bail!("Workspace path does not exist: {:?}", ws_path);
    }

    println!("Activating workspace: {}", name.green());
    println!("Entering sub-shell at {:?}", ws_path);

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    
    let status = Command::new(shell)
        .current_dir(&ws_path)
        .status()
        .context("Failed to start shell")?;

    if !status.success() {
        eprintln!("Shell exited with non-zero status");
    }

    println!("Deactivated workspace: {}", name.yellow());
    Ok(())
}
