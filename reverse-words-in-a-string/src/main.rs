//! ## 翻转字符串里的单词 ##
//! [原题目地址](https://leetcode-cn.com/problems/reverse-words-in-a-string/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个字符串，逐个翻转字符串中的每个单词。
//! 
//! 示例 1：
//! 
//! 输入: "the sky is blue"
//! 输出: "blue is sky the"
//! 说明：
//! 
//! * 无空格字符构成一个单词。
//! * 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
//! * 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。
pub struct Solution;

impl Solution {
    /// 调用函数
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    /// 先反转整个字符串，再反转每个单词，并去除多余空格
    pub fn reverse_words_v2(s: String) -> String {
        use std::mem::ManuallyDrop;
        let mut s = ManuallyDrop::new(s);
        let s_start = s.as_mut_ptr();
        unsafe{
            let s_end = s_start.offset(s.len() as isize - 1);
            // 反转字符串
            Self::reverse(s_start, s_end);

            let mut index = s_start;
            let mut w_start = s_start;
            let mut w_end = w_start;
            let mut len = 0;
            while w_start <= s_end && w_end <= s_end {
                // 找到单词
                while *w_start == b' ' && w_start <= s_end {
                    w_start = w_start.offset(1);
                }
                w_end = w_start;
                while *w_end != b' ' && w_end <= s_end {
                    w_end = w_end.offset(1);
                }

                // 找到单词添加一个空格
                if w_start != w_end {
                    // 第一个单词不加空格
                    if index != s_start {
                        *index = b' ';
                        index = index.offset(1);
                        len += 1;
                    }

                    // 反转单词
                    Self::reverse(w_start, w_end.offset(-1));
                }

                // 移动单词
                while w_start != w_end {
                    *index = *w_start;
                    index = index.offset(1);
                    len += 1;
                    w_start = w_start.offset(1);
                }
            }

            String::from_raw_parts(s_start, len, len)
        }
    }

    fn reverse(mut start: *mut u8, mut end: *mut u8) {
        while start < end {
            unsafe{
                let temp = *start;
                *start = *end;
                *end = temp;
                start = start.offset(1);
                end = end.offset(-1);
            }
        }
    }
}

fn main() {
    println!("{}", Solution::reverse_words_v2("  hello world!  ".to_string()));
}
