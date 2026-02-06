# workspace-cli

[English README](README.md)
Workspace CLI 是一个用于管理本地开发环境的工作区命令行工具。引入了“工作区（Workspace）”的概念，通过软链接（Symbolic Link）将分散在各处的项目仓库聚合在一起，从而解决部分开发工具不支持多项目同时管理的问题，提升多仓库协同开发的效率。

## 实现细节

### 技术栈
- **语言**: Rust
- **CLI 框架**: `clap`
- **TUI 框架**: `ratatui` + `crossterm`
- **错误处理**: `anyhow`

### 存储结构
- 默认位置: `~/.workspaces/<name>/`
- **无中心注册表文件**；文件系统的目录结构即为数据源。
- 软链接（Symlinks）直接创建在每个工作区目录下。

```
  └── <workspace_name>/   # 工作区目录，存放软链接
      └── <link_name> -> /path/to/repo
```

## 安装

### Cargo 安装（推荐）

```bash
cargo install workspace-cli
```

### 源码编译

```bash
# 方式一：使用脚本包装的编译命令
./build.sh

# 方式二：直接使用 cargo
cargo build --release
# 二进制文件位于 ./target/release/workspace-cli
# 可选：安装到系统 PATH，例如 /usr/local/bin
sudo cp ./target/release/workspace-cli /usr/local/bin/workspace
# 现在可以在任何地方使用 `workspace` 命令
```

## 使用说明

### 创建工作区
```bash
workspace create my-project -r ~/repos/backend ~/repos/frontend
# 或者使用 -n flag
workspace create -n my-project ...
```
**输出示例：**
```text
Created workspace directory: "/Users/user/.workspaces/my-project"
Linked "/Users/user/.workspaces/my-project/backend" -> "/Users/user/repos/backend"
Linked "/Users/user/.workspaces/my-project/frontend" -> "/Users/user/repos/frontend"
```

### 列出工作区
```bash
workspace list
workspace list --detail
```
**输出示例 (默认)：**
```text
my-project
  backend; frontend
other-workspace
  api; web
```
**输出示例 (详细)：**
```text
my-project
  backend -> "/Users/user/repos/backend"
  frontend -> "/Users/user/repos/frontend"
other-workspace
  api -> "/Users/user/repos/other/api"
  web -> "/Users/user/repos/other/web"
```

### 更新工作区

**添加新项目：**
```bash
workspace update my-project --add ~/repos/docs
```
**输出示例：**
```text
Linked "/Users/user/.workspaces/my-project/docs" -> "/Users/user/repos/docs"
```

**移除项目链接：**
```bash
workspace update my-project --remove frontend
```
**输出示例：**
```text
Removed link: frontend
```

### 激活工作区
```bash
# 直接激活指定工作区
workspace activate my-project
# or
# 交互式选择
workspace activate
```
**交互界面示例：**
```text
┌ Select Workspace ──────────────┐
│>> my-project                   │
│   other-workspace              │
│                                │
└────────────────────────────────┘
Use ↑/↓ to move, Enter to select, q/Esc to quit
```
**选中后输出：**
```text
Activating workspace: my-project
Entering sub-shell at "/Users/user/.workspaces/workspaces/my-project"
# (此时你已进入新的 Shell 环境，当前目录为工作区根目录)
```

### 删除工作区
```bash
workspace remove my-project
```
**输出示例：**
```text
Removed workspace: my-project
```

## 配置
可以通过环境变量自定义存储位置：
- `WORKSPACE_ROOT`: 指定工作区根目录（默认为 `~/.workspaces`）。

```bash
export WORKSPACE_ROOT="/path/to/my/workspaces"
```
