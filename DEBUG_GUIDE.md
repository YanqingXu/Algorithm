# 调试配置指南

本文档介绍如何在VS Code中调试多语言算法项目。

## 📋 目录

- [环境要求](#环境要求)
- [调试配置说明](#调试配置说明)
- [使用方法](#使用方法)
- [故障排除](#故障排除)
- [扩展推荐](#扩展推荐)

## 🔧 环境要求

### 必需工具

| 语言 | 编译器/解释器 | 调试器 | 版本要求 |
|------|---------------|--------|----------|
| C++ | g++ 或 clang++ | gdb 或 lldb | C++17+ |
| Rust | rustc | lldb | 1.70+ |
| Go | go | delve | 1.19+ |
| Python | python | pdb | 3.8+ |
| TypeScript | tsc, ts-node | node | Node 16+ |
| Lua | lua | lua-debug | 5.3+ |

### 安装命令

#### Windows (使用 Chocolatey)
```powershell
# 安装基础工具
choco install mingw golang python nodejs lua

# 安装 Rust
choco install rustup.install

# 安装 TypeScript
npm install -g typescript ts-node
```

#### Linux (Ubuntu/Debian)
```bash
# 安装基础工具
sudo apt update
sudo apt install build-essential gdb golang-go python3 python3-pip nodejs npm lua5.3

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 TypeScript
npm install -g typescript ts-node
```

## ⚙️ 调试配置说明

### 1. C++ 调试配置

**配置名称**: `C++ Debug`

- **编译器**: g++ (默认) 或 clang++
- **标准**: C++17
- **调试器**: gdb
- **预构建任务**: 自动编译当前文件

**使用步骤**:
1. 打开 `.cpp` 文件
2. 设置断点
3. 按 `F5` 或选择 "C++ Debug" 配置
4. 程序会自动编译并开始调试

### 2. Rust 调试配置

**配置名称**: `Rust Debug`

- **构建工具**: cargo
- **调试器**: lldb
- **预构建任务**: `cargo build`

**使用步骤**:
1. 确保在 Rust 项目目录中
2. 设置断点
3. 选择 "Rust Debug" 配置
4. 程序会自动构建并开始调试

### 3. Go 调试配置

**配置名称**: `Go Debug`

- **调试器**: delve (内置)
- **模式**: debug

**使用步骤**:
1. 打开 Go 项目
2. 设置断点
3. 选择 "Go Debug" 配置
4. 开始调试

### 4. Python 调试配置

**配置名称**: `Python Debug`

- **解释器**: python
- **调试器**: debugpy (内置)
- **控制台**: 集成终端

**使用步骤**:
1. 打开 `.py` 文件
2. 设置断点
3. 选择 "Python Debug" 配置
4. 开始调试

### 5. TypeScript 调试配置

提供两种调试方式：

#### 方式一: 编译后调试
**配置名称**: `TypeScript Debug`
- 先编译为 JavaScript，再调试
- 适合生产环境调试

#### 方式二: 直接调试 (推荐)
**配置名称**: `TypeScript Debug (ts-node)`
- 使用 ts-node 直接运行 TypeScript
- 适合开发环境调试

### 6. Lua 调试配置

**配置名称**: `Lua Debug`

- **解释器**: lua
- **调试器**: lua-local

**注意**: 需要安装 Lua Debug 扩展

## 🚀 使用方法

### 快速开始

1. **安装推荐扩展**:
   - 打开 VS Code
   - 按 `Ctrl+Shift+P`
   - 输入 "Extensions: Show Recommended Extensions"
   - 安装所有推荐扩展

2. **选择调试配置**:
   - 按 `F5` 或点击调试面板
   - 在下拉菜单中选择对应语言的调试配置

3. **设置断点**:
   - 在代码行号左侧点击设置断点
   - 红色圆点表示断点已设置

4. **开始调试**:
   - 按 `F5` 开始调试
   - 使用调试控制按钮控制执行流程

### 调试快捷键

| 功能 | 快捷键 | 说明 |
|------|--------|------|
| 开始调试 | `F5` | 启动调试会话 |
| 停止调试 | `Shift+F5` | 停止当前调试会话 |
| 重启调试 | `Ctrl+Shift+F5` | 重新启动调试会话 |
| 继续执行 | `F5` | 继续执行到下一个断点 |
| 单步跳过 | `F10` | 执行当前行，不进入函数 |
| 单步进入 | `F11` | 进入函数内部 |
| 单步跳出 | `Shift+F11` | 跳出当前函数 |
| 切换断点 | `F9` | 在当前行设置/取消断点 |

### 调试面板功能

1. **变量面板**: 查看当前作用域的变量值
2. **监视面板**: 添加自定义表达式监视
3. **调用堆栈**: 查看函数调用链
4. **断点面板**: 管理所有断点

## 🔧 故障排除

### 常见问题

#### 1. C++ 编译失败

**问题**: 找不到编译器
```
'g++' is not recognized as an internal or external command
```

**解决方案**:
- 安装 MinGW-w64 或 MSYS2
- 将编译器路径添加到系统 PATH
- 在 VS Code 设置中指定编译器路径

#### 2. Rust 调试器无法启动

**问题**: LLDB 调试器未找到

**解决方案**:
- 安装 LLDB: `rustup component add lldb-preview`
- 安装 CodeLLDB 扩展
- 检查 Rust 工具链是否完整

#### 3. Go 调试失败

**问题**: Delve 调试器问题

**解决方案**:
```bash
# 安装/更新 delve
go install github.com/go-delve/delve/cmd/dlv@latest

# 检查 Go 模块
go mod tidy
```

#### 4. Python 调试器无响应

**问题**: Python 扩展未正确配置

**解决方案**:
- 确保安装了 Python 扩展
- 选择正确的 Python 解释器
- 检查 Python 路径配置

#### 5. TypeScript 调试源码映射问题

**问题**: 断点位置不准确

**解决方案**:
- 确保 `tsconfig.json` 中启用了 `sourceMap`
- 使用 ts-node 调试配置
- 检查文件路径映射

### 性能优化

1. **禁用不必要的扩展**: 只启用当前使用语言的扩展
2. **调整调试设置**: 根据项目大小调整调试器配置
3. **使用条件断点**: 避免在循环中设置无条件断点

## 📦 扩展推荐

### 必装扩展

| 扩展名 | 功能 | 支持语言 |
|--------|------|----------|
| C/C++ | C++ 开发支持 | C, C++ |
| Python | Python 开发支持 | Python |
| Go | Go 开发支持 | Go |
| rust-analyzer | Rust 开发支持 | Rust |
| Lua | Lua 开发支持 | Lua |
| TypeScript | TypeScript 支持 | TypeScript |

### 推荐扩展

| 扩展名 | 功能 | 用途 |
|--------|------|------|
| GitLens | Git 增强 | 版本控制 |
| Code Runner | 快速运行代码 | 测试代码片段 |
| Bracket Pair Colorizer | 括号配对着色 | 代码可读性 |
| indent-rainbow | 缩进彩虹 | 代码结构 |
| TODO Tree | TODO 管理 | 任务跟踪 |

## 📝 配置文件说明

### `.vscode/launch.json`
调试启动配置，定义了各种语言的调试参数。

### `.vscode/tasks.json`
任务配置，定义了编译和构建任务。

### `.vscode/settings.json`
工作区设置，配置了各种语言的开发环境。

### `.vscode/extensions.json`
扩展推荐列表，自动提示安装相关扩展。

## 🎯 最佳实践

1. **合理设置断点**: 在关键逻辑处设置断点，避免过多断点影响性能
2. **使用条件断点**: 在循环中使用条件断点，只在特定条件下暂停
3. **监视关键变量**: 将重要变量添加到监视面板
4. **保存调试配置**: 为不同场景创建不同的调试配置
5. **定期更新工具**: 保持编译器、调试器和扩展的最新版本

## 📞 获取帮助

如果遇到问题，可以：

1. 查看 VS Code 官方文档
2. 检查相关语言的官方调试指南
3. 在项目 Issues 中提问
4. 参考社区解决方案

---

**注意**: 本配置适用于 Windows 环境，其他操作系统可能需要调整部分配置。