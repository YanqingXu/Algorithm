# 🤝 合并算法模块（Merge）

本模块包含链表合并相关的算法实现，提供多语言版本与可运行示例。

说明/Notes：源码包含中英文双语注释，便于快速理解与对照学习。

## 已实现算法

- 合并 K 个有序链表（LeetCode 23）
  - 难度：Hard
  - 思路：分治法（两两合并，间隔倍增），总时间复杂度 O(N log K)，空间复杂度 O(1) 额外空间（不计结果链表）。
  - 支持语言：C++, Rust, Go, Python, TypeScript, Lua

### 题目描述
给定包含 K 个升序链表的数组，将所有链表合并为一个升序链表并返回其头节点。

### 核心方法
- mergeTwo(l1, l2): 合并两个有序链表，返回新链表头。
- mergeK(lists): 按 1,2,4,8... 的间隔两两合并，最终得到结果。

### 运行示例
进入对应语言目录，按下述方式运行：
- C++: 编译并运行 `merge_k_sorted_lists.cpp`
- Rust: `cargo run`
- Go: `go run main.go`
- Python: `python merge_k_sorted_lists.py`
- TypeScript: `npx ts-node merge_k_sorted_lists.ts`
- Lua: `lua merge_k_sorted_lists.lua`
