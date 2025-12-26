struct Solution{}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let ans: Vec<char> = strs[0].chars().collect();
        let mut j = ans.len();
        for i in 1..strs.len() {
            let temp: Vec<char> = strs[i].chars().collect();
            j = j.min(temp.len());
            for k in 0..j {
                if ans[k] != temp[k] {
                    j = k;
                    break;
                }
            }
        }
        ans.iter().take(j).collect()
    }
}

#[cfg(test)]
mod tests{
    use crate::leetcode::p00014::Solution;

    #[test]
    fn test1() {
        let strs = vec![String::from("flower"),"flow".to_string(),"flight".to_string()];
        assert_eq!("fl".to_string(), Solution::longest_common_prefix(strs));
    }
}