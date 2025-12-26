use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut map = HashMap::new();
        for i in 0..values.len() {
            map.insert(symbols[i].to_string(), values[i]);
        }
        return Self::helper(&map, s);
    }
    pub fn helper(map: &HashMap<String, i32>, s: String) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        while i < s.len() {
            if i + 1 < s.len() && map.contains_key(&s[i..i + 2]) {
                ans += map.get(&s[i..i + 2]).unwrap();
                i += 2;
                continue;
            }
            ans += map.get(&s[i..i + 1]).unwrap();
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::p00013::Solution;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }
}
