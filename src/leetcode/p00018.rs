struct Solution {}
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 4 {
            return ans;
        }
        let mut l = 0;
        while l < (nums.len() - 3) {
            let mut m = l + 1;
            while m < nums.len() - 2 {
                let mut n = m + 1;
                let mut o = nums.len() - 1;
                while n < o {
                    let sum = nums[l] as i64 + nums[m] as i64 + nums[n] as i64 + nums[o] as i64;
                    if sum < target as i64 {
                        n += 1;
                    } else if sum > target as i64 {
                        o -= 1;
                    } else {
                        ans.push(vec![nums[l], nums[m], nums[n], nums[o]]);
                        n += 1;
                        o -= 1;
                        while n < o && nums[n] == nums[n - 1] {
                            n += 1;
                        }
                        while n < o && nums[o] == nums[o + 1] {
                            o -= 1;
                        }
                    }
                }
                m += 1;
                while m < nums.len() - 2 && nums[m] == nums[m - 1] {
                    m += 1;
                }
            }
            l += 1;
            while l < nums.len() - 3 && nums[l] == nums[l - 1] {
                l += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00018::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            )
        );
    }
}
