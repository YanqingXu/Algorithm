// Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
// 方法: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
// 时间复杂度: O(N log K)，空间复杂度: O(1) 额外空间（不计返回链表）

class ListNode {
    val: number;
    next: ListNode | null = null;
    constructor(val: number) { this.val = val; }
}

// mergeTwo: 合并两个有序链表 / merge two sorted lists
function mergeTwo(a: ListNode | null, b: ListNode | null): ListNode | null {
    const dummy = new ListNode(0);
    let tail = dummy;
    while (a && b) {
        if (a.val <= b.val) { tail.next = a; a = a.next; }
        else { tail.next = b; b = b.next; }
        tail = tail.next!;
    }
    tail.next = a ?? b;
    return dummy.next;
}

// mergeK: 通过倍增间隔两两合并 K 个链表 / merge K lists by doubling interval
function mergeK(lists: Array<ListNode | null>): ListNode | null {
    const n = lists.length;
    if (n === 0) return null;
    let interval = 1;
    while (interval < lists.length) {
        for (let i = 0; i + interval < lists.length; i += interval * 2) {
            lists[i] = mergeTwo(lists[i], lists[i + interval]);
        }
        interval *= 2;
    }
    return lists[0];
}

// build: 由数组构造链表 / build list from array
function build(arr: number[]): ListNode | null {
    const dummy = new ListNode(0);
    let t: ListNode = dummy;
    for (const v of arr) { t.next = new ListNode(v); t = t.next; }
    return dummy.next;
}

// printList: 打印链表 / print list
function printList(h: ListNode | null): void {
    const out: number[] = [];
    while (h) { out.push(h.val); h = h.next; }
    console.log(out.join(" -> "));
}

// Demo: 构造三个有序链表并合并 / Build and merge 3 sorted lists
const lists = [build([1, 4, 5]), build([1, 3, 4]), build([2, 6])];
const res = mergeK(lists);
printList(res);
