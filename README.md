# Multi-Language Algorithms Learning Repository

Welcome to the Multi-Language Algorithms Learning Repository! This project is designed to help developers learn and practice algorithms across multiple programming languages, providing a comprehensive platform for algorithm implementation and study.

## Purpose

This repository serves as a centralized hub for:
- Learning fundamental algorithms and data structures
- Comparing algorithm implementations across different programming languages
- Practicing coding skills in multiple languages
- Building a reference library of algorithm solutions

## Supported Programming Languages

This repository supports the following programming languages:

- **C++** (`cpp/`) - High-performance systems programming
- **Rust** (`rust/`) - Memory-safe systems programming
- **Go** (`go/`) - Concurrent and scalable applications
- **Lua** (`lua/`) - Lightweight scripting language
- **TypeScript** (`typescript/`) - Type-safe JavaScript development
- **Python** (`python/`) - Readable and versatile programming

## Getting Started

### C++
```bash
cd cpp/
# Compile with g++ or clang++
g++ -std=c++17 -o algorithm algorithm.cpp
./algorithm
```

### Rust
```bash
cd rust/
# Initialize a new Rust project (if needed)
cargo init .
# Run Rust code
cargo run
```

### Go
```bash
cd go/
# Initialize Go module (if needed)
go mod init algorithms
# Run Go code
go run main.go
```

### Lua
```bash
cd lua/
# Run Lua script
lua algorithm.lua
```

### TypeScript
```bash
cd typescript/
# Install dependencies (if needed)
npm install
# Compile and run TypeScript
npx tsc algorithm.ts
node algorithm.js
```

### Python
```bash
cd python/
# Run Python script
python algorithm.py
# or
python3 algorithm.py
```

## Repository Structure

This repository adopts an **algorithm-centric organization structure**, facilitating cross-language learning and comparison:

```
Algorithm/
├── linked_list/           # Linked List Algorithms Module
│   ├── reverse/          # Reversal Algorithms
│   │   ├── cpp/          # C++ Implementation
│   │   ├── rust/         # Rust Implementation  
│   │   ├── go/           # Go Implementation
│   │   ├── python/       # Python Implementation
│   │   ├── typescript/   # TypeScript Implementation
│   │   ├── lua/          # Lua Implementation
│   │   └── README.md     # Reversal Algorithms Documentation
│   ├── merge/            # Merge Algorithms (Planned)
│   ├── two_pointers/     # Two Pointers Technique (Planned)
│   ├── cycle_detection/  # Cycle Detection Algorithms (Planned)
│   └── README.md         # Linked List Algorithms Overview
├── sorting/              # Sorting Algorithms Module (Planned)
├── graph/                # Graph Algorithms Module (Planned)
├── dynamic_programming/  # Dynamic Programming Module (Planned)
└── docs/                 # General Documentation and Learning Guides
    └── linked_list_algorithms.md
```

### Currently Implemented Algorithms

#### 🔄 Linked List Reversal Algorithms
- **Reverse Nodes in K-Group with Constraints** - Advanced version of K-group reversal
  - Supported Languages: C++, Rust, Go, Python, TypeScript, Lua
  - Difficulty: Hard
  - Features: Includes threshold checking and separator insertion functionality

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Implement algorithms in the appropriate language directory
2. Include clear comments and documentation
3. Add test cases when possible
4. Follow the coding conventions of each language
5. Update relevant README files

## Project File Management

To keep the project clean, this project uses `.gitignore` to ignore the following types of generated files:

### Automatically Ignored File Types
- **Compiled Artifacts**: `*.exe`, `*.out`, `*.o`, `*.a`, etc.
- **Build Directories**: 
  - TypeScript: `dist/`, `node_modules/`
  - Rust: `target/`, `Cargo.lock`
  - Python: `__pycache__/`, `*.pyc`
- **Dependency Lock Files**: `package-lock.json`, `yarn.lock`
- **IDE Temporary Files**: Editor caches and temporary files
- **System Files**: `.DS_Store`, `Thumbs.db`, etc.

### Preserved Configuration Files
- `.vscode/`: VS Code project configuration (debugging and build tasks)
- Language configuration files: `package.json`, `tsconfig.json`, `Cargo.toml`, etc.

This ensures that the repository contains only source code and necessary configuration files, maintaining code repository cleanliness.

## Learning Resources

- [Algorithm Visualizations](https://visualgo.net/)
- [LeetCode](https://leetcode.com/)
- [HackerRank](https://www.hackerrank.com/)
- [Codeforces](https://codeforces.com/)

## License

This project is open source and available under the [MIT License](LICENSE).

---

**Happy Coding and Learning! 🚀**

*Also available in: [中文版本](README_CN.md)*