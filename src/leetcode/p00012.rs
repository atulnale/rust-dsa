struct Solution {}
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut ans = String::new();
        let mut i = 0;

        while i < values.len() {
            if values[i] <= num {
                ans.push_str(symbols[i]);
                num = num - values[i];
            } else {
                i += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00012::Solution;

    #[test]
    fn test1() {
        assert_eq!("MMMDCCXLIX", Solution::int_to_roman(3749));
    }

    #[test]
    fn test2() {
        assert_eq!("LVIII", Solution::int_to_roman(58));
    }
}
