//! Leetcode 283. Move Zeroes
//!
//! URL: https://leetcode.com/problems/move-zeroes/description/
//!
//! Submission: 100% runtime, 15.83% memory
struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != 0 {
                let mut temp = nums[right];
                nums[right] = nums[left];
                nums[left] = temp;
                left += 1;
            }
        }
    }
}
#[test]
fn test1() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
