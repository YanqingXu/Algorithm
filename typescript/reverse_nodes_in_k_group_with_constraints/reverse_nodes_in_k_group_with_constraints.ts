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
class ListNode {
    val: number;
    next: ListNode | null;
    
    constructor(val?: number, next?: ListNode | null) {
        this.val = val ?? 0;
        this.next = next ?? null;
    }
}

// 解决方案类
class Solution {
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
     * @returns 修改后的链表头节点
     */
    reverseKGroupWithConstraints(
        head: ListNode | null,
        k: number,
        threshold: number,
        separator: number
    ): ListNode | null {
        if (!head || k <= 1) {
            return head;
        }
        
        // 创建虚拟头节点，简化边界处理
        const dummy = new ListNode(0);
        dummy.next = head;
        let prevGroupEnd = dummy;
        
        while (true) {
            // 检查是否还有k个节点
            const groupStart = prevGroupEnd.next;
            if (!this.hasKNodes(groupStart, k)) {
                break;
            }
            
            // 计算当前k个节点的和
            const sum = this.calculateSum(groupStart, k);
            
            // 找到当前组的结束位置
            let groupEnd = groupStart;
            for (let i = 1; i < k; i++) {
                groupEnd = groupEnd!.next;
            }
            const nextGroupStart = groupEnd!.next;
            
            if (sum >= threshold) {
                // 满足条件，进行反转
                groupEnd!.next = null; // 暂时断开连接
                const reversedHead = this.reverseList(groupStart);
                
                // 重新连接
                prevGroupEnd.next = reversedHead;
                groupStart!.next = nextGroupStart; // groupStart现在是反转后的尾节点
                
                // 如果还有下一组，检查是否需要插入分隔符
                if (nextGroupStart && this.hasKNodes(nextGroupStart, k)) {
                    const nextSum = this.calculateSum(nextGroupStart, k);
                    if (nextSum >= threshold) {
                        const separatorNode = new ListNode(separator);
                        groupStart!.next = separatorNode;
                        separatorNode.next = nextGroupStart;
                        prevGroupEnd = separatorNode;
                    } else {
                        prevGroupEnd = groupStart!;
                    }
                } else {
                    prevGroupEnd = groupStart!;
                }
            } else {
                // 不满足条件，不反转，直接移动到下一组
                prevGroupEnd = groupEnd!;
            }
        }
        
        return dummy.next;
    }
    
    /**
     * 检查从当前节点开始是否还有k个节点
     */
    private hasKNodes(node: ListNode | null, k: number): boolean {
        for (let i = 0; i < k && node; i++) {
            node = node.next;
        }
        return node !== null || k === 0;
    }
    
    /**
     * 计算从当前节点开始k个节点的值的和
     */
    private calculateSum(node: ListNode | null, k: number): number {
        let sum = 0;
        for (let i = 0; i < k && node; i++) {
            sum += node.val;
            node = node.next;
        }
        return sum;
    }
    
    /**
     * 反转链表（三指针法）
     */
    private reverseList(head: ListNode | null): ListNode | null {
        let prev: ListNode | null = null;
        let current = head;
        
        while (current) {
            const next = current.next;
            current.next = prev;
            prev = current;
            current = next;
        }
        
        return prev;
    }
}

// 辅助函数：创建链表
function createList(values: number[]): ListNode | null {
    if (values.length === 0) {
        return null;
    }
    
    const head = new ListNode(values[0]);
    let current = head;
    
    for (let i = 1; i < values.length; i++) {
        current.next = new ListNode(values[i]);
        current = current.next;
    }
    
    return head;
}

// 辅助函数：打印链表
function printList(head: ListNode | null): void {
    const values: string[] = [];
    while (head) {
        values.push(head.val.toString());
        head = head.next;
    }
    console.log(values.join(' -> ') || '(空链表)');
}

// 辅助函数：链表转数组（用于测试验证）
function listToArray(head: ListNode | null): number[] {
    const result: number[] = [];
    while (head) {
        result.push(head.val);
        head = head.next;
    }
    return result;
}

// 测试函数
function runTests(): void {
    const solution = new Solution();
    
    console.log('=== 带约束条件的K个一组反转链表 ===');
    console.log('难度: Hard');
    console.log('时间复杂度: O(n)');
    console.log('空间复杂度: O(1)');
    console.log();
    
    // 测试用例1
    console.log('测试用例1:');
    const head1 = createList([1, 2, 3, 4, 5, 6]);
    console.log('输入: ');
    printList(head1);
    const result1 = solution.reverseKGroupWithConstraints(head1, 3, 6, 0);
    console.log('输出: ');
    printList(result1);
    console.log('期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4');
    console.log();
    
    // 测试用例2
    console.log('测试用例2:');
    const head2 = createList([1, 1, 1, 2, 2, 2]);
    console.log('输入: ');
    printList(head2);
    const result2 = solution.reverseKGroupWithConstraints(head2, 3, 5, 9);
    console.log('输出: ');
    printList(result2);
    console.log('期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2');
    console.log();
    
    // 测试用例3：边界情况
    console.log('测试用例3（边界情况）:');
    const head3 = createList([5]);
    console.log('输入: ');
    printList(head3);
    const result3 = solution.reverseKGroupWithConstraints(head3, 1, 3, 0);
    console.log('输出: ');
    printList(result3);
    console.log('期望: 5');
    console.log();
    
    // 测试用例4：负数情况
    console.log('测试用例4（负数情况）:');
    const head4 = createList([-1, -2, -3, 4, 5, 6]);
    console.log('输入: ');
    printList(head4);
    const result4 = solution.reverseKGroupWithConstraints(head4, 3, -5, 0);
    console.log('输出: ');
    printList(result4);
    console.log('期望: -3 -> -2 -> -1 -> 0 -> 6 -> 5 -> 4');
    console.log();
    
    // 测试用例5：空链表
    console.log('测试用例5（空链表）:');
    const head5: ListNode | null = null;
    console.log('输入: ');
    printList(head5);
    const result5 = solution.reverseKGroupWithConstraints(head5, 2, 5, 0);
    console.log('输出: ');
    printList(result5);
    console.log('期望: (空链表)');
    console.log();
}

// 性能测试函数
function performanceTest(): void {
    console.log('=== 性能测试 ===');
    const solution = new Solution();
    
    // 创建大链表
    const largeValues: number[] = [];
    for (let i = 1; i <= 1000; i++) {
        largeValues.push(i % 10);
    }
    
    const startTime = Date.now();
    const largeHead = createList(largeValues);
    const result = solution.reverseKGroupWithConstraints(largeHead, 5, 20, 99);
    const endTime = Date.now();
    
    console.log(`处理1000个节点耗时: ${endTime - startTime}ms`);
    console.log('性能测试完成');
    console.log();
}

// 单元测试（使用简单的断言）
function unitTests(): void {
    console.log('=== 单元测试 ===');
    const solution = new Solution();
    
    // 测试1：基本功能
    const test1 = createList([1, 2, 3, 4, 5, 6]);
    const result1 = solution.reverseKGroupWithConstraints(test1, 3, 6, 0);
    const expected1 = [3, 2, 1, 0, 6, 5, 4];
    const actual1 = listToArray(result1);
    console.log('测试1:', JSON.stringify(actual1) === JSON.stringify(expected1) ? '通过' : '失败');
    
    // 测试2：阈值不满足
    const test2 = createList([1, 1, 1, 2, 2, 2]);
    const result2 = solution.reverseKGroupWithConstraints(test2, 3, 5, 9);
    const expected2 = [1, 1, 1, 9, 2, 2, 2];
    const actual2 = listToArray(result2);
    console.log('测试2:', JSON.stringify(actual2) === JSON.stringify(expected2) ? '通过' : '失败');
    
    // 测试3：单节点
    const test3 = createList([5]);
    const result3 = solution.reverseKGroupWithConstraints(test3, 1, 3, 0);
    const expected3 = [5];
    const actual3 = listToArray(result3);
    console.log('测试3:', JSON.stringify(actual3) === JSON.stringify(expected3) ? '通过' : '失败');
    
    console.log('单元测试完成');
    console.log();
}

// 主函数
function main(): void {
    runTests();
    performanceTest();
    unitTests();
}

// 如果是直接运行（不是作为模块导入）
if (require.main === module) {
    main();
}

// 导出类和函数（用于模块化）
export {
    ListNode,
    Solution,
    createList,
    printList,
    listToArray
};