use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);
        let mut ans: Vec<String> = Vec::new();
        Self::helper(&map, &mut ans, &mut vec![], 0, &digits);
        ans
    }

    pub fn helper(
        map: &HashMap<char, Vec<char>>,
        ans: &mut Vec<String>,
        curr: &mut Vec<char>,
        i: usize,
        digits: &String,
    ) {
        if i == digits.len() {
            ans.push(curr.iter().take(curr.len()).collect());
            return;
        }
        let arr = map.get(&digits.chars().nth(i).unwrap()).unwrap();
        for ele in arr {
            curr.push(ele.clone());
            Self::helper(map, ans, curr, i + 1, digits);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00017::Solution;

    #[test]
    fn test1() {
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations(String::from("23"))
        );
    }
}
