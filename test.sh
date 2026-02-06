#!/bin/bash

# 设置颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

function log_header() {
    echo -e "\n${BLUE}================================================================${NC}"
    echo -e "${BLUE}# $1${NC}"
    echo -e "${BLUE}================================================================${NC}"
}

function log_info() {
    echo -e "${GREEN}[INFO] $1${NC}"
}

function log_error() {
    echo -e "${RED}[ERROR] $1${NC}"
}

# 1. 环境准备
log_header "1. 环境准备"
log_info "准备测试环境..."
TEST_ROOT="$(pwd)/.test_env_$(date +%s)"
export WORKSPACE_ROOT="$TEST_ROOT/workspaces"
MOCK_REPOS="$TEST_ROOT/repos"

# 清理旧环境
rm -rf "$TEST_ROOT"
mkdir -p "$MOCK_REPOS"
mkdir -p "$WORKSPACE_ROOT"

# 创建模拟仓库
mkdir -p "$MOCK_REPOS/repo1"
mkdir -p "$MOCK_REPOS/repo2"
mkdir -p "$MOCK_REPOS/repo3"
touch "$MOCK_REPOS/repo1/file1"
touch "$MOCK_REPOS/repo2/file2"
touch "$MOCK_REPOS/repo3/file3"

# 编译项目
log_info "编译项目..."
cargo build --quiet
CLI_BIN="./target/debug/workspace-cli"

if [ ! -f "$CLI_BIN" ]; then
    log_error "编译失败，找不到二进制文件"
    exit 1
fi

function run_cmd() {
    echo -e "${GREEN}> $1${NC}"
    eval "$1"
}

function cleanup() {
    log_header "10. 清理测试环境"
    rm -rf "$TEST_ROOT"
    log_info "测试环境已清理"
}
trap cleanup EXIT

# 2. 测试 Create 命令
log_header "2. 测试 Create 命令"
# 创建名为 'backend' 的工作区，包含 repo1 和 repo2
run_cmd "$CLI_BIN create -n backend -r $MOCK_REPOS/repo1 $MOCK_REPOS/repo2"

if [ -d "$WORKSPACE_ROOT/backend" ] && [ -L "$WORKSPACE_ROOT/backend/repo1" ] && [ -L "$WORKSPACE_ROOT/backend/repo2" ]; then
    log_info "Create 'backend' 成功"
else
    log_error "Create 'backend' 失败"
    exit 1
fi

# 创建名为 'frontend' 的空工作区
run_cmd "$CLI_BIN create -n frontend"

if [ -d "$WORKSPACE_ROOT/frontend" ]; then
    log_info "Create 'frontend' 成功"
else
    log_error "Create 'frontend' 失败"
    exit 1
fi

# 3. 测试 List 命令
log_header "3. 测试 List 命令"
echo "--- List Output ---"
run_cmd "$CLI_BIN list"
echo "-------------------"
echo "--- List Detail Output ---"
run_cmd "$CLI_BIN list --detail"
echo "--------------------------"

# 4. 测试 Update 命令
log_header "4. 测试 Update 命令"

# 向 'frontend' 添加 repo3
run_cmd "$CLI_BIN update -n frontend --add $MOCK_REPOS/repo3"
if [ -L "$WORKSPACE_ROOT/frontend/repo3" ]; then
    log_info "Update: 添加 repo3 到 frontend 成功"
else
    log_error "Update: 添加 repo3 到 frontend 失败"
    exit 1
fi

# 从 'backend' 移除 repo1
run_cmd "$CLI_BIN update -n backend --remove repo1"
if [ ! -L "$WORKSPACE_ROOT/backend/repo1" ]; then
    log_info "Update: 从 backend 移除 repo1 成功"
else
    log_error "Update: 从 backend 移除 repo1 失败"
    ls -l "$WORKSPACE_ROOT/backend"
    exit 1
fi

# 5. 测试 Activate 命令 (直接激活)
log_header "5. 测试 Activate 命令 (直接激活)"
# 使用 exit 命令立即退出子 shell，验证命令是否成功执行
echo "exit" | $CLI_BIN activate frontend
if [ $? -eq 0 ]; then
    log_info "Activate 'frontend' 直接激活成功"
else
    log_error "Activate 'frontend' 直接激活失败"
    exit 1
fi

# 6. 测试 Remove 命令 (安全性验证)
log_header "6. 测试 Remove 命令 (安全性验证)"
# 确保源文件存在
if [ ! -d "$MOCK_REPOS/repo1" ] || [ ! -d "$MOCK_REPOS/repo2" ]; then
     log_error "测试前源 repo 不存在，环境异常"
     exit 1
fi

run_cmd "$CLI_BIN remove -n backend"

if [ ! -d "$WORKSPACE_ROOT/backend" ]; then
    log_info "Remove 'backend' 工作区目录删除成功"
else
    log_error "Remove 'backend' 失败，目录仍存在"
    exit 1
fi

# 关键：验证源 repo 是否依然存在
if [ -d "$MOCK_REPOS/repo1" ] && [ -d "$MOCK_REPOS/repo2" ]; then
    log_info "源 repo 安全性验证通过 (repo1, repo2 仍存在)"
else
    log_error "严重错误：源 repo 被误删除！"
    exit 1
fi

# 7. 最终检查
log_header "7. 最终状态检查"
run_cmd "$CLI_BIN list --detail"

# 8. 测试位置参数 (Positional Arguments)
log_header "8. 测试位置参数 (Positional Arguments)"

# Create without -n
run_cmd "$CLI_BIN create ws_pos_1"
if [ -d "$WORKSPACE_ROOT/ws_pos_1" ]; then
    log_info "Create ws_pos_1 (位置参数) 成功"
else
    log_error "Create ws_pos_1 (位置参数) 失败"
    exit 1
fi

# Update without -n
run_cmd "$CLI_BIN update ws_pos_1 --add $MOCK_REPOS/repo1"
if [ -L "$WORKSPACE_ROOT/ws_pos_1/repo1" ]; then
    log_info "Update ws_pos_1 (位置参数) 成功"
else
    log_error "Update ws_pos_1 (位置参数) 失败"
    exit 1
fi

# Remove without -n
run_cmd "$CLI_BIN remove ws_pos_1"
if [ ! -d "$WORKSPACE_ROOT/ws_pos_1" ]; then
    log_info "Remove ws_pos_1 (位置参数) 成功"
else
    log_error "Remove ws_pos_1 (位置参数) 失败"
    exit 1
fi

log_header "9. 测试总结"
log_info "所有测试通过！"
