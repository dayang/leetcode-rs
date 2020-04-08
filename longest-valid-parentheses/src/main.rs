//! ##  最长有效括号 ## 
//! [原题目地址](https://leetcode-cn.com/problems/longest-valid-parentheses/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
//! 
//! 示例 1:
//! ```
//! 输入: "(()"
//! 输出: 2
//! 解释: 最长有效括号子串为 "()"
//! ```
//! 示例 2:
//! ```
//! 输入: ")()())"
//! 输出: 4
//! 解释: 最长有效括号子串为 "()()"
//! ```

pub struct Solution;

#[derive(Clone,Eq,PartialEq,Copy)]
pub enum SE{
    LeftParam,
    Len(i32),
}

use SE::*;
use std::cmp::max;
impl Solution {
    /// 使用栈，栈顶一直是当前有效子串的长度
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];
        for c in s.chars() {
            if c == ')' {
                if let Some(se) = stack.pop() {
                    match se {
                        LeftParam => {
                            ans = max(ans, Self::reduce(&mut stack, Len(2)));
                        },
                        Len(len) => {
                            if let Some(LeftParam) = stack.pop() {
                                ans = max(ans, Self::reduce(&mut stack, Len(len + 2)));
                            } else {
                                stack.clear();
                            }
                        },
                    };
                }
            } else {
                stack.push(LeftParam);
            }
        }

        ans
    }

    fn reduce(stack: &mut Vec<SE>, se_len: SE) -> i32 {
        match se_len {
            Len(len) => {
                if let Some(Len(l)) = stack.last_mut() {
                    *l += len;
                    *l
                } else {
                    stack.push(se_len);
                    len
                }
            },
            _ => unreachable!()
        }
    }

    /// 动态规划
    pub fn longest_valid_parentheses_v2(_s: String) -> i32 {
        todo!();
    }

    /// 栈
    pub fn longest_valid_parentheses_v3(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = vec![-1];
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    ans = max(ans, i as i32 - stack.last().unwrap());
                }
            }
        }

        ans
    }

    /// 左右遍历，妙啊
    pub fn longest_valid_parentheses_v4(s: String) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut right = 0;
        for c in s.chars() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                ans = max(ans, left * 2);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }

        left = 0;
        right = 0;
        for c in s.chars().rev() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                ans = max(ans, left * 2);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_longest_valid_parentheses(){
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()(()()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()((".to_string()), 2);
    }
}