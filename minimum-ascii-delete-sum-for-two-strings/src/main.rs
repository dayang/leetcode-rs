//! ##  两个字符串的最小ASCII删除和## 
//! [原题目地址](https://leetcode-cn.com/problems/minimum-ascii-delete-sum-for-two-strings/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定两个字符串s1, s2，找到使两个字符串相等所需删除字符的ASCII值的最小和。
//! 
//! 示例 1:
//! ```
//! 输入: s1 = "sea", s2 = "eat"
//! 输出: 231
//! 解释: 在 "sea" 中删除 "s" 并将 "s" 的值(115)加入总和。
//! 在 "eat" 中删除 "t" 并将 116 加入总和。
//! 结束时，两个字符串相等，115 + 116 = 231 就是符合条件的最小和。
//! ```
//! 示例 2:
//! ```
//! 输入: s1 = "delete", s2 = "leet"
//! 输出: 403
//! 解释: 在 "delete" 中删除 "dee" 字符串变成 "let"，
//! 将 100[d]+101[e]+101[e] 加入总和。在 "leet" 中删除 "e" 将 101[e] 加入总和。
//! 结束时，两个字符串都等于 "let"，结果即为 100+101+101+101 = 403 。
//! 如果改为将两个字符串转换为 "lee" 或 "eet"，我们会得到 433 或 417 的结果，比答案更大。
//! ```
//! 注意:
//! 
//! * 0 < s1.length, s2.length <= 1000。
//! * 所有字符串中的字符ASCII值在[97, 122]之间。

pub struct Solution;

use std::cmp::min;
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        for i in 1..=s1.len(){
            dp[i][0] = dp[i-1][0] + s1[i-1] as i32;
        }

        for i in 1..=s2.len(){
            dp[0][i] = dp[0][i-1] + s2[i-1] as i32;
        }

        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                if s1[i-1] == s2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = min(dp[i-1][j] + s1[i-1] as i32, dp[i][j-1] + s2[j-1] as i32);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_minimum_delete_sum() {
        assert_eq!(Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()), 231);
        assert_eq!(Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()), 403);
    }
}