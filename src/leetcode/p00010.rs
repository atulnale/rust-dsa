struct Solution {}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_regex_match(&s.chars().collect(), &p.chars().collect(), 0, 0)
    }

    pub fn is_regex_match(s: &Vec<char>, p: &Vec<char>, mut i: usize, mut j: usize) -> bool {
        if i == s.len() && j == p.len() {
            return true;
        }
        if i == s.len() {
            if j + 1 < p.len() && p[j + 1] == '*' {
                return Self::is_regex_match(s, p, i, j + 2);
            } else {
                return false;
            }
        }
        if j >=p.len() {
            return false;
        }

        if s[i] == p[j] || p[j] == '.' {
            let mut res1 = false;
            let mut has_star = false;
            if j + 1 < p.len() && p[j + 1] == '*' {
                res1 = Self::is_regex_match(s, p, i + 1, j) || Self::is_regex_match(s, p, i, j + 2);
                has_star = true;
            }
            if has_star {
                j = j + 2;
            } else {
                j = j + 1;
            }

            return res1 || Self::is_regex_match(s, p, i + 1, j);
        } else if j + 1 < p.len() && p[j + 1] == '*' {
            return Self::is_regex_match(s, p, i, j + 2);
        }
        return false;
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
            true,
            Solution::is_match(String::from("aaaaaaaaaaaaaaaaaaab"), String::from("a*a*a*a*a*a*a*a*a*a*"))
        );
    }
}
