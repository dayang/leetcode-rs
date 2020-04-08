//! ## 两个字符串的删除操作 ## 
//! [原题目地址](https://leetcode-cn.com/problems/delete-operation-for-two-strings/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定两个单词 word1 和 word2，找到使得 word1 和 word2 相同所需的最小步数，每步可以删除任意一个字符串中的一个字符。
//! 
//! 示例：
//! ```
//! 输入: "sea", "eat"
//! 输出: 2
//! 解释: 第一步将"sea"变为"ea"，第二步将"eat"变为"ea"
//! ```

pub struct Solution;

impl Solution {
    /// 最长公共子序列问题
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();
        let mut dp = vec![0;n + 1];
        let mut temp;
        for i in 1..=m{
            let mut last = 0;
            for j in 1..=n{
                temp = dp[j];
                if s1[i-1] == s2[j-1] {
                    dp[j] = last + 1;
                } else {
                    dp [j] = std::cmp::max(temp, dp[j-1]);
                }
                last = temp;
            }
        }
        
        (m + n) as i32 - dp[n] * 2
    }
}

fn main() {
    println!("Hello, world!");
}
