//! ## 无重复字符的最长子串 ## 
//! [原题目地址](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
//! 
//! 示例 1:
//! ```
//! 输入: "abcabcbb"
//! 输出: 3 
//! 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
//! ```
//! 
//! 示例 2:
//! ```
//! 输入: "bbbbb"
//! 输出: 1
//! 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
//! ```
//! 
//! 示例 3:
//! ```
//! 输入: "pwwkew"
//! 输出: 3
//! 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//!      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
//! ```
//! 

use std::collections::HashMap;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0i32;
        let mut exist_chars : HashMap<u8, usize> = HashMap::new();
        let mut sub_seq_start = 0;
        for (i, v) in s.bytes().enumerate() {
            if let Some(pre_index) = exist_chars.insert(v, i) {
                if pre_index >= sub_seq_start {
                    max_len = max(max_len, (i - sub_seq_start) as i32);
                    sub_seq_start = pre_index + 1;
                }
            }
        }
        
        max(max_len, (s.len() - sub_seq_start) as i32)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}