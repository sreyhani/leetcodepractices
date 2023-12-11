use crate::Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        *arr.iter()
            .map(|i| (arr.iter().filter(|&x| x == i).count(), i))
            .max()
            .unwrap()
            .1
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
        6
    )
}
