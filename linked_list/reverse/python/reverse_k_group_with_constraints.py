#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
题目：带约束条件的K个一组反转链表 (Reverse Nodes in k-Group with Constraints)
难度：Hard

题目描述：
给你一个链表，每 k 个节点一组进行翻转，但有以下约束条件：
1. 只有当这k个节点的值的和大于等于给定阈值threshold时，才进行反转
2. 如果某一组的节点数不足k个，则不进行反转
3. 反转后需要将相邻的两个反转组之间插入一个值为separator的新节点
4. 最终返回修改后的链表头节点

时间复杂度: O(n)
空间复杂度: O(1)
"""

from typing import Optional, List


class ListNode:
    """链表节点定义"""
    
    def __init__(self, val: int = 0, next_node: Optional['ListNode'] = None):
        self.val = val
        self.next = next_node


class Solution:
    """解决方案类"""
    
    def reverse_k_group_with_constraints(
        self,
        head: Optional[ListNode],
        k: int,
        threshold: int,
        separator: int
    ) -> Optional[ListNode]:
        """
        带约束条件的K个一组反转链表
        
        算法思路：
        1. 使用迭代方法处理每一组k个节点
        2. 对于每一组，先检查节点数是否足够
        3. 计算当前组节点值的和，判断是否满足阈值条件
        4. 如果满足条件，反转当前组
        5. 在相邻的反转组之间插入分隔符节点
        
        Args:
            head: 链表头节点
            k: 每组节点数
            threshold: 阈值
            separator: 分隔符值
            
        Returns:
            修改后的链表头节点
        """
        if not head or k <= 1:
            return head
        
        # 创建虚拟头节点，简化边界处理
        dummy = ListNode(0)
        dummy.next = head
        prev_group_end = dummy
        
        while True:
            # 检查是否还有k个节点
            group_start = prev_group_end.next
            if not self._has_k_nodes(group_start, k):
                break
            
            # 计算当前k个节点的和
            current_sum = self._calculate_sum(group_start, k)
            
            # 找到当前组的结束位置
            group_end = group_start
            for _ in range(k - 1):
                group_end = group_end.next
            next_group_start = group_end.next
            
            if current_sum >= threshold:
                # 满足条件，进行反转
                group_end.next = None  # 暂时断开连接
                reversed_head = self._reverse_list(group_start)
                
                # 重新连接
                prev_group_end.next = reversed_head
                group_start.next = next_group_start  # group_start现在是反转后的尾节点
                
                # 如果还有下一组，检查是否需要插入分隔符
                if next_group_start and self._has_k_nodes(next_group_start, k):
                    next_sum = self._calculate_sum(next_group_start, k)
                    if next_sum >= threshold:
                        separator_node = ListNode(separator)
                        group_start.next = separator_node
                        separator_node.next = next_group_start
                        prev_group_end = separator_node
                    else:
                        prev_group_end = group_start
                else:
                    prev_group_end = group_start
            else:
                # 不满足条件，不反转，直接移动到下一组
                prev_group_end = group_end
        
        return dummy.next
    
    def _has_k_nodes(self, node: Optional[ListNode], k: int) -> bool:
        """检查从当前节点开始是否还有k个节点"""
        for _ in range(k):
            if not node:
                return False
            node = node.next
        return True
    
    def _calculate_sum(self, node: Optional[ListNode], k: int) -> int:
        """计算从当前节点开始k个节点的值的和"""
        total = 0
        for _ in range(k):
            if node:
                total += node.val
                node = node.next
        return total
    
    def _reverse_list(self, head: Optional[ListNode]) -> Optional[ListNode]:
        """反转链表（三指针法）"""
        prev = None
        current = head
        
        while current:
            next_node = current.next
            current.next = prev
            prev = current
            current = next_node
        
        return prev


def create_list(values: List[int]) -> Optional[ListNode]:
    """辅助函数：创建链表"""
    if not values:
        return None
    
    head = ListNode(values[0])
    current = head
    
    for val in values[1:]:
        current.next = ListNode(val)
        current = current.next
    
    return head


def print_list(head: Optional[ListNode]) -> None:
    """辅助函数：打印链表"""
    values = []
    while head:
        values.append(str(head.val))
        head = head.next
    print(' -> '.join(values) if values else '(空链表)')


def run_tests() -> None:
    """测试函数"""
    solution = Solution()
    
    print('=== 带约束条件的K个一组反转链表 ===')
    print('难度: Hard')
    print('时间复杂度: O(n)')
    print('空间复杂度: O(1)')
    print()
    
    # 测试用例1
    print('测试用例1:')
    head1 = create_list([1, 2, 3, 4, 5, 6])
    print('输入: ', end='')
    print_list(head1)
    result1 = solution.reverse_k_group_with_constraints(head1, 3, 6, 0)
    print('输出: ', end='')
    print_list(result1)
    print('期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4')
    print()
    
    # 测试用例2
    print('测试用例2:')
    head2 = create_list([1, 1, 1, 2, 2, 2])
    print('输入: ', end='')
    print_list(head2)
    result2 = solution.reverse_k_group_with_constraints(head2, 3, 5, 9)
    print('输出: ', end='')
    print_list(result2)
    print('期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2')
    print()
    
    # 测试用例3：边界情况
    print('测试用例3（边界情况）:')
    head3 = create_list([5])
    print('输入: ', end='')
    print_list(head3)
    result3 = solution.reverse_k_group_with_constraints(head3, 1, 3, 0)
    print('输出: ', end='')
    print_list(result3)
    print('期望: 5')
    print()


if __name__ == '__main__':
    run_tests()
