use crate::Solution;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<i32> = points.iter().map(|f| f[0]).collect();
        points.sort();
        let mut points_double = points.clone();
        points_double.insert(0, points[0]);
        points
            .iter()
            .zip(points_double)
            .map(|f| f.0 - f.1)
            .max()
            .unwrap()
    }
}

#[test]
fn test1() {
    let points = [[8, 7], [9, 9], [7, 4], [9, 7]]
        .iter()
        .map(|i| i.to_vec())
        .collect();

    assert_eq!(Solution::max_width_of_vertical_area(points), 1);
}

#[test]
fn test2() {
    let points = [[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]]
        .iter()
        .map(|i| i.to_vec())
        .collect();
    assert_eq!(Solution::max_width_of_vertical_area(points), 3);
}
