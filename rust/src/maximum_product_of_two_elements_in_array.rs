use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut first_max = 0;
        let mut second_max = 0;
        nums.iter().for_each(|x| {
            if *x >= first_max {
                second_max = first_max;
                first_max = *x;
            } else if *x > second_max {
                second_max = *x;
            }
        });
        (first_max - 1) * (second_max - 1)
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(Solution::max_product(vec![3, 7]), 12);
    assert_eq!(Solution::max_product(vec![10, 2, 5, 2]), 36);
}
