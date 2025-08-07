--[[
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
--]]
-- 设置UTF-8编码以支持中文输出
if os.getenv("OS") and string.find(os.getenv("OS"):lower(), "windows") then
    os.execute("chcp 65001 > nul 2>&1")
end

-- 链表节点定义
local ListNode = {}
ListNode.__index = ListNode

function ListNode:new(val, next)
    local node = {
        val = val or 0,
        next = next or nil
    }
    setmetatable(node, ListNode)
    return node
end

-- 解决方案类
local Solution = {}
Solution.__index = Solution

function Solution:new()
    local obj = {}
    setmetatable(obj, Solution)
    return obj
end

--[[
带约束条件的K个一组反转链表

算法思路：
1. 使用迭代方法处理每一组k个节点
2. 对于每一组，先检查节点数是否足够
3. 计算当前组节点值的和，判断是否满足阈值条件
4. 如果满足条件，反转当前组
5. 在相邻的反转组之间插入分隔符节点

@param head 链表头节点
@param k 每组节点数
@param threshold 阈值
@param separator 分隔符值
@return 修改后的链表头节点
--]]
function Solution:reverseKGroupWithConstraints(head, k, threshold, separator)
    if not head or k <= 1 then
        return head
    end

    -- 创建虚拟头节点，简化边界处理
    local dummy = ListNode:new(0)
    dummy.next = head
    local prevGroupEnd = dummy

    while true do
        -- 检查是否还有k个节点
        local groupStart = prevGroupEnd.next
        if not self:hasKNodes(groupStart, k) then
            break
        end

        -- 计算当前k个节点的和
        local sum = self:calculateSum(groupStart, k)

        -- 找到当前组的结束位置
        local groupEnd = groupStart
        for i = 1, k - 1 do
            groupEnd = groupEnd.next
        end
        local nextGroupStart = groupEnd.next

        if sum >= threshold then
            -- 满足条件，进行反转
            groupEnd.next = nil -- 暂时断开连接
            local reversedHead = self:reverseList(groupStart)

            -- 重新连接
            prevGroupEnd.next = reversedHead
            groupStart.next = nextGroupStart -- groupStart现在是反转后的尾节点

            -- 如果还有下一组，检查是否需要插入分隔符
            if nextGroupStart and self:hasKNodes(nextGroupStart, k) then
                local nextSum = self:calculateSum(nextGroupStart, k)
                if nextSum >= threshold then
                    local separatorNode = ListNode:new(separator)
                    groupStart.next = separatorNode
                    separatorNode.next = nextGroupStart
                    prevGroupEnd = separatorNode
                else
                    prevGroupEnd = groupStart
                end
            else
                prevGroupEnd = groupStart
            end
        else
            -- 不满足条件，不反转，直接移动到下一组
            prevGroupEnd = groupEnd
        end
    end

    return dummy.next
end

--[[
检查从当前节点开始是否还有k个节点
--]]
function Solution:hasKNodes(node, k)
    for i = 1, k do
        if not node then
            return false
        end
        node = node.next
    end
    return true
end

--[[
计算从当前节点开始k个节点的值的和
--]]
function Solution:calculateSum(node, k)
    local sum = 0
    for i = 1, k do
        if not node then
            break
        end
        sum = sum + node.val
        node = node.next
    end
    return sum
end

--[[
反转链表（三指针法）
--]]
function Solution:reverseList(head)
    local prev = nil
    local current = head

    while current do
        local next = current.next
        current.next = prev
        prev = current
        current = next
    end

    return prev
end

-- 辅助函数：创建链表
function createList(values)
    if not values or #values == 0 then
        return nil
    end

    local head = ListNode:new(values[1])
    local current = head

    for i = 2, #values do
        current.next = ListNode:new(values[i])
        current = current.next
    end

    return head
end

-- 辅助函数：打印链表
function printList(head)
    local values = {}
    while head do
        table.insert(values, tostring(head.val))
        head = head.next
    end
    if #values > 0 then
        print(table.concat(values, " -> "))
    else
        print("(空链表)")
    end
end

-- 测试函数
local function runTests()
    local solution = Solution:new()

    print("=== 带约束条件的K个一组反转链表 ===")
    print("难度: Hard")
    print("时间复杂度: O(n)")
    print("空间复杂度: O(1)")
    print()

    -- 测试用例1
    print("测试用例1:")
    local head1 = createList({1, 2, 3, 4, 5, 6})
    io.write("输入: ")
    printList(head1)
    local result1 = solution:reverseKGroupWithConstraints(head1, 3, 6, 0)
    io.write("输出: ")
    printList(result1)
    print("期望: 3 -> 2 -> 1 -> 0 -> 6 -> 5 -> 4")
    print()

    -- 测试用例2
    print("测试用例2:")
    local head2 = createList({1, 1, 1, 2, 2, 2})
    io.write("输入: ")
    printList(head2)
    local result2 = solution:reverseKGroupWithConstraints(head2, 3, 5, 9)
    io.write("输出: ")
    printList(result2)
    print("期望: 1 -> 1 -> 1 -> 9 -> 2 -> 2 -> 2")
    print()

    -- 测试用例3：边界情况
    print("测试用例3（边界情况）:")
    local head3 = createList({5})
    io.write("输入: ")
    printList(head3)
    local result3 = solution:reverseKGroupWithConstraints(head3, 1, 3, 0)
    io.write("输出: ")
    printList(result3)
    print("期望: 5")
    print()
end

-- 运行测试
runTests()
