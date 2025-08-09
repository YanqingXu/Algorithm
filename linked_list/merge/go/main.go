// Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
// Approach 方法: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
// Time 时间复杂度: O(N log K), Space 空间复杂度: O(1) 额外空间（不含结果链表）
package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// mergeTwo: 合并两个有序链表 / merge two sorted lists

func mergeTwo(a, b *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for a != nil && b != nil {
		if a.Val <= b.Val {
			tail.Next = a
			a = a.Next
		} else {
			tail.Next = b
			b = b.Next
		}
		tail = tail.Next
	}
	tail.Next = a
	if tail.Next == nil {
		tail.Next = b
	}
	return dummy.Next
}

// mergeK: 通过间隔倍增两两合并 K 个链表 / merge K lists by doubling interval

func mergeK(lists []*ListNode) *ListNode {
	n := len(lists)
	if n == 0 {
		return nil
	}
	interval := 1
	for interval < n {
		for i := 0; i+interval < n; i += interval * 2 {
			lists[i] = mergeTwo(lists[i], lists[i+interval])
		}
		interval *= 2
	}
	// build: 由切片构造链表 / build list from slice
	return lists[0]
}

func build(nums []int) *ListNode {
	dummy := &ListNode{}
	t := dummy
	for _, x := range nums {
		t.Next = &ListNode{Val: x}
		t = t.Next
	}
	// printList: 打印链表 / print list
	return dummy.Next
}

func printList(h *ListNode) {
	for h != nil {
		fmt.Print(h.Val)
		if h.Next != nil {
			fmt.Print(" -> ")
		}
		h = h.Next
		// Demo: 构造三个有序链表并合并 / Build and merge 3 sorted lists
	}
	fmt.Println()
}

func main() {
	lists := []*ListNode{
		build([]int{1, 4, 5}),
		build([]int{1, 3, 4}),
		build([]int{2, 6}),
	}
	res := mergeK(lists)
	printList(res)
}
