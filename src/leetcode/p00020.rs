struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let arr: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = Vec::new();
        for i in 0..arr.len() {
            match arr[i] {
                ']' => { if Some('[') != stack.pop() {return false;}}
                '}' => {if Some('{') != stack.pop() {return false;}}
                ')' => {if Some('(') != stack.pop() {return false;}}
                _ => stack.push(arr[i]),
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p00020::Solution;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::is_valid(String::from("()")));
    }
}
