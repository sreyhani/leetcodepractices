use crate::Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut count = 0;
        let mut cookie_i = 0;

        for child_greed in g {
            if cookie_i >= s.len() {
                return count;
            }
            while s[cookie_i] < child_greed {
                cookie_i += 1;
                if cookie_i >= s.len() {
                    return count;
                }
            }
            count += 1;
            cookie_i += 1;
        }
        count
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    );
    assert_eq!(
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
        2
    );
    assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![3]), 1);
    assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![]), 0);
}
