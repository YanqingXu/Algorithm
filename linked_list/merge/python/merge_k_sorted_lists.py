"""
Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
方法: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
时间复杂度: O(N log K)，空间复杂度: O(1) 额外空间（不计返回链表）
"""

from __future__ import annotations
from typing import List, Optional

class ListNode:
    def __init__(self, val: int, next: Optional['ListNode']=None) -> None:
        self.val = val
        self.next = next


# merge_two: 合并两个有序链表 / merge two sorted lists
def merge_two(a: Optional[ListNode], b: Optional[ListNode]) -> Optional[ListNode]:
    dummy = ListNode(0)
    tail = dummy
    while a and b:
        if a.val <= b.val:
            tail.next = a
            a = a.next
        else:
            tail.next = b
            b = b.next
        tail = tail.next
    tail.next = a or b
    return dummy.next


# merge_k: 通过倍增间隔两两合并 K 个链表 / merge K lists by doubling interval
def merge_k(lists: List[Optional[ListNode]]) -> Optional[ListNode]:
    n = len(lists)
    if n == 0:
        return None
    interval = 1
    while interval < n:
        for i in range(0, n - interval, interval * 2):
            lists[i] = merge_two(lists[i], lists[i + interval])
        interval *= 2
    return lists[0]


# build: 由数组构造链表 / build list from array
def build(arr: List[int]) -> Optional[ListNode]:
    dummy = ListNode(0)
    t = dummy
    for v in arr:
        t.next = ListNode(v)
        t = t.next
    return dummy.next


# print_list: 打印链表 / print list
def print_list(h: Optional[ListNode]) -> None:
    first = True
    while h:
        if not first:
            print(" -> ", end="")
        print(h.val, end="")
        first = False
        h = h.next
    print()


if __name__ == "__main__":
    # Demo: 构造三个有序链表并合并 / Build and merge 3 sorted lists
    lists = [
        build([1,4,5]),
        build([1,3,4]),
        build([2,6]),
    ]
    res = merge_k(lists)
    print_list(res)
