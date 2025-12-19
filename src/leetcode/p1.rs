use std::collections::HashMap;


struct Solution{}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i,num) in nums.iter().enumerate() {
            if map.contains_key(&(target - num)) {
                return vec![i as i32, *map.get(&(target - num)).unwrap()];
            }
            map.insert(num, i as i32);
        }
        return vec![-1,-1];
    }
}

#[test]
fn test1() {
    let ans = Solution::two_sum(vec![2,7,11,15], 9);
    assert_eq!(ans, vec![1,0]);
}