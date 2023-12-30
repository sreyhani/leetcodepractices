use crate::Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = [0; 26];
        for word in &words {
            for ch in word.chars() {
                let i: usize = (ch as u32 - 'a' as u32).try_into().unwrap();
                counts[i] += 1;
            }
        }
        for count in counts {
            if count % words.len() != 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test1() {
    assert!(Solution::make_equal(vec![
        "abc".to_string(),
        "aabc".to_string(),
        "bc".to_string()
    ]));
}
