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
    pub fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
        let mut start: Vec<i32> = intervals.iter().map(|x| x.start).collect();
        let mut end: Vec<i32> = intervals.iter().map(|x| x.end).collect();
        start.sort();
        end.sort();

        let mut count = 0;
        let mut i = 0;
        for n in start {
            if n < end[i] {
                count += 1;
            } else {
                i += 1;
            }
        }
        count
    }
}
