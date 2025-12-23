use std::i32;

struct Solution{}
impl Solution {
     pub fn reverse(mut x: i32) -> i32 {
        let sign = if x < 0 {
            -1
        } else {
            1
        };
        x = x * sign;
        let mut ans = 0;
        while x != 0 {
            let mut temp = ans;
            let last_digit = x % 10;
            x = x / 10;
            temp = ans * 10 + last_digit;
            if ans != ( temp - last_digit ) / 10 {
                return 0;
            }
            ans = temp;
        }
         ans * sign
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00007::Solution;

    #[test]
    fn test1() {
        assert_eq!(321,Solution::reverse(123));
    }

    #[test]
    fn test2() {
        assert_eq!(-321,Solution::reverse(-123));
    }
}
