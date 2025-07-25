/*
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
*/

package main

import (
	"fmt"
)

// ListNode 链表节点定义
type ListNode struct {
	Val  int
	Next *ListNode
}

// NewListNode 创建新的链表节点
func NewListNode(val int) *ListNode {
	return &ListNode{Val: val, Next: nil}
}

// Solution 解决方案结构体
type Solution struct{}

/**
 * 带约束条件的K个一组反转链表
 *
 * 算法思路：
 * 1. 使用迭代方法处理每一组k个节点
 * 2. 对于每一组，先检查节点数是否足够
 * 3. 计算当前组节点值的和，判断是否满足阈值条件
 * 4. 如果满足条件，反转当前组
 * 5. 在相邻的反转组之间插入分隔符节点
 *
 * @param head 链表头节点
 * @param k 每组节点数
 * @param threshold 阈值
 * @param separator 分隔符值
 * @return 修改后的链表头节点
 */
func (s *Solution) ReverseKGroupWithConstraints(head *ListNode, k int, threshold int, separator int) *ListNode {
	if head == nil || k <= 1 {
		return head
	}

	// 创建虚拟头节点，简化边界处理
	dummy := &ListNode{Val: 0, Next: head}
	prevGroupEnd := dummy

	for {
		// 检查是否还有k个节点
		groupStart := prevGroupEnd.Next
		if !s.hasKNodes(groupStart, k) {
			break
		}

		// 计算当前k个节点的和
		sum := s.calculateSum(groupStart, k)

		// 找到当前组的结束位置
		groupEnd := groupStart
		for i := 1; i < k; i++ {
			groupEnd = groupEnd.Next
		}
		nextGroupStart := groupEnd.Next

		if sum >= threshold {
			// 满足条件，进行反转
			groupEnd.Next = nil // 暂时断开连接
			reversedHead := s.reverseList(groupStart)

			// 重新连接
			prevGroupEnd.Next = reversedHead
			groupStart.Next = nextGroupStart // groupStart现在是反转后的尾节点

			// 如果还有下一组，检查是否需要插入分隔符
			if nextGroupStart != nil && s.hasKNodes(nextGroupStart, k) {
				nextSum := s.calculateSum(nextGroupStart, k)
				if nextSum >= threshold {
					separatorNode := NewListNode(separator)
					groupStart.Next = separatorNode
					separatorNode.Next = nextGroupStart
					prevGroupEnd = separatorNode
				} else {
					prevGroupEnd = groupStart
				}
			} else {
				prevGroupEnd = groupStart
			}
		} else {
			// 不满足条件，不反转，直接移动到下一组
			prevGroupEnd = groupEnd
		}
	}

	return dummy.Next
}

/**
 * 检查从当前节点开始是否还有k个节点
 */
func (s *Solution) hasKNodes(node *ListNode, k int) bool {
	for i := 0; i < k && node != nil; i++ {
		node = node.Next
	}
	return node != nil || k == 0
}

/**
 * 计算从当前节点开始k个节点的值的和
 */
func (s *Solution) calculateSum(node *ListNode, k int) int {
	sum := 0
	for i := 0; i < k && node != nil; i++ {
		sum += node.Val
		node = node.Next
	}
	return sum
}

/**
 * 反转链表（三指针法）
 */
func (s *Solution) reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	current := head

	for current != nil {
		next := current.Next
		current.Next = prev
		prev = current
		current = next
	}

	return prev
}

// 辅助函数：创建链表
func createList(values []int) *ListNode {
	if len(values) == 0 {
		return nil
	}

	head := NewListNode(values[0])
	current := head

	for i := 1; i < len(values); i++ {
		current.Next = NewListNode(values[i])
		current = current.Next
	}

	return head
}

// 辅助函数：打印链表
func printList(head *ListNode) {
	for head != nil {
		fmt.Print(head.Val)
		if head.Next != nil {
			fmt.Print(" -> ")
		}
		head = head.Next
	}
	fmt.Println()
}

// 测试函数
func runTests() {
	solution := &Solution{}

	fmt.Println("=== 带约束条件的K个一组反转链表 ===")
	fmt.Println("难度: Hard")
	fmt.Println("时间复杂度: O(n)")
	fmt.Println("空间复杂度: O(1)")
	fmt.Println()

	// 测试用例1
	fmt.Println("测试用例1:")
	head1 := createList([]int{1, 2, 3, 4, 5, 6})
	fmt.Print("输入: ")
	printList(head1)
	result1 := solution.ReverseKGroupWithConstraints(head1, 3, 6, 0)
	fmt.Print("输出: ")
	printList(result1)
	fmt.Println("期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4")
	fmt.Println()

	// 测试用例2
	fmt.Println("测试用例2:")
	head2 := createList([]int{1, 1, 1, 2, 2, 2})
	fmt.Print("输入: ")
	printList(head2)
	result2 := solution.ReverseKGroupWithConstraints(head2, 3, 5, 9)
	fmt.Print("输出: ")
	printList(result2)
	fmt.Println("期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2")
	fmt.Println()

	// 测试用例3：边界情况
	fmt.Println("测试用例3（边界情况）:")
	head3 := createList([]int{5})
	fmt.Print("输入: ")
	printList(head3)
	result3 := solution.ReverseKGroupWithConstraints(head3, 1, 3, 0)
	fmt.Print("输出: ")
	printList(result3)
	fmt.Println("期望: 5")
	fmt.Println()

	// 测试用例4：负数情况
	fmt.Println("测试用例4（负数情况）:")
	head4 := createList([]int{-1, -2, -3, 4, 5, 6})
	fmt.Print("输入: ")
	printList(head4)
	result4 := solution.ReverseKGroupWithConstraints(head4, 3, -5, 0)
	fmt.Print("输出: ")
	printList(result4)
	fmt.Println("期望: -3 -> -2 -> -1 -> 0 -> 6 -> 5 -> 4")
	fmt.Println()
}

func main() {
	runTests()
}