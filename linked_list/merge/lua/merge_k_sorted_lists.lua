-- Merge K Sorted Lists — 合并 K 个有序链表 (Hard)
-- 方法: 迭代分治（两两合并，间隔倍增）/ Iterative divide-and-conquer (pairwise merge with doubling interval)
-- 时间复杂度: O(N log K)，空间复杂度: O(1) 额外空间（不计返回链表）

local ListNode = {}
ListNode.__index = ListNode
function ListNode.new(val)
    return setmetatable({val = val, next = nil}, ListNode)
end

-- mergeTwo: 合并两个有序链表 / merge two sorted lists
local function mergeTwo(a, b)
    local dummy = ListNode.new(0)
    local tail = dummy
    while a ~= nil and b ~= nil do
        if a.val <= b.val then
            tail.next = a
            a = a.next
        else
            tail.next = b
            b = b.next
        end
        tail = tail.next
    end
    tail.next = a or b
    return dummy.next
end

-- mergeK: 通过倍增间隔两两合并 K 个链表 / merge K lists by doubling interval
local function mergeK(lists)
    local n = #lists
    if n == 0 then
        return nil
    end
    local interval = 1
    while interval < n do
        local i = 1
        while i + interval <= n do
            lists[i] = mergeTwo(lists[i], lists[i + interval])
            i = i + interval * 2
        end
        interval = interval * 2
    end
    return lists[1]
end

-- build: 由数组构造链表 / build list from array
local function build(arr)
    local dummy = ListNode.new(0)
    local t = dummy
    for _, v in ipairs(arr) do
        t.next = ListNode.new(v)
        t = t.next
    end
    return dummy.next
end

-- printList: 打印链表 / print list
local function printList(h)
    local first = true
    while h do
        if not first then
            io.write(" -> ")
        end
        io.write(tostring(h.val))
        first = false
        h = h.next
    end
    io.write("\n")
end

-- Demo: 构造三个有序链表并合并 / Build and merge 3 sorted lists
local lists = {
    build({1, 4, 5}),
    build({1, 3, 4}),
    build({2, 6})
}
local res = mergeK(lists)
printList(res)
