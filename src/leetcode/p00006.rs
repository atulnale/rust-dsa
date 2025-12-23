struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut ans: Vec<char> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        
        let incr = ((num_rows - 1)*2) - 1;
        let mut l = incr;
    
        for i in 0..num_rows {
            let mut j = i;
            while j < chars.len() as i32 {
                ans.push(chars[j as usize]);
                if l > 0 && l < incr && ((j as usize + l as usize + 1) < chars.len()) {
                    ans.push(chars[(j + l + 1) as usize]);
                }
                
                
                j = j + incr + 1;
            }
            l -=2;
        }
        return ans.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00006::Solution;

    #[test]
    fn test1() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 1);
        assert_eq!("PAYPALISHIRING", ans);
    }

    #[test]
    fn test2() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 2);
        assert_eq!("PYAIHRNAPLSIIG", ans);
    }

    #[test]
    fn test3() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 3);
        assert_eq!("PAHNAPLSIIGYIR", ans);
    }

    #[test]
    fn test4() {
        let ans = Solution::convert(String::from("PAYPALISHIRING"), 4);
        assert_eq!("PINALSIGYAHRPI", ans);
    }

    #[test]
    fn test5() {
        let ans = Solution::convert(String::from("ABC"), 3);
        assert_eq!("ABC", ans);
    }

    #[test]
    fn test6() {
        let ans = Solution::convert(String::from("ABCDE"), 4);
        assert_eq!("ABCED", ans);
    }
}
