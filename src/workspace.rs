use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

const WORKSPACE_DIR: &str = ".workspaces";

pub struct WorkspaceManager {
    root_dir: PathBuf,
}

impl WorkspaceManager {
    fn validate_name(&self, name: &str, label: &str) -> Result<()> {
        if name.is_empty() {
            anyhow::bail!("{} name cannot be empty", label);
        }

        let mut components = Path::new(name).components();
        let first = components.next();
        let second = components.next();

        match (first, second) {
            (Some(std::path::Component::Normal(_)), None) => Ok(()),
            _ => anyhow::bail!(
                "Invalid {} name '{}'. Use a single path segment without separators or '..'",
                label,
                name
            ),
        }
    }

    pub fn new() -> Result<Self> {
        let root_dir = if let Ok(env_root) = std::env::var("WORKSPACE_ROOT") {
            PathBuf::from(env_root)
        } else {
            let home = dirs::home_dir().context("Could not find home directory")?;
            home.join(WORKSPACE_DIR)
        };
        Ok(Self { root_dir })
    }

    pub fn get_workspace_dir(&self, name: &str) -> PathBuf {
        self.root_dir.join(name)
    }

    pub fn create_workspace(&self, name: &str, repos: Option<Vec<PathBuf>>) -> Result<()> {
        self.validate_name(name, "Workspace")?;
        let ws_dir = self.get_workspace_dir(name);

        if ws_dir.exists() {
            anyhow::bail!("Workspace '{}' already exists at {:?}", name, ws_dir);
        }

        fs::create_dir_all(&ws_dir).context("Failed to create workspace directory")?;
        println!("Created workspace directory: {:?}", ws_dir);

        if let Some(repo_paths) = repos {
            for repo_path in repo_paths {
                self.add_link(name, &repo_path)?;
            }
        }

        Ok(())
    }

    pub fn list_workspaces(&self) -> Result<Vec<String>> {
        let workspaces_dir = &self.root_dir;
        if !workspaces_dir.exists() {
            return Ok(Vec::new());
        }

        let mut workspaces = Vec::new();
        for entry in fs::read_dir(workspaces_dir)? {
            let entry = entry?;
            // Filter out hidden files (starting with dot) and ensure it's a directory
            if let Some(name) = entry.file_name().to_str() {
                if !name.starts_with('.') && entry.file_type()?.is_dir() {
                     workspaces.push(name.to_string());
                }
            }
        }
        workspaces.sort();
        Ok(workspaces)
    }

    pub fn list_links(&self, name: &str) -> Result<Vec<(String, PathBuf)>> {
        self.validate_name(name, "Workspace")?;
        let ws_dir = self.get_workspace_dir(name);
        if !ws_dir.exists() {
            anyhow::bail!("Workspace '{}' does not exist", name);
        }

        let mut links = Vec::new();
        for entry in fs::read_dir(ws_dir)? {
            let entry = entry?;
            let path = entry.path();
            // Check if it's a symlink
            if path.is_symlink() {
                let target = fs::read_link(&path)?;
                if let Some(link_name) = path.file_name().and_then(|n| n.to_str()) {
                    links.push((link_name.to_string(), target));
                }
            }
        }
        links.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(links)
    }

    pub fn remove_workspace(&self, name: &str) -> Result<()> {
        self.validate_name(name, "Workspace")?;
        let ws_dir = self.get_workspace_dir(name);
        if !ws_dir.exists() {
            anyhow::bail!("Workspace '{}' does not exist", name);
        }
        fs::remove_dir_all(&ws_dir).context("Failed to remove workspace directory")?;
        println!("Removed workspace: {}", name);
        Ok(())
    }

    pub fn remove_link(&self, workspace_name: &str, link_name: &str) -> Result<()> {
        self.validate_name(workspace_name, "Workspace")?;
        self.validate_name(link_name, "Link")?;
        let ws_dir = self.get_workspace_dir(workspace_name);
        let link_path = ws_dir.join(link_name);

        let meta = match fs::symlink_metadata(&link_path) {
            Ok(meta) => meta,
            Err(_) => {
                // Try to see if user provided a full path, maybe we can resolve it to a link name?
                // For now, strict matching by name is safer and simpler.
                anyhow::bail!("Link '{}' does not exist in workspace '{}'", link_name, workspace_name);
            }
        };

        if !meta.file_type().is_symlink() {
            anyhow::bail!("'{}' is not a symlink in workspace '{}'", link_name, workspace_name);
        }

        fs::remove_file(&link_path).context("Failed to remove symlink")?;
        println!("Removed link: {}", link_name);
        Ok(())
    }

    pub fn add_link(&self, workspace_name: &str, target_path: &Path) -> Result<()> {
        self.validate_name(workspace_name, "Workspace")?;
        let ws_dir = self.get_workspace_dir(workspace_name);
        if !ws_dir.exists() {
            anyhow::bail!("Workspace '{}' does not exist", workspace_name);
        }

        let abs_target = fs::canonicalize(target_path)
            .with_context(|| format!("Failed to resolve path: {:?}", target_path))?;

        let dir_name = abs_target
            .file_name()
            .context("Invalid path: no file name")?;
        
        let link_path = ws_dir.join(dir_name);

        if link_path.exists() {
            println!("Link {:?} already exists, skipping", link_path);
            return Ok(());
        }

        symlink(&abs_target, &link_path)
            .with_context(|| format!("Failed to create symlink at {:?}", link_path))?;

        println!("Linked {:?} -> {:?}", link_path, abs_target);
        Ok(())
    }
}
