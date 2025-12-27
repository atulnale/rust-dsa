struct Solution {}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let temp = nums[i] + nums[j] + nums[k];
                if temp > 0 {
                    k -= 1;
                } else if temp < 0 {
                    j += 1;
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                    while (j < k && nums[j] == nums[j - 1]) {
                        j += 1;
                    }
                    while (j < k && nums[k] == nums[k + 1]) {
                        k -= 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00015::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(nums)
        );
    }
}
