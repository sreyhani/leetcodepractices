use leetcodepractice::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::total_money(4), 10);
}

#[test]
fn test_2() {
    assert_eq!(Solution::total_money(10), 37);
}

#[test]
fn test_3() {
    assert_eq!(Solution::total_money(20), 96);
}
