struct Solution{}

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut allow_small: bool = true;
        for i in 0..nums.len() {
            if nums[i] > nums[(i+1) % nums.len()] {
                if allow_small {
                    allow_small = false;
                    continue;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test1() {
    assert_eq!(true, Solution::check(vec![1,2,3,4,5]));
    assert_eq!(true, Solution::check(vec![4,5,1,2,3]));
    assert_eq!(false, Solution::check(vec![4,5,1,2,3,1]));
    assert_eq!(false, Solution::check(vec![2,1,3,4]));
}