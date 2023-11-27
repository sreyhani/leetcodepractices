use crate::Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut most_sign_one: i32 = 0;
        let mut ones: i32 = 0;
        for pos in (0..32).rev() {
            let bit = (num >> pos) & 1;
            match (most_sign_one, bit) {
                (0, 0) => (),
                (0, 1) => most_sign_one = pos + 1,
                (_, 0) => (),
                (_, 1) => ones += 1,
                (_, _) => (),
            }
        }

        most_sign_one + ones
    }
}
