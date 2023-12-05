use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters: HashMap<char, usize> = HashMap::new();
        for ch in magazine.chars() {
            *letters.entry(ch).or_insert(0) += 1
        }
        for ch in ransom_note.chars() {
            let reps = letters.entry(ch).or_insert(0);
            if *reps == 0 {
                return false;
            } else {
                *reps -= 1;
            }
        }
        true
    }
}
