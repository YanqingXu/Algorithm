# 链表反转算法

本目录包含各种链表反转相关的算法实现，支持多种编程语言。

## 算法列表

### 🔄 K个一组反转链表（带约束条件）
**文件名：** `reverse_k_group_with_constraints.*`  
**难度：** Hard  
**时间复杂度：** O(n)  
**空间复杂度：** O(1)

#### 问题描述
给你一个链表，每 k 个节点一组进行翻转，但有以下约束条件：
1. 只有当这k个节点的值的和大于等于给定阈值threshold时，才进行反转
2. 如果某一组的节点数不足k个，则不进行反转
3. 反转后需要将相邻的两个反转组之间插入一个值为separator的新节点
4. 最终返回修改后的链表头节点

#### 算法思路
1. 使用双指针技术遍历链表，每次处理k个节点
2. 计算当前k个节点的和，判断是否满足阈值条件
3. 如果满足条件，使用三指针法反转这k个节点
4. 在相邻的反转组之间插入分隔符节点
5. 继续处理下一组，直到链表结束

#### 示例
**输入:** head = [1,2,3,4,5,6], k = 3, threshold = 6, separator = 0  
**输出:** [3,2,1,0,6,5,4]  
**解释:** 
- 第一组[1,2,3]，和=6>=6，反转为[3,2,1]
- 第二组[4,5,6]，和=15>=6，反转为[6,5,4]  
- 在两组之间插入分隔符0

#### 支持语言
- **C++**: `cpp/reverse_k_group_with_constraints.cpp`
- **Rust**: `rust/src/main.rs` (需要 `cargo run`)
- **Go**: `go/main.go`
- **Python**: `python/reverse_k_group_with_constraints.py`
- **TypeScript**: `typescript/reverse_k_group_with_constraints.ts`
- **Lua**: `lua/reverse_k_group_with_constraints.lua`

## 快速开始

### C++
```bash
cd cpp
g++ -std=c++17 -o reverse_k_group reverse_k_group_with_constraints.cpp
./reverse_k_group
```

### Rust
```bash
cd rust
cargo run
```

### Go
```bash
cd go
go run main.go
```

### Python
```bash
cd python
python reverse_k_group_with_constraints.py
```

### TypeScript
```bash
cd typescript
npx ts-node reverse_k_group_with_constraints.ts
```

### Lua
```bash
cd lua
lua reverse_k_group_with_constraints.lua
```

## 核心技巧

### 1. 虚拟头节点
使用虚拟头节点简化边界情况的处理，避免特殊处理头节点的情况。

### 2. 三指针反转法
使用 `prev`, `current`, `next` 三个指针实现链表反转：
```cpp
ListNode* prev = nullptr;
ListNode* current = head;
while (current) {
    ListNode* next = current->next;
    current->next = prev;
    prev = current;
    current = next;
}
return prev;
```

### 3. 分组处理
先检查是否有足够的k个节点，再进行处理，避免不完整的组。

### 4. 条件判断
计算节点值的和，只有满足阈值条件才进行反转操作。

## 相关问题

这个算法是以下经典问题的组合和扩展：
- [LeetCode 25: K个一组反转链表](https://leetcode.com/problems/reverse-nodes-in-k-group/)
- [LeetCode 206: 反转链表](https://leetcode.com/problems/reverse-linked-list/)
- [LeetCode 92: 反转链表 II](https://leetcode.com/problems/reverse-linked-list-ii/)

## 复杂度分析

| 操作 | 时间复杂度 | 空间复杂度 | 说明 |
|------|------------|------------|------|
| 遍历检查 | O(n) | O(1) | 需要遍历每个节点一次 |
| 计算和值 | O(k) | O(1) | 每组k个节点 |
| 反转操作 | O(k) | O(1) | 每组k个节点 |
| 总体复杂度 | O(n) | O(1) | n为链表长度 |

## 测试用例

1. **基本用例**: [1,2,3,4,5,6], k=3, threshold=6, separator=0
2. **阈值不满足**: [1,1,1,2,2,2], k=3, threshold=5, separator=9  
3. **边界情况**: [5], k=1, threshold=3, separator=0
4. **负数情况**: [-1,-2,-3,4,5,6], k=3, threshold=-5, separator=0
5. **空链表**: null, k=2, threshold=5, separator=0
