use crate::Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transpos = Vec::new();
        for i in 0..matrix[0].len() {
            let mut t_row = Vec::new();
            for row in &matrix {
                t_row.push(row[i]);
            }
            transpos.push(t_row);
        }
        transpos
    }
}

#[test]
fn test1() {
    let v = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].to_vec();
    let v_t = [vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]].to_vec();
    assert_eq!(Solution::transpose(v), v_t);
}
