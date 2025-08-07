# 单链表算法完全指南

## 目录

1. [单链表基础](#单链表基础)
2. [算法分类概览](#算法分类概览)
3. [遍历算法](#遍历算法)
4. [插入删除算法](#插入删除算法)
5. [反转算法](#反转算法)
6. [合并算法](#合并算法)
7. [环检测算法](#环检测算法)
8. [双指针技巧](#双指针技巧)
9. [高级技巧](#高级技巧)
10. [总结与练习建议](#总结与练习建议)

---

## 单链表基础

### 链表节点定义

```cpp
// 单链表节点结构
struct ListNode {
    int val;
    ListNode* next;
    
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};
```

### 基本特性

- **动态大小**：可以在运行时动态增长或缩小
- **非连续存储**：节点在内存中不连续存储
- **单向访问**：只能从头节点开始顺序访问
- **插入删除高效**：O(1)时间复杂度（已知位置）

---

## 算法分类概览

| 分类 | 主要问题 | 核心技巧 | 时间复杂度 |
|------|----------|----------|------------|
| 遍历 | 访问所有节点 | 迭代/递归 | O(n) |
| 插入删除 | 修改链表结构 | 指针操作 | O(1)/O(n) |
| 反转 | 改变链表方向 | 三指针法 | O(n) |
| 合并 | 组合多个链表 | 归并思想 | O(n+m) |
| 环检测 | 检测循环结构 | 快慢指针 | O(n) |
| 双指针 | 特定位置操作 | 距离控制 | O(n) |

---

## 遍历算法

### 问题描述
遍历是链表最基本的操作，包括顺序访问所有节点、查找特定值、计算长度等。

### 核心技巧
- **迭代遍历**：使用while循环
- **递归遍历**：函数自调用
- **边界处理**：空链表和单节点情况

### 复杂度分析
- **时间复杂度**：O(n) - 需要访问每个节点
- **空间复杂度**：O(1) 迭代，O(n) 递归

### 经典例题

#### 例题1：计算链表长度

```cpp
// 迭代方法
int getLength(ListNode* head) {
    int length = 0;
    ListNode* current = head;
    
    while (current != nullptr) {
        length++;
        current = current->next;
    }
    
    return length;
}

// 递归方法
int getLengthRecursive(ListNode* head) {
    if (head == nullptr) {
        return 0;
    }
    return 1 + getLengthRecursive(head->next);
}
```

#### 例题2：查找链表中的最大值

```cpp
int findMax(ListNode* head) {
    if (head == nullptr) {
        throw std::invalid_argument("Empty list");
    }
    
    int maxVal = head->val;
    ListNode* current = head->next;
    
    while (current != nullptr) {
        maxVal = std::max(maxVal, current->val);
        current = current->next;
    }
    
    return maxVal;
}
```

#### 例题3：打印链表（正序和逆序）

```cpp
// 正序打印
void printList(ListNode* head) {
    ListNode* current = head;
    while (current != nullptr) {
        std::cout << current->val;
        if (current->next != nullptr) {
            std::cout << " -> ";
        }
        current = current->next;
    }
    std::cout << std::endl;
}

// 逆序打印（递归）
void printListReverse(ListNode* head) {
    if (head == nullptr) {
        return;
    }
    
    printListReverse(head->next);
    std::cout << head->val;
    if (head != nullptr) {
        std::cout << " <- ";
    }
}
```

---

## 插入删除算法

### 问题描述
在链表中插入或删除节点，包括头部、尾部、中间位置的操作。

### 核心技巧
- **虚拟头节点**：简化边界情况处理
- **前驱节点**：删除操作需要找到前一个节点
- **指针重连**：正确更新next指针

### 复杂度分析
- **时间复杂度**：O(1) 已知位置，O(n) 查找位置
- **空间复杂度**：O(1)

### 经典例题

#### 例题1：删除链表中的节点（LeetCode 203）

```cpp
// 删除所有值为val的节点
ListNode* removeElements(ListNode* head, int val) {
    // 创建虚拟头节点
    ListNode* dummy = new ListNode(0);
    dummy->next = head;
    
    ListNode* prev = dummy;
    ListNode* current = head;
    
    while (current != nullptr) {
        if (current->val == val) {
            prev->next = current->next;
            delete current;
            current = prev->next;
        } else {
            prev = current;
            current = current->next;
        }
    }
    
    ListNode* newHead = dummy->next;
    delete dummy;
    return newHead;
}
```

#### 例题2：在排序链表中插入节点

```cpp
ListNode* insertIntoSortedList(ListNode* head, int val) {
    ListNode* newNode = new ListNode(val);
    
    // 如果链表为空或新值小于头节点
    if (head == nullptr || val < head->val) {
        newNode->next = head;
        return newNode;
    }
    
    ListNode* current = head;
    
    // 找到插入位置
    while (current->next != nullptr && current->next->val < val) {
        current = current->next;
    }
    
    // 插入新节点
    newNode->next = current->next;
    current->next = newNode;
    
    return head;
}
```

#### 例题3：删除排序链表中的重复元素（LeetCode 83）

```cpp
ListNode* deleteDuplicates(ListNode* head) {
    if (head == nullptr) {
        return head;
    }
    
    ListNode* current = head;
    
    while (current->next != nullptr) {
        if (current->val == current->next->val) {
            ListNode* duplicate = current->next;
            current->next = current->next->next;
            delete duplicate;
        } else {
            current = current->next;
        }
    }
    
    return head;
}
```

---

## 反转算法

### 问题描述
改变链表的方向，使原来的尾节点变成头节点。

### 核心技巧
- **三指针法**：prev, current, next
- **递归反转**：分治思想
- **部分反转**：指定区间反转

### 复杂度分析
- **时间复杂度**：O(n)
- **空间复杂度**：O(1) 迭代，O(n) 递归

### 经典例题

#### 例题1：反转整个链表（LeetCode 206）

```cpp
// 迭代方法
ListNode* reverseList(ListNode* head) {
    ListNode* prev = nullptr;
    ListNode* current = head;
    
    while (current != nullptr) {
        ListNode* next = current->next;
        current->next = prev;
        prev = current;
        current = next;
    }
    
    return prev;
}

// 递归方法
ListNode* reverseListRecursive(ListNode* head) {
    // 基础情况
    if (head == nullptr || head->next == nullptr) {
        return head;
    }
    
    // 递归反转剩余部分
    ListNode* newHead = reverseListRecursive(head->next);
    
    // 反转当前连接
    head->next->next = head;
    head->next = nullptr;
    
    return newHead;
}
```

#### 例题2：反转链表II（LeetCode 92）

```cpp
// 反转从位置m到n的链表
ListNode* reverseBetween(ListNode* head, int left, int right) {
    if (left == right) {
        return head;
    }
    
    ListNode* dummy = new ListNode(0);
    dummy->next = head;
    ListNode* prev = dummy;
    
    // 移动到反转起始位置的前一个节点
    for (int i = 0; i < left - 1; i++) {
        prev = prev->next;
    }
    
    ListNode* current = prev->next;
    
    // 执行反转
    for (int i = 0; i < right - left; i++) {
        ListNode* next = current->next;
        current->next = next->next;
        next->next = prev->next;
        prev->next = next;
    }
    
    ListNode* result = dummy->next;
    delete dummy;
    return result;
}
```

#### 例题3：K个一组反转链表（LeetCode 25）

```cpp
ListNode* reverseKGroup(ListNode* head, int k) {
    // 检查是否有足够的节点
    ListNode* current = head;
    int count = 0;
    while (current != nullptr && count < k) {
        current = current->next;
        count++;
    }
    
    if (count == k) {
        // 递归处理剩余部分
        current = reverseKGroup(current, k);
        
        // 反转当前k个节点
        while (count > 0) {
            ListNode* next = head->next;
            head->next = current;
            current = head;
            head = next;
            count--;
        }
        head = current;
    }
    
    return head;
}
```

---

## 合并算法

### 问题描述
将两个或多个链表合并成一个新的链表，通常要求保持某种顺序。

### 核心技巧
- **归并思想**：类似归并排序
- **虚拟头节点**：简化合并过程
- **分治法**：处理多个链表

### 复杂度分析
- **时间复杂度**：O(n+m) 两个链表，O(nlogk) k个链表
- **空间复杂度**：O(1) 迭代，O(logk) 递归

### 经典例题

#### 例题1：合并两个有序链表（LeetCode 21）

```cpp
// 迭代方法
ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
    ListNode* dummy = new ListNode(0);
    ListNode* current = dummy;
    
    while (list1 != nullptr && list2 != nullptr) {
        if (list1->val <= list2->val) {
            current->next = list1;
            list1 = list1->next;
        } else {
            current->next = list2;
            list2 = list2->next;
        }
        current = current->next;
    }
    
    // 连接剩余节点
    current->next = (list1 != nullptr) ? list1 : list2;
    
    ListNode* result = dummy->next;
    delete dummy;
    return result;
}

// 递归方法
ListNode* mergeTwoListsRecursive(ListNode* list1, ListNode* list2) {
    if (list1 == nullptr) return list2;
    if (list2 == nullptr) return list1;
    
    if (list1->val <= list2->val) {
        list1->next = mergeTwoListsRecursive(list1->next, list2);
        return list1;
    } else {
        list2->next = mergeTwoListsRecursive(list1, list2->next);
        return list2;
    }
}
```

#### 例题2：合并K个有序链表（LeetCode 23）

```cpp
// 分治合并
ListNode* mergeKLists(std::vector<ListNode*>& lists) {
    if (lists.empty()) return nullptr;
    
    while (lists.size() > 1) {
        std::vector<ListNode*> mergedLists;
        
        for (int i = 0; i < lists.size(); i += 2) {
            ListNode* list1 = lists[i];
            ListNode* list2 = (i + 1 < lists.size()) ? lists[i + 1] : nullptr;
            mergedLists.push_back(mergeTwoLists(list1, list2));
        }
        
        lists = mergedLists;
    }
    
    return lists[0];
}

// 优先队列方法
struct Compare {
    bool operator()(ListNode* a, ListNode* b) {
        return a->val > b->val;
    }
};

ListNode* mergeKListsPQ(std::vector<ListNode*>& lists) {
    std::priority_queue<ListNode*, std::vector<ListNode*>, Compare> pq;
    
    // 将所有链表的头节点加入优先队列
    for (ListNode* list : lists) {
        if (list != nullptr) {
            pq.push(list);
        }
    }
    
    ListNode* dummy = new ListNode(0);
    ListNode* current = dummy;
    
    while (!pq.empty()) {
        ListNode* node = pq.top();
        pq.pop();
        
        current->next = node;
        current = current->next;
        
        if (node->next != nullptr) {
            pq.push(node->next);
        }
    }
    
    ListNode* result = dummy->next;
    delete dummy;
    return result;
}
```

---

## 环检测算法

### 问题描述
检测链表中是否存在环，以及找到环的入口点。

### 核心技巧
- **Floyd判圈算法**：快慢指针
- **哈希表法**：记录访问过的节点
- **数学推导**：计算环的起始位置

### 复杂度分析
- **时间复杂度**：O(n)
- **空间复杂度**：O(1) 快慢指针，O(n) 哈希表

### 经典例题

#### 例题1：环形链表（LeetCode 141）

```cpp
// 快慢指针法
bool hasCycle(ListNode* head) {
    if (head == nullptr || head->next == nullptr) {
        return false;
    }
    
    ListNode* slow = head;
    ListNode* fast = head->next;
    
    while (slow != fast) {
        if (fast == nullptr || fast->next == nullptr) {
            return false;
        }
        slow = slow->next;
        fast = fast->next->next;
    }
    
    return true;
}

// 哈希表法
bool hasCycleHash(ListNode* head) {
    std::unordered_set<ListNode*> visited;
    
    while (head != nullptr) {
        if (visited.find(head) != visited.end()) {
            return true;
        }
        visited.insert(head);
        head = head->next;
    }
    
    return false;
}
```

#### 例题2：环形链表II（LeetCode 142）

```cpp
ListNode* detectCycle(ListNode* head) {
    if (head == nullptr || head->next == nullptr) {
        return nullptr;
    }
    
    // 第一步：检测是否有环
    ListNode* slow = head;
    ListNode* fast = head;
    
    while (fast != nullptr && fast->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;
        
        if (slow == fast) {
            break;
        }
    }
    
    // 如果没有环
    if (fast == nullptr || fast->next == nullptr) {
        return nullptr;
    }
    
    // 第二步：找到环的起始点
    slow = head;
    while (slow != fast) {
        slow = slow->next;
        fast = fast->next;
    }
    
    return slow;
}
```

---

## 双指针技巧

### 问题描述
使用两个指针以不同的速度或位置遍历链表，解决特定位置的问题。

### 核心技巧
- **快慢指针**：不同速度移动
- **前后指针**：保持固定距离
- **左右指针**：从两端向中间移动

### 复杂度分析
- **时间复杂度**：O(n)
- **空间复杂度**：O(1)

### 经典例题

#### 例题1：删除链表的倒数第N个节点（LeetCode 19）

```cpp
ListNode* removeNthFromEnd(ListNode* head, int n) {
    ListNode* dummy = new ListNode(0);
    dummy->next = head;
    
    ListNode* first = dummy;
    ListNode* second = dummy;
    
    // 让first先走n+1步
    for (int i = 0; i <= n; i++) {
        first = first->next;
    }
    
    // 同时移动两个指针
    while (first != nullptr) {
        first = first->next;
        second = second->next;
    }
    
    // 删除节点
    ListNode* nodeToDelete = second->next;
    second->next = second->next->next;
    delete nodeToDelete;
    
    ListNode* result = dummy->next;
    delete dummy;
    return result;
}
```

#### 例题2：链表的中间节点（LeetCode 876）

```cpp
ListNode* middleNode(ListNode* head) {
    ListNode* slow = head;
    ListNode* fast = head;
    
    while (fast != nullptr && fast->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;
    }
    
    return slow;
}
```

#### 例题3：回文链表（LeetCode 234）

```cpp
bool isPalindrome(ListNode* head) {
    if (head == nullptr || head->next == nullptr) {
        return true;
    }
    
    // 找到中间节点
    ListNode* slow = head;
    ListNode* fast = head;
    
    while (fast->next != nullptr && fast->next->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;
    }
    
    // 反转后半部分
    ListNode* secondHalf = reverseList(slow->next);
    
    // 比较两部分
    ListNode* firstHalf = head;
    ListNode* secondHalfCopy = secondHalf;
    bool result = true;
    
    while (secondHalf != nullptr) {
        if (firstHalf->val != secondHalf->val) {
            result = false;
            break;
        }
        firstHalf = firstHalf->next;
        secondHalf = secondHalf->next;
    }
    
    // 恢复链表
    slow->next = reverseList(secondHalfCopy);
    
    return result;
}
```

---

## 高级技巧

### 问题描述
复杂的链表操作，包括复制、排序、分割等高级算法。

### 经典例题

#### 例题1：复制带随机指针的链表（LeetCode 138）

```cpp
struct Node {
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};

Node* copyRandomList(Node* head) {
    if (head == nullptr) return nullptr;
    
    // 第一步：在每个节点后插入复制节点
    Node* current = head;
    while (current != nullptr) {
        Node* copy = new Node(current->val);
        copy->next = current->next;
        current->next = copy;
        current = copy->next;
    }
    
    // 第二步：设置random指针
    current = head;
    while (current != nullptr) {
        if (current->random != nullptr) {
            current->next->random = current->random->next;
        }
        current = current->next->next;
    }
    
    // 第三步：分离两个链表
    Node* dummy = new Node(0);
    Node* copyPrev = dummy;
    current = head;
    
    while (current != nullptr) {
        Node* copy = current->next;
        current->next = copy->next;
        copyPrev->next = copy;
        copyPrev = copy;
        current = current->next;
    }
    
    Node* result = dummy->next;
    delete dummy;
    return result;
}
```

#### 例题2：排序链表（LeetCode 148）

```cpp
ListNode* sortList(ListNode* head) {
    if (head == nullptr || head->next == nullptr) {
        return head;
    }
    
    // 找到中间节点并分割
    ListNode* mid = getMid(head);
    ListNode* left = head;
    ListNode* right = mid->next;
    mid->next = nullptr;
    
    // 递归排序
    left = sortList(left);
    right = sortList(right);
    
    // 合并
    return mergeTwoLists(left, right);
}

ListNode* getMid(ListNode* head) {
    ListNode* slow = head;
    ListNode* fast = head;
    ListNode* prev = nullptr;
    
    while (fast != nullptr && fast->next != nullptr) {
        prev = slow;
        slow = slow->next;
        fast = fast->next->next;
    }
    
    return prev;
}
```

---

## 总结与练习建议

### 核心要点总结

1. **基础操作掌握**：遍历、插入、删除是所有算法的基础
2. **指针操作技巧**：正确处理指针的重连和边界情况
3. **常用模式识别**：
   - 快慢指针：环检测、中间节点
   - 虚拟头节点：简化边界处理
   - 递归分治：反转、合并、排序

### 练习建议

#### 初级练习
- LeetCode 206: 反转链表
- LeetCode 21: 合并两个有序链表
- LeetCode 83: 删除排序链表中的重复元素
- LeetCode 141: 环形链表

#### 中级练习
- LeetCode 19: 删除链表的倒数第N个节点
- LeetCode 92: 反转链表II
- LeetCode 142: 环形链表II
- LeetCode 234: 回文链表

#### 高级练习
- LeetCode 25: K个一组反转链表
- LeetCode 23: 合并K个有序链表
- LeetCode 138: 复制带随机指针的链表
- LeetCode 148: 排序链表

### 调试技巧

1. **画图分析**：在纸上画出链表结构变化
2. **边界测试**：空链表、单节点、两节点情况
3. **内存管理**：注意delete释放内存，避免内存泄漏
4. **指针检查**：确保指针操作的正确性

### 时间复杂度速查表

| 操作 | 时间复杂度 | 空间复杂度 |
|------|------------|------------|
| 遍历 | O(n) | O(1) |
| 查找 | O(n) | O(1) |
| 插入（已知位置） | O(1) | O(1) |
| 删除（已知位置） | O(1) | O(1) |
| 反转 | O(n) | O(1) |
| 合并两个链表 | O(n+m) | O(1) |
| 排序 | O(nlogn) | O(logn) |

---

**祝您学习愉快！掌握这些算法模式后，您将能够解决大部分链表相关的算法问题。** 🚀