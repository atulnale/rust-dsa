struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut temp = x;
        let mut rev = 0;
        while temp != 0 {
            rev = (rev * 10) + (temp % 10);
            temp = temp / 10;
        }
        x == rev
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00009::Solution;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_palindrome(323));
    }

    #[test]
    fn test2() {
        assert_eq!(false, Solution::is_palindrome(-323));
    }
}
