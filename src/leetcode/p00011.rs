struct Solution{}
impl Solution { 
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len()-1;
        let mut ans = 0;
        while i < j {
            let temp = (j - i) as i32 * height[i].min(height[j]);
            ans = ans.max(temp);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::leetcode::p00011::Solution;
    #[test]
    fn test1() {
        assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    }
}