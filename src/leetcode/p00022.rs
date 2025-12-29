struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        Self::paran(n, n, String::new(), &mut ans);
        ans
    }
    pub fn paran(o: i32, c: i32, curr: String, res: &mut Vec<String>) {
        if o == 0 && c == 0 {
            res.push(curr);
            return;
        }
        if o > 0 {
            Self::paran(o - 1, c, format!("{}{}", curr, "("), res);
        }
        if c > 0 && c > o {
            Self::paran(o, c - 1, format!("{}{}", curr, ")"), res);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00022::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
            Solution::generate_parenthesis(3)
        );
    }
}
