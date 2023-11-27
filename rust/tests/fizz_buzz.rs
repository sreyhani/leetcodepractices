use leetcodepractice::Solution;

#[test]
fn test_1_2() {
    assert_eq!(Solution::fizz_buzz(1), ["1"]);
    assert_eq!(Solution::fizz_buzz(2), ["1", "2"]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::fizz_buzz(3), ["1", "2", "Fizz"]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::fizz_buzz(5), ["1", "2", "Fizz", "4", "Buzz"]);
}

#[test]
fn test_15() {
    assert_eq!(
        Solution::fizz_buzz(15),
        [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
