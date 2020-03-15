//! ## 有效的字母异位词 ##
//! [原题目地址](https://leetcode-cn.com/problems/valid-anagram/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词(包含字母完全相同的单词)。
//! 
//! 示例 1:
//! ```
//! 输入: s = "anagram", t = "nagaram"
//! 输出: true
//! ```
//! 示例 2:
//! ```
//! 输入: s = "rat", t = "car"
//! 输出: false
//! 说明:
//! 你可以假设字符串只包含小写字母。
//! ```

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut nums = vec![0; 26];
        for b in s.bytes() {
            nums[(b - 'a' as u8) as usize] += 1;
        }

        for b in t.bytes() {
            nums[(b - 'a' as u8) as usize] -= 1;
        }

        for n in nums {
            if n != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_two_sum(){
        assert_eq!(Solution::is_anagram("angram".to_string(), "nagram".to_string()), true);
        assert_eq!(Solution::is_anagram("cat".to_string(), "car".to_string()), false);
    }
}