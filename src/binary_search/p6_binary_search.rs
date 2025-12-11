//! 704. Binary Search
//!
//! URL: https://leetcode.com/problems/binary-search/submissions/1852912253/
//!
//! Submission: 100% runtime, 67.83% memory

struct Solution{}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search(&nums, 0, nums.len() as i32 -1, target)
    }
    pub fn binary_search(nums: &Vec<i32>, l: i32, r: i32, target: i32) -> i32 {
        if l > r {
            return -1;
        }
        let mid = l + ( r - l )/ 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Less => Self::binary_search(nums, mid+1, r, target),
            std::cmp::Ordering::Greater => Self::binary_search(nums, l, mid-1, target)
        }
    }
}

#[test]
fn test() {
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 9));
    println!("{}", Solution::search(vec![5], -5));
}
    