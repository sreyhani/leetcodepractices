use leetcodepractice::Solution;

#[test]
fn test_1() {
    assert_eq!(Solution::largest_odd_number("52".to_string()), "5");
}

#[test]
fn test_2() {
    assert_eq!(Solution::largest_odd_number("4206".to_string()), "");
}

#[test]
fn test_3() {
    assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427");
}

#[test]
fn test_4() {
    assert_eq!(Solution::largest_odd_number("12570048".to_string()), "1257");
}
