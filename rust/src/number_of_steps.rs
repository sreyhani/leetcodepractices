use crate::Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        match num {
            0 => 0,
            _ => 31 - num.leading_zeros() as i32 + num.count_ones() as i32,
        }
    }
}
