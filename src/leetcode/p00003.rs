struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        let mut i: usize = 0;
        let mut ans = 0;
        for (j, ele) in s.chars().enumerate() {
            if let Some(&x) = map.get(&ele) {
                if x >= i {
                    i = x + 1;
                }
            }
            ans = ans.max(j - i + 1);
            map.insert(ele, j);
        }
        return ans as i32;
    }
}

#[test]
fn test1() {
    let s = "abba".to_string();
    assert_eq!(2, Solution::length_of_longest_substring(s));
}

#[test]
fn test2() {
    let s = "abcabcbb".to_string();
    assert_eq!(3, Solution::length_of_longest_substring(s));
}

#[test]
fn test3() {
    let s = "pwwkew".to_string();
    assert_eq!(3, Solution::length_of_longest_substring(s));
}

#[test]
fn test4() {
    let s = "".to_string();
    assert_eq!(0, Solution::length_of_longest_substring(s));
}
