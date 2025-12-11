//! Leetcode 62. Unique Paths
//!
//! URL: https://leetcode.com/problems/unique-paths/
//!
//! Submission: 100% runtime, 47.83% memory
struct Solution{}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut matrix= vec![vec![1; n as usize]; m as usize];
        for i in 1..m as usize {
            for j in 1..n as usize {
                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }
        matrix[m as usize - 1][n as usize - 1]
    }
}

#[test]
fn test1() {
    println!("{}", Solution::unique_paths(3, 7));
}