use crate::Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        let remained_days = n % 7;
        weeks * Self::summage(7)
            + 7 * Self::summage(weeks - 1)
            + Self::summage(remained_days)
            + (weeks) * remained_days
    }

    fn summage(n: i32) -> i32 {
        (n * (n + 1)) / 2
    }
}

#[test]
fn test_summage() {
    assert_eq!(Solution::summage(0), 0);
    assert_eq!(Solution::summage(1), 1);
    assert_eq!(Solution::summage(2), 3);
    assert_eq!(Solution::summage(3), 6);
    assert_eq!(Solution::summage(7), 28);
}
