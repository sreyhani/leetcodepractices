use crate::Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let len = arr.len();
        for i in (0..len).rev() {
            if arr[i] == 0 {
                arr.insert(i, 0)
            }
        }
        arr.resize(len, 0);
    }
}

#[test]
fn test_1() {
    let mut v = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut v);
    assert_eq!(v, vec![1, 0, 0, 2, 3, 0, 0, 4])
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        while k < (m + n).try_into().unwrap() {
            if j == n.try_into().unwrap() {
                break;
            }
            if nums1[k] > nums2[j] {
                nums1.insert(k, nums2[j]);
                k += 1;
                j += 1;
            } else if i >= m.try_into().unwrap() {
                nums1.insert(k, nums2[j]);
                k += 1;
                j += 1;
            } else {
                i += 1;
                k += 1;
            }
        }
        nums1.resize((m + n).try_into().unwrap(), 0);
    }
}

#[test]
fn test_2() {
    let mut v1 = vec![4, 5, 6, 0, 0, 0];
    let mut v2 = vec![1, 2, 3];
    Solution::merge(&mut v1, 3, &mut v2, 3);
    assert_eq!(v1, vec![1, 2, 3, 4, 5, 6])
}

#[test]
fn test_3() {
    let mut v1 = vec![1];
    let mut v2 = vec![];
    Solution::merge(&mut v1, 1, &mut v2, 0);
    assert_eq!(v1, vec![1])
}

#[test]
fn test_4() {
    let mut v1 = vec![1, 2, 3, 0, 0, 0];
    let mut v2 = vec![2, 5, 6];
    Solution::merge(&mut v1, 3, &mut v2, 3);
    assert_eq!(v1, vec![1, 2, 2, 3, 5, 6])
}
