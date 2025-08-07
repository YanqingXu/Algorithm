# 多语言算法学习仓库

欢迎来到多语言算法学习仓库！这个项目旨在帮助开发者跨多种编程语言学习和练习算法，提供一个全面的算法实现和学习平台。

## 项目目的

本仓库作为一个集中化的学习中心，用于：
- 学习基础算法和数据结构
- 比较不同编程语言的算法实现
- 在多种语言中练习编程技能
- 构建算法解决方案的参考库

## 支持的编程语言

本仓库支持以下编程语言：

- **C++** (`cpp/`) - 高性能系统编程
- **Rust** (`rust/`) - 内存安全的系统编程
- **Go** (`go/`) - 并发和可扩展应用程序
- **Lua** (`lua/`) - 轻量级脚本语言
- **TypeScript** (`typescript/`) - 类型安全的JavaScript开发
- **Python** (`python/`) - 可读性强且多用途的编程语言

## 快速开始

### C++
```bash
cd cpp/
# 使用 g++ 或 clang++ 编译
g++ -std=c++17 -o algorithm algorithm.cpp
./algorithm
```

### Rust
```bash
cd rust/
# 初始化新的Rust项目（如需要）
cargo init .
# 运行Rust代码
cargo run
```

### Go
```bash
cd go/
# 初始化Go模块（如需要）
go mod init algorithms
# 运行Go代码
go run main.go
```

### Lua
```bash
cd lua/
# 运行Lua脚本
lua algorithm.lua
```

### TypeScript
```bash
cd typescript/
# 安装依赖（如需要）
npm install
# 编译并运行TypeScript
npx tsc algorithm.ts
node algorithm.js
```

### Python
```bash
cd python/
# 运行Python脚本
python algorithm.py
# 或者
python3 algorithm.py
```

## 仓库结构

本仓库采用**算法分类主导**的组织结构，便于跨语言学习和对比：

```
Algorithm/
├── linked_list/           # 链表算法模块
│   ├── reverse/          # 反转类算法
│   │   ├── cpp/          # C++实现
│   │   ├── rust/         # Rust实现  
│   │   ├── go/           # Go实现
│   │   ├── python/       # Python实现
│   │   ├── typescript/   # TypeScript实现
│   │   ├── lua/          # Lua实现
│   │   └── README.md     # 反转算法文档
│   ├── merge/            # 合并类算法（计划中）
│   ├── two_pointers/     # 双指针技巧（计划中）
│   ├── cycle_detection/  # 环检测算法（计划中）
│   └── README.md         # 链表算法总览
├── sorting/              # 排序算法模块（计划中）
├── graph/                # 图算法模块（计划中）
├── dynamic_programming/  # 动态规划模块（计划中）
└── docs/                 # 通用文档和学习指南
    └── linked_list_algorithms.md
```

### 当前已实现算法

#### 🔄 链表反转算法
- **K个一组反转链表（带约束条件）** - 高级版本的K个一组反转
  - 支持语言：C++, Rust, Go, Python, TypeScript, Lua
  - 难度：Hard
  - 特色：增加了阈值判断和分隔符插入功能

## 贡献指南

欢迎贡献！请遵循以下指导原则：

1. 在适当的语言目录中实现算法
2. 包含清晰的注释和文档
3. 尽可能添加测试用例
4. 遵循每种语言的编码约定
5. 更新相关的README文件

## 项目文件管理

为了保持项目整洁，本项目使用 `.gitignore` 来忽略以下类型的生成文件：

### 自动忽略的文件类型
- **编译产物**: `*.exe`, `*.out`, `*.o`, `*.a` 等
- **构建目录**: 
  - TypeScript: `dist/`, `node_modules/`
  - Rust: `target/`, `Cargo.lock`
  - Python: `__pycache__/`, `*.pyc`
- **依赖锁文件**: `package-lock.json`, `yarn.lock`
- **IDE临时文件**: 编辑器缓存和临时文件
- **系统文件**: `.DS_Store`, `Thumbs.db` 等

### 保留的配置文件
- `.vscode/`: VS Code 项目配置（调试和构建任务）
- 各语言的配置文件：`package.json`, `tsconfig.json`, `Cargo.toml` 等

这确保了仓库只包含源代码和必要的配置文件，保持代码库的整洁性。

## 学习资源

- [算法可视化](https://visualgo.net/)
- [力扣](https://leetcode-cn.com/)
- [HackerRank](https://www.hackerrank.com/)
- [Codeforces](https://codeforces.com/)

## 许可证

本项目是开源的，采用 [MIT 许可证](LICENSE)。

---

**祝您编程和学习愉快！🚀**

*其他语言版本：[English Version](README.md)*