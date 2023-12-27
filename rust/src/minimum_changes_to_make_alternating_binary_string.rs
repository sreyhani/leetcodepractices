use std::cmp::min;

use crate::Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut zero = true;
        let mut count = 0;
        let mut count2 = 0;
        for ch in s.chars() {
            match (ch, zero) {
                ('0', false) => count += 1,
                ('1', true) => count += 1,
                ('0', true) => count2 += 1,
                ('1', false) => count2 += 1,
                (_, _) => (),
            }
            zero = !zero;
        }
        min(count, count2)
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::min_operations("0100".to_string()), 1);
    assert_eq!(Solution::min_operations("01".to_string()), 0);
    assert_eq!(Solution::min_operations("1111".to_string()), 2);
}

#[test]
fn test2() {
    assert_eq!(Solution::min_operations("10010100".to_string()), 3);
}
