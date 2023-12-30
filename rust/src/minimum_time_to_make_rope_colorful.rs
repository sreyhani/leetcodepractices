use crate::Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut costs = 0;
		let colors = colors.as_bytes();
		let mut i = 0;
		while i < colors.len() {
			let mut temp_cost = 0;
			let mut max_time = needed_time[i];
			let mut j = i + 1;
			while j < colors.len() {
				if colors[i] == colors[j] {
					if max_time < needed_time[j] {
						temp_cost += max_time;
						max_time = needed_time[j];
					} else {
						temp_cost += needed_time[j];
					}
				} else {
					break;
				}
				j+=1;
			}
			i = j;
			costs += temp_cost;
		}
		costs
    }
}

#[test]
fn test1(){
	assert_eq!(Solution::min_cost("abaac".to_string(), vec![1,2,3,4,5]), 3);
	assert_eq!(Solution::min_cost("abc".to_string(), vec![1,2,3]), 0);
	assert_eq!(Solution::min_cost("aabaa".to_string(), vec![1,2,3,4,1]), 2);
}

#[test]
fn test2(){
	assert_eq!(Solution::min_cost("bbb".to_string(), vec![4,9,3]), 7);
	assert_eq!(Solution::min_cost("bbbaaa".to_string(), vec![4,9,3,8,8,9]), 23);
}

