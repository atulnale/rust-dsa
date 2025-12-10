//! Leetcode 189. Rotate Array
//!
//! URL: https://leetcode.com/problems/rotate-array/description/
//!
//! Submission: 100% runtime, 87.9% memory

struct Solution{}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        k = k % nums.len() as i32;
        if k == 0 {
            return;
        }
        Solution::reverse_array(nums, 0, nums.len()-1);
        Solution::reverse_array(nums, 0, k as usize -1);
        Solution::reverse_array(nums, k as usize, nums.len()-1);

    }

    pub fn reverse_array(nums: &mut Vec<i32>, mut start: usize,mut end: usize) {
        while start < end {
            let temp = nums[start];
            nums[start] = nums[end];
            nums[end] = temp;
            start += 1;
            end -= 1;
        }
            
    }
}

#[test]
fn test() {
    Solution::rotate(&mut vec![1], 2);
}