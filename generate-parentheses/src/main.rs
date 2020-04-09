//! ## 括号生成 ## 
//! [原题目地址](https://leetcode-cn.com/problems/generate-parentheses/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 
//! 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
//! 
//! 示例：
//! ```
//! 输入：n = 3
//! 输出：[
//!        "((()))",
//!        "(()())",
//!        "(())()",
//!        "()(())",
//!        "()()()"
//!      ]
//! ```


pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        if n <= 0 {
            return res;
        }
        let mut s = String::new();
        Self::helper(n, n, &mut s, &mut res);
        res
    }

    fn helper(left: i32, right: i32, s: &mut String, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(s.to_string());
        }

        if left > 0 {
            s.push('(');
            Self::helper(left - 1, right, s, res);
            s.pop();
        }

        if right > 0 && right > left {
            s.push(')');
            Self::helper(left, right - 1, s, res);
            s.pop();
        }
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
