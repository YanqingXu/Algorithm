// Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
// 方法: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
// 时间复杂度: O(N log K)，空间复杂度: O(1) 额外空间（不计返回链表）
#[derive(Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

// merge_two: 合并两个有序链表 / merge two sorted lists
fn merge_two(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut a = a;
    let mut b = b;
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    // 使用 take() 转移所有权，避免克隆，零拷贝串接节点
    // Use take() to move ownership and splice without cloning
    loop {
        match (a.take(), b.take()) {
            (Some(mut na), Some(mut nb)) => {
                if na.val <= nb.val {
                    let next = na.next.take();
                    tail.next = Some(na);
                    tail = tail.next.as_mut().unwrap();
                    a = next;
                    b = Some(nb);
                } else {
                    let next = nb.next.take();
                    tail.next = Some(nb);
                    tail = tail.next.as_mut().unwrap();
                    b = next;
                    a = Some(na);
                }
            }
            (Some(na), None) => {
                tail.next = Some(na);
                break;
            }
            (None, Some(nb)) => {
                tail.next = Some(nb);
                break;
            }
            (None, None) => break,
        }
    }
    dummy.next
}

// merge_k: 通过倍增间隔两两合并 K 个链表 / merge K lists by doubling interval
fn merge_k(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let n = lists.len();
    if n == 0 {
        return None;
    }
    let mut interval = 1usize;
    while interval < lists.len() {
        let mut i = 0usize;
        while i + interval < lists.len() {
            let a = lists[i].take();
            let b = lists[i + interval].take();
            lists[i] = merge_two(a, b);
            i += interval * 2;
        }
        interval *= 2;
    }
    lists.into_iter().next().unwrap()
}

// build: 由切片构造链表 / build list from slice
fn build(v: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    for &x in v {
        tail.next = Some(Box::new(ListNode::new(x)));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

// print_list: 打印链表 / print list
fn print_list(mut h: &Option<Box<ListNode>>) {
    let mut first = true;
    while let Some(node) = h {
        if !first {
            print!(" -> ");
        }
        print!("{}", node.val);
        first = false;
        h = &node.next;
    }
    println!();
}

fn main() {
    // Demo: 构造三个有序链表并合并 / Build and merge 3 sorted lists
    let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
    let res = merge_k(lists);
    print_list(&res);
}
