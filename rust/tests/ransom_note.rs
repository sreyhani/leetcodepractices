use leetcodepractice::Solution;

#[test]
fn test_1() {
    assert!(!Solution::can_construct("a".to_string(), "b".to_string()))
}

#[test]
fn test_2() {
    assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()))
}

#[test]
fn test_3() {
    assert!(Solution::can_construct("aa".to_string(), "aab".to_string()))
}
