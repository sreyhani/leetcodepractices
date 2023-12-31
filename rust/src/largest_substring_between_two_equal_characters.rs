use crate::Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max_len = -1;
        let mut first_occurence = [-1; 26];
        for (i, ch) in s.chars().enumerate() {
            let char_int: usize = (ch as u32 - 'a' as u32).try_into().unwrap();
            if first_occurence[char_int] == -1 {
                first_occurence[char_int] = i as i32;
            } else {
                max_len = std::cmp::max(max_len, i as i32 - first_occurence[char_int] - 1)
            }
        }
        max_len
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::max_length_between_equal_characters("aa".to_string()),
        0
    );
    assert_eq!(
        Solution::max_length_between_equal_characters("aba".to_string()),
        1
    );
    assert_eq!(
        Solution::max_length_between_equal_characters("abca".to_string()),
        2
    );
}
