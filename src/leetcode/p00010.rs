struct Solution {}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_regex_match(&s.chars().collect(), &p.chars().collect(), 0, 0)
    }

    pub fn is_regex_match(s: &Vec<char>, p: &Vec<char>, mut i: usize, mut j: usize) -> bool {
        if j == p.len() {
            return i == s.len();
        }
        let is_char_match = i != s.len() && (s[i] == p[j] || p[j] == '.');
        if j + 1 < p.len() && p[j + 1] == '*' {
            return Self::is_regex_match(s, p, i, j + 2)
                || (is_char_match && Self::is_regex_match(s, p, i + 1, j));
        }
        return is_char_match && Self::is_regex_match(s, p, i + 1, j + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00010::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            false,
            Solution::is_match(String::from("aa"), String::from("a"))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aa"), String::from("aa"))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            false,
            Solution::is_match(String::from("aa"), String::from("."))
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aa"), String::from(".."))
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aa"), String::from(".*"))
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            false,
            Solution::is_match(String::from("aab"), String::from("a*"))
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aab"), String::from("c*a*b"))
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            true,
            Solution::is_match(String::from("a"), String::from("ab*"))
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            false,
            Solution::is_match(
                String::from("aaaaaaaaaaaaaaaaaaab"),
                String::from("a*a*a*a*a*a*a*a*a*a*")
            )
        );
    }
}
