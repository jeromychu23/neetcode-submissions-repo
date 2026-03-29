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
            res.append(curr)
            curr = curr.next
        removeindex = len(res) - n
        if removeindex == 0:
            return head.next

        res[removeindex - 1].next = res[removeindex].next
        return head