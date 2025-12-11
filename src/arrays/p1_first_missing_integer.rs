//! LeetCode 41. First Missing Positive
//!
//! URL: https://leetcode.com/problems/first-missing-positive/
//!
//! Submission: 100% runtime

struct Solution {}
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mod_number = nums.len() as i32 + 1;

        for ele in &mut nums {
            if *ele < 1 || *ele > mod_number - 1 {
                *ele = 0;
            }
        }

        for i in 0..nums.len() {
            let ele = nums[i] % mod_number;
            if ele != 0 && ele <= nums.len() as i32 {
                nums[(ele as usize) - 1] = nums[(ele as usize) - 1].abs() + mod_number;
            }
        }
        for (i, ele) in nums.iter().enumerate() {
            if *ele < mod_number {
                return (i + 1) as i32;
            }
        }
        nums.len() as i32 + 1
    }
}

#[test]
fn test_1() {
    let nums = vec![2];
    let ans = Solution::first_missing_positive(nums);
    println!("{ans}");
}

// 2   3   1
