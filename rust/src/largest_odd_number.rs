use crate::Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut numm = num;
        match numm
            .chars()
            .rev()
            .position(|c: char| c.to_digit(10).unwrap() % 2 != 0)
        {
            Some(pos) => {
                numm.truncate(numm.len() - pos);
                numm
            }
            None => "".to_string(),
        }
    }
}
