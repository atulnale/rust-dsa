use std::i32;

struct Solution {}
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut ans = i32::MAX;
        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let temp = nums[i] + nums[j] + nums[k];
                if (target - ans).abs() > (target - (temp)).abs() {
                    ans = temp
                }
                if (target - temp) == 0 {
                    return temp;
                }
                if temp < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00016::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(2, Solution::three_sum_closest(nums, 1));
    }
}
