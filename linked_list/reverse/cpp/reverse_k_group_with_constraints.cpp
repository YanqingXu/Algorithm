/*
题目：带约束条件的K个一组反转链表 (Reverse Nodes in k-Group with Constraints)
难度：Hard

题目描述：
给你一个链表，每 k 个节点一组进行翻转，但有以下约束条件：
1. 只有当这k个节点的值的和大于等于给定阈值threshold时，才进行反转
2. 如果某一组的节点数不足k个，则不进行反转
3. 反转后需要将相邻的两个反转组之间插入一个值为separator的新节点
4. 最终返回修改后的链表头节点

输入格式：
- head: 链表头节点
- k: 每组的节点数
- threshold: 阈值
- separator: 分隔符值

输出格式：
- 返回修改后的链表头节点

示例1：
输入: head = [1,2,3,4,5,6], k = 3, threshold = 6, separator = 0
输出: [3,2,1,0,6,5,4]
解释:
- 第一组[1,2,3]，和=6>=6，反转为[3,2,1]
- 第二组[4,5,6]，和=15>=6，反转为[6,5,4]
- 在两组之间插入分隔符0

示例2：
输入: head = [1,1,1,2,2,2], k = 3, threshold = 5, separator = 9
输出: [1,1,1,9,2,2,2]
解释:
- 第一组[1,1,1]，和=3<5，不反转
- 第二组[2,2,2]，和=6>=5，反转为[2,2,2]（本身就是回文）
- 在两组之间插入分隔符9

约束条件：
- 链表长度范围: [1, 5000]
- 节点值范围: [-1000, 1000]
- k范围: [1, 链表长度]
- threshold范围: [-3000, 3000]
- separator范围: [-1000, 1000]

时间复杂度: O(n)
空间复杂度: O(1)
*/

#include <iostream>
#include <vector>
#include <cassert>

// 链表节点定义
struct ListNode
{
    int val;
    ListNode *next;

    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution
{
public:
    /**
     * 带约束条件的K个一组反转链表
     *
     * 算法思路：
     * 1. 使用双指针技术遍历链表，每次处理k个节点
     * 2. 计算当前k个节点的和，判断是否满足阈值条件
     * 3. 如果满足条件，使用三指针法反转这k个节点
     * 4. 在相邻的反转组之间插入分隔符节点
     * 5. 继续处理下一组，直到链表结束
     *
     * @param head 链表头节点
     * @param k 每组节点数
     * @param threshold 阈值
     * @param separator 分隔符值
     * @return 修改后的链表头节点
     */
    ListNode *reverseKGroupWithConstraints(ListNode *head, int k, int threshold, int separator)
    {
        if (!head || k <= 1)
            return head;

        // 创建虚拟头节点，简化边界处理
        ListNode *dummy = new ListNode(0);
        dummy->next = head;
        ListNode *prevGroupEnd = dummy;

        while (true)
        {
            // 检查是否还有k个节点
            ListNode *groupStart = prevGroupEnd->next;
            if (!hasKNodes(groupStart, k))
            {
                break;
            }

            // 计算当前k个节点的和
            int sum = calculateSum(groupStart, k);

            // 找到当前组的结束位置
            ListNode *groupEnd = groupStart;
            for (int i = 1; i < k; i++)
            {
                groupEnd = groupEnd->next;
            }
            ListNode *nextGroupStart = groupEnd->next;

            if (sum >= threshold)
            {
                // 满足条件，进行反转
                groupEnd->next = nullptr; // 暂时断开连接
                ListNode *reversedHead = reverseList(groupStart);

                // 重新连接
                prevGroupEnd->next = reversedHead;
                groupStart->next = nextGroupStart; // groupStart现在是反转后的尾节点

                // 如果还有下一组，插入分隔符
                if (nextGroupStart && hasKNodes(nextGroupStart, k))
                {
                    int nextSum = calculateSum(nextGroupStart, k);
                    if (nextSum >= threshold)
                    {
                        ListNode *separatorNode = new ListNode(separator);
                        groupStart->next = separatorNode;
                        separatorNode->next = nextGroupStart;
                        prevGroupEnd = separatorNode;
                    }
                    else
                    {
                        prevGroupEnd = groupStart;
                    }
                }
                else
                {
                    prevGroupEnd = groupStart;
                }
            }
            else
            {
                // 不满足条件，不反转，直接移动到下一组
                prevGroupEnd = groupEnd;
            }
        }

        ListNode *result = dummy->next;
        delete dummy;
        return result;
    }

private:
    /**
     * 检查从当前节点开始是否还有k个节点
     */
    bool hasKNodes(ListNode *node, int k)
    {
        for (int i = 0; i < k && node; i++)
        {
            node = node->next;
        }
        return node != nullptr || k == 0;
    }

    /**
     * 计算从当前节点开始k个节点的值的和
     */
    int calculateSum(ListNode *node, int k)
    {
        int sum = 0;
        for (int i = 0; i < k && node; i++)
        {
            sum += node->val;
            node = node->next;
        }
        return sum;
    }

    /**
     * 反转链表（三指针法）
     */
    ListNode *reverseList(ListNode *head)
    {
        ListNode *prev = nullptr;
        ListNode *current = head;

        while (current)
        {
            ListNode *next = current->next;
            current->next = prev;
            prev = current;
            current = next;
        }

        return prev;
    }
};

// 辅助函数：创建链表
ListNode *createList(const std::vector<int> &values)
{
    if (values.empty())
        return nullptr;

    ListNode *head = new ListNode(values[0]);
    ListNode *current = head;

    for (size_t i = 1; i < values.size(); i++)
    {
        current->next = new ListNode(values[i]);
        current = current->next;
    }

    return head;
}

// 辅助函数：打印链表
void printList(ListNode *head)
{
    while (head)
    {
        std::cout << head->val;
        if (head->next)
            std::cout << " -> ";
        head = head->next;
    }
    std::cout << std::endl;
}

// 辅助函数：释放链表内存
void deleteList(ListNode *head)
{
    while (head)
    {
        ListNode *temp = head;
        head = head->next;
        delete temp;
    }
}

// 测试函数
void runTests()
{
    Solution solution;

    // 测试用例1
    std::cout << "测试用例1:" << std::endl;
    ListNode *head1 = createList({1, 2, 3, 4, 5, 6});
    std::cout << "输入: ";
    printList(head1);
    ListNode *result1 = solution.reverseKGroupWithConstraints(head1, 3, 6, 0);
    std::cout << "输出: ";
    printList(result1);
    std::cout << "期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4" << std::endl
              << std::endl;
    deleteList(result1);

    // 测试用例2
    std::cout << "测试用例2:" << std::endl;
    ListNode *head2 = createList({1, 1, 1, 2, 2, 2});
    std::cout << "输入: ";
    printList(head2);
    ListNode *result2 = solution.reverseKGroupWithConstraints(head2, 3, 5, 9);
    std::cout << "输出: ";
    printList(result2);
    std::cout << "期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2" << std::endl
              << std::endl;
    deleteList(result2);

    // 测试用例3：边界情况
    std::cout << "测试用例3（边界情况）:" << std::endl;
    ListNode *head3 = createList({5});
    std::cout << "输入: ";
    printList(head3);
    ListNode *result3 = solution.reverseKGroupWithConstraints(head3, 1, 3, 0);
    std::cout << "输出: ";
    printList(result3);
    std::cout << "期望: 5" << std::endl
              << std::endl;
    deleteList(result3);
}

int main()
{
    std::cout << "=== 带约束条件的K个一组反转链表 ===" << std::endl;
    std::cout << "难度: Hard" << std::endl;
    std::cout << "时间复杂度: O(n)" << std::endl;
    std::cout << "空间复杂度: O(1)" << std::endl
              << std::endl;

    runTests();

    return 0;
}
