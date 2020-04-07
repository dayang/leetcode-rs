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

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        
        0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_longest_common_subsequence(){
        assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
        assert_eq!(Solution::longest_common_subsequence("asdgsdfay".to_string(), "sddfzzzy".to_string()), 5);
    }
}