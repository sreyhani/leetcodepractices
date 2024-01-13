use std::convert::TryInto;

use crate::Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        for i in nums {
            if i == 0 {
                max = std::cmp::max(max, count);
                count = 0;
            } else {
                count += 1;
            }
        }
        std::cmp::max(max, count)
    }
}
pub fn is_even_digit(num: &&i32) -> bool {
    let mut num = **num;
    let mut digits = 0;
    while num > 0 {
        digits += 1;
        num = num / 10;
    }
    digits % 2 == 0
}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(is_even_digit)
            .count()
            .try_into()
            .unwrap()
    }
}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = nums.iter().map(|i| i * i).collect();
        squares.sort();
        squares
    }
}
