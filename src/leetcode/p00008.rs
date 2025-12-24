struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut ans: i64 = 0;
        while i < s.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= s.len() {
            return 0;
        }
        let sign = if chars[i] == '-' { -1 } else { 1 };
        if chars[i] == '+' || chars[i] == '-' {
            i += 1;
        }
        while i < chars.len() && chars[i].is_ascii_digit() {
            let digit = chars[i].to_digit(10).unwrap();
            ans = (ans * 10) + (sign * digit as i64);
            if ans > i32::MAX as i64 {
                return i32::MAX;
            } else if ans < i32::MIN as i64 {
                return i32::MIN;
            }
            i += 1;
        }

        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00008::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            2147483647,
            Solution::my_atoi(String::from("9223372036854775808"))
        );
    }
}
