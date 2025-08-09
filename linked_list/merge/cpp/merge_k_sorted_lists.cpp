// Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
// Approach: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
// Time: O(N log K)  (N 为所有节点总数, K 为链表个数)
// Space: O(1) 额外空间（不含返回链表）/ O(1) extra (excluding result list)

#include <bits/stdc++.h>
using namespace std;

struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};
// mergeTwo: 合并两个有序链表，返回新链表头 / Merge two sorted lists and return the merged head
ListNode *mergeTwo(ListNode *a, ListNode *b)
{
    ListNode dummy(0);
    ListNode *tail = &dummy;
    while (a && b)
    {
        if (a->val <= b->val)
        {
            tail->next = a;
            a = a->next;
        }
        else
        {
            tail->next = b;
            b = b->next;
        }
    }
    tail->next = a ? a : b;
    return dummy.next;
}

// 迭代分治合并 K 个链表 / Divide-and-conquer (iterative) merge K lists
ListNode *mergeK(vector<ListNode *> &lists)
{
    if (lists.empty())
        return nullptr;
    int n = (int)lists.size();
    int interval = 1;
    while (interval < n)
    {
        for (int i = 0; i + interval < n; i += interval * 2)
        {
            lists[i] = mergeTwo(lists[i], lists[i + interval]);
        }
        interval *= 2;
    }
    return lists[0];
}

// helpers for demo
ListNode *build(const vector<int> &v)
{
    ListNode dummy(0);
    ListNode *t = &dummy;
    for (int x : v)
    {
        t->next = new ListNode(x);
        t = t->next;
    }
    return dummy.next;
}

void print(ListNode *h)
{
    while (h)
    {
        cout << h->val;
        if (h->next)
            cout << " -> ";
        h = h->next;
    }
    cout << "\n";
}

int main()
{
    vector<ListNode *> lists;
    lists.push_back(build({1, 4, 5}));
    lists.push_back(build({1, 3, 4}));
    lists.push_back(build({2, 6}));

    ListNode *res = mergeK(lists);
    print(res);
    return 0;
}
