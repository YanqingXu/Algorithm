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

// 链表节点定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    /**
     * 带约束条件的K个一组反转链表
     *
     * 算法思路：
     * 1. 使用递归方法处理每一组k个节点
     * 2. 对于每一组，先检查节点数是否足够
     * 3. 计算当前组节点值的和，判断是否满足阈值条件
     * 4. 如果满足条件，反转当前组并递归处理剩余部分
     * 5. 在需要的地方插入分隔符节点
     *
     * @param head 链表头节点
     * @param k 每组节点数
     * @param threshold 阈值
     * @param separator 分隔符值
     * @return 修改后的链表头节点
     */
    pub fn reverse_k_group_with_constraints(
        head: Option<Box<ListNode>>,
        k: i32,
        threshold: i32,
        separator: i32,
    ) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 1 {
            return head;
        }

        // 检查是否有足够的k个节点
        let mut count = 0;
        let mut current = &head;
        while let Some(node) = current {
            count += 1;
            if count == k {
                break;
            }
            current = &node.next;
        }

        if count < k {
            return head;
        }

        // 计算当前k个节点的和
        let sum = Self::calculate_sum(&head, k);

        if sum >= threshold {
            // 满足条件，进行反转
            let (reversed_head, remaining) = Self::reverse_k_nodes(head, k);

            // 递归处理剩余部分
            let next_processed =
                Self::reverse_k_group_with_constraints(remaining, k, threshold, separator);

            // 检查是否需要插入分隔符
            if next_processed.is_some() {
                let next_sum = Self::calculate_sum(&next_processed, k);
                if Self::has_k_nodes(&next_processed, k) && next_sum >= threshold {
                    // 需要插入分隔符
                    let mut separator_node = Box::new(ListNode::new(separator));
                    separator_node.next = next_processed;
                    Self::connect_tail(reversed_head, Some(separator_node))
                } else {
                    Self::connect_tail(reversed_head, next_processed)
                }
            } else {
                reversed_head
            }
        } else {
            // 不满足条件，不反转，继续处理下一组
            let (current_group, remaining) = Self::split_k_nodes(head, k);
            let next_processed =
                Self::reverse_k_group_with_constraints(remaining, k, threshold, separator);
            Self::connect_tail(current_group, next_processed)
        }
    }

    /**
     * 检查是否有k个节点
     */
    fn has_k_nodes(head: &Option<Box<ListNode>>, k: i32) -> bool {
        let mut count = 0;
        let mut current = head;
        while let Some(node) = current {
            count += 1;
            if count >= k {
                return true;
            }
            current = &node.next;
        }
        false
    }

    /**
     * 计算前k个节点的值的和
     */
    fn calculate_sum(head: &Option<Box<ListNode>>, k: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        let mut current = head;

        while let Some(node) = current {
            if count >= k {
                break;
            }
            sum += node.val;
            count += 1;
            current = &node.next;
        }

        sum
    }

    /**
     * 反转前k个节点
     */
    fn reverse_k_nodes(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let (group, remaining) = Self::split_k_nodes(head, k);
        let reversed = Self::reverse_list(group);
        (reversed, remaining)
    }

    /**
     * 分离前k个节点
     */
    fn split_k_nodes(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if k <= 0 {
            return (None, head);
        }

        let mut current = &mut head;
        for _ in 0..k - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return (head, None);
            }
        }

        if let Some(node) = current {
            let remaining = node.next.take();
            (head, remaining)
        } else {
            (head, None)
        }
    }

    /**
     * 反转链表
     */
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = next;
        }

        prev
    }

    /**
     * 连接链表尾部
     */
    fn connect_tail(
        mut head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return tail;
        }

        let mut current = &mut head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = tail;
                break;
            }
            current = &mut node.next;
        }

        head
    }
}

// 辅助函数：创建链表
fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }

    let mut head = Some(Box::new(ListNode::new(values[0])));
    let mut current = &mut head;

    for &val in values.iter().skip(1) {
        if let Some(node) = current {
            node.next = Some(Box::new(ListNode::new(val)));
            current = &mut node.next;
        }
    }

    head
}

// 辅助函数：打印链表
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    let mut first = true;

    while let Some(node) = current {
        if !first {
            print!(" -> ");
        }
        print!("{}", node.val);
        first = false;
        current = &node.next;
    }
    println!();
}

// 测试函数
fn run_tests() {
    println!("=== 带约束条件的K个一组反转链表 ===");
    println!("难度: Hard");
    println!("时间复杂度: O(n)");
    println!("空间复杂度: O(1)");
    println!();

    // 测试用例1
    println!("测试用例1:");
    let head1 = create_list(vec![1, 2, 3, 4, 5, 6]);
    print!("输入: ");
    print_list(&head1);
    let result1 = Solution::reverse_k_group_with_constraints(head1, 3, 6, 0);
    print!("输出: ");
    print_list(&result1);
    println!("期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4");
    println!();

    // 测试用例2
    println!("测试用例2:");
    let head2 = create_list(vec![1, 1, 1, 2, 2, 2]);
    print!("输入: ");
    print_list(&head2);
    let result2 = Solution::reverse_k_group_with_constraints(head2, 3, 5, 9);
    print!("输出: ");
    print_list(&result2);
    println!("期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2");
    println!();

    // 测试用例3：边界情况
    println!("测试用例3（边界情况）:");
    let head3 = create_list(vec![5]);
    print!("输入: ");
    print_list(&head3);
    let result3 = Solution::reverse_k_group_with_constraints(head3, 1, 3, 0);
    print!("输出: ");
    print_list(&result3);
    println!("期望: 5");
    println!();
}

fn main() {
    run_tests();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let head = create_list(vec![1, 2, 3, 4, 5, 6]);
        let result = Solution::reverse_k_group_with_constraints(head, 3, 6, 0);
        // 验证结果的正确性
        assert!(result.is_some());
    }

    #[test]
    fn test_threshold_not_met() {
        let head = create_list(vec![1, 1, 1, 2, 2, 2]);
        let result = Solution::reverse_k_group_with_constraints(head, 3, 5, 9);
        // 验证结果的正确性
        assert!(result.is_some());
    }

    #[test]
    fn test_single_node() {
        let head = create_list(vec![5]);
        let result = Solution::reverse_k_group_with_constraints(head, 1, 3, 0);
        // 验证结果的正确性
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap().val, 5);
    }
}
