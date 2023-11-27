use leetcodepractice::Solution;

#[test]
fn test_8() {
    assert_eq!(Solution::number_of_steps(8), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn test_123() {
    assert_eq!(Solution::number_of_steps(123), 12);
}
