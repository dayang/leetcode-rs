//! ## 编辑距离 ## 
//! [原题目地址](https://leetcode-cn.com/problems/edit-distance/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给你两个单词 word1 和 word2，请你计算出将 word1 转换成 word2 所使用的最少操作数 。
//! 
//! 你可以对一个单词进行如下三种操作：
//! 
//! * 插入一个字符
//! * 删除一个字符
//! * 替换一个字符
//!  
//! 
//! 示例 1：
//! ```
//! 输入：word1 = "horse", word2 = "ros"
//! 输出：3
//! 解释：
//! horse -> rorse (将 'h' 替换为 'r')
//! rorse -> rose (删除 'r')
//! rose -> ros (删除 'e')
//! ```
//! 示例 2：
//! ```
//! 输入：word1 = "intention", word2 = "execution"
//! 输出：5
//! 解释：
//! intention -> inention (删除 't')
//! inention -> enention (将 'i' 替换为 'e')
//! enention -> exention (将 'n' 替换为 'x')
//! exention -> exection (将 'n' 替换为 'c')
//! exection -> execution (插入 'u')
//! ```

pub struct Solution;
use std::cmp::min;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0;word2.len() + 1];word1.len() + 1];
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();

        // base case
        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
        }

        for i in 1..=word2.len() {
            dp[0][i] = i as i32;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if s1[i-1] == s2[j-1] {
                    // 字符相等可以不做操作，插入和删除
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    // 字符不等可以替换，插入和删除
                    dp[i][j] = dp[i-1][j-1] + 1;
                }

                // 删除和插入操作，对应4种情况
                dp[i][j] = min(dp[i][j], dp[i-1][j] + 1);
                dp[i][j] = min(dp[i][j], dp[i][j-1] + 1);
            }
        }
        
        dp[word1.len()][word2.len()]
    }
}

fn main() {
    println!("{}", Solution::min_distance("".to_string(), "execution".to_string()));
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
        assert_eq!(Solution::min_distance("".to_string(), "execution".to_string()), 9);
        assert_eq!(Solution::min_distance("sea".to_string(), "eat".to_string()), 2);
    }
}