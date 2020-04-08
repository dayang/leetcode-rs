//! ## 最长公共子序列 ## 
//! [原题目地址](https://leetcode-cn.com/problems/longest-common-subsequence/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定两个字符串 text1 和 text2，返回这两个字符串的最长公共子序列。
//! 
//! 一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
//! 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。两个字符串的「公共子序列」是这两个字符串所共同拥有的子序列。
//! 
//! 若这两个字符串没有公共子序列，则返回 0。
//! 
//! 示例 1:
//! ```
//! 输入：text1 = "abcde", text2 = "ace" 
//! 输出：3  
//! 解释：最长公共子序列是 "ace"，它的长度为 3。
//! ```
//! 提示:
//! 
//! * 1 <= text1.length <= 1000
//! * 1 <= text2.length <= 1000
//! * 输入的字符串只含有小写英文字符。
//! 
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let mut dp = vec![vec![0;n + 1];m + 1];
        for i in 1..=m{
            for j in 1..=n{
                if s1[i-1] == s2[j-1]{
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j-1], dp[i-1][j]);
                }
            }
        }
        
        dp[m][n]
    }

    /// dp可以优化，二维dp数组，每次生成下一行数据只需要上一行的值，可以优化成一维数组
    pub fn longest_common_subsequence_v2(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let mut dp = vec![0;n + 1];
        // 相当于dp[i-1][j]
        let mut temp;
        for i in 1..=m{
            // 相当于dp[i-1][j-1]
            let mut last = 0;
            for j in 1..=n{
                // dp[j]相当于dp[i][j]
                temp = dp[j];
                if s1[i-1] == s2[j-1] {
                    dp[j] = last + 1;
                } else {
                    dp [j] = std::cmp::max(temp, dp[j-1]);
                }
                last = temp;
            }
        }
        
        dp[n]
    }

    /// 动态规划，自顶向下
    pub fn longest_common_subsequence_v3(text1: String, text2: String) -> i32 {
        let mut memorized = HashMap::new();

        Self::longest_common_subsequence_recurse(text1.as_bytes(), text2.as_bytes(), &mut memorized)
    }

    fn longest_common_subsequence_recurse(s1: &[u8], s2: &[u8], memorized: &mut HashMap<(usize, usize), i32>) -> i32 {
        if s1.len() == 0 || s2.len() == 0 {
            return 0;
        }

        if let Some(len) = memorized.get(&(s1.len(), s2.len())) {
            return *len;
        }

        let res;
        if s1[s1.len() - 1] == s2[s2.len() - 1] {
            res = Self::longest_common_subsequence_recurse(&s1[0..s1.len() - 1], &s2[0..s2.len() - 1], memorized) + 1
        } else {
            res = std::cmp::max(Self::longest_common_subsequence_recurse(&s1[0..s1.len() - 1], s2, memorized),
                            Self::longest_common_subsequence_recurse(s1, &s2[0..s2.len() - 1], memorized))
        }

        memorized.insert((s1.len(), s2.len()), res);
        return res;
    }
}

fn main() {
    println!("{}", Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_longest_common_subsequence(){
        assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
        assert_eq!(Solution::longest_common_subsequence("asdgsdfay".to_string(), "sddfzzzy".to_string()), 5);

        assert_eq!(Solution::longest_common_subsequence_v3("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(Solution::longest_common_subsequence_v3("abc".to_string(), "def".to_string()), 0);
        assert_eq!(Solution::longest_common_subsequence_v3("asdgsdfay".to_string(), "sddfzzzy".to_string()), 5);
    }
}