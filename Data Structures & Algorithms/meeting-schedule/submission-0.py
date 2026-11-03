"""
Definition of Interval:
class Interval(object):
    def __init__(self, start, end):
        self.start = start
        self.end = end
"""

class Solution:
    def canAttendMeetings(self, intervals: List[Interval]) -> bool:
        intervals = sorted(intervals, key=lambda self: self.start)
        cur_end = 0
        for time in intervals:
            if time.start < cur_end:
                return False
            cur_end = time.end

        return True