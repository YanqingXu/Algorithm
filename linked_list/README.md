# 链表算法模块

这个模块包含了各种链表相关的算法实现，支持多种编程语言。每个算法都有详细的解释和多语言实现。

## 算法分类

### 🔄 反转算法 (`reverse/`)
链表反转相关的算法，包括整个链表反转、部分反转、K个一组反转等。

**已实现算法：**
- **K个一组反转链表（带约束条件）** - 高级版本的K个一组反转，增加了阈值判断和分隔符插入
  - 难度：Hard
  - 时间复杂度：O(n)
  - 空间复杂度：O(1)
  - 支持语言：C++, Rust, Go, Python, TypeScript, Lua

**计划实现：**
- 反转整个链表 (LeetCode 206)
- 反转链表 II (LeetCode 92)
- 标准K个一组反转链表 (LeetCode 25)

### 🤝 合并算法 (`merge/`)
将两个或多个链表合并的算法。

**已实现算法：**
- 合并K个有序链表 (LeetCode 23)
  - 难度：Hard
  - 时间复杂度：O(N log K)
  - 空间复杂度：O(1) 额外空间（不含返回链表）
  - 支持语言：C++, Rust, Go, Python, TypeScript, Lua

**计划实现：**
- 合并两个有序链表 (LeetCode 21)

### 🔍 双指针技巧 (`two_pointers/`)
使用快慢指针、前后指针等技巧解决链表问题。

**计划实现：**
- 链表的中间节点 (LeetCode 876)
- 删除链表的倒数第N个节点 (LeetCode 19)
- 回文链表 (LeetCode 234)

### ⭕ 环检测算法 (`cycle_detection/`)
检测链表中是否存在环的算法。

**计划实现：**
- 环形链表 (LeetCode 141)
- 环形链表 II (LeetCode 142)

### ✏️ 插入删除算法 (`insert_delete/`)
在链表中插入或删除节点的算法。

**计划实现：**
- 删除链表中的节点 (LeetCode 203)
- 删除排序链表中的重复元素 (LeetCode 83)

## 支持的编程语言

每个算法都提供以下语言的实现：

| 语言 | 目录 | 编译/运行方式 |
|------|------|--------------|
| C++ | `cpp/` | `g++ -std=c++17 -o output file.cpp` |
| Rust | `rust/` | `cargo run` |
| Go | `go/` | `go run main.go` |
| Python | `python/` | `python file.py` |
| TypeScript | `typescript/` | `npx ts-node file.ts` |
| Lua | `lua/` | `lua file.lua` |

## 快速开始

### 运行示例（K个一组反转链表）

**C++:**
```bash
cd linked_list/reverse/cpp
g++ -std=c++17 -o reverse_k_group reverse_k_group_with_constraints.cpp
./reverse_k_group
```

**Rust:**
```bash
cd linked_list/reverse/rust
cargo run
```

**Go:**
```bash
cd linked_list/reverse/go
go run main.go
```

## 学习建议

1. **从基础开始**：建议先学习基本的遍历和插入删除操作
2. **掌握核心技巧**：重点理解快慢指针、虚拟头节点等常用技巧
3. **跨语言对比**：通过不同语言的实现加深理解
4. **循序渐进**：从简单问题开始，逐步挑战复杂算法

## 复杂度参考

| 操作类型 | 时间复杂度 | 空间复杂度 | 备注 |
|----------|------------|------------|------|
| 遍历 | O(n) | O(1) | 基础操作 |
| 查找 | O(n) | O(1) | 需要遍历 |
| 插入/删除（已知位置） | O(1) | O(1) | 指针操作 |
| 反转 | O(n) | O(1) | 迭代实现 |
| 合并两个链表 | O(n+m) | O(1) | n,m为链表长度 |
| 环检测 | O(n) | O(1) | 快慢指针 |

## 相关资源

- [链表算法完全指南](../docs/linked_list_algorithms.md)
- [LeetCode链表专题](https://leetcode.com/tag/linked-list/)
- [算法可视化演示](https://visualgo.net/en/list)
