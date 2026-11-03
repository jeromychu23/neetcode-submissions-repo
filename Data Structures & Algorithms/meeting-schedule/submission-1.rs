/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|x| x.start);
        let mut cur_end = 0;
        for time in intervals {
            if cur_end > time.start {
                return false;
            }
            cur_end = time.end
        } 
        true
    }
}
