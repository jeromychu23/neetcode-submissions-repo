# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        res = []
        curr = head
        while curr:
            res.append(curr.val)
            curr = curr.next
        res.pop(-n)
        dummy = ListNode()
        curr = dummy
        for val in res:
            curr.next = ListNode(val)
            curr = curr.next
        return dummy.next