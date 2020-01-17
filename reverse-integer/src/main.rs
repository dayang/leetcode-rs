//! ## 整数反转 ##
//! [原题目地址](https://leetcode-cn.com/problems/reverse-integer/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//! 
//! 示例 1:
//! ```
//! 输入: 123
//! 输出: 321
//! ```
//! 示例 2:
//! ```
//! 输入: -123
//! 输出: -321
//! ```
//! 示例 3:
//! 
//! 输入: 120
//! 输出: 21
//! 注意:
//! 
//! 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

/// 实现
pub struct Solution;

impl Solution {
    /// 很简单，主要注意溢出问题，rust自带函数检查溢出函数，checked_xxx
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0i32;
        let mut con = x;
        while con != 0 {
            result = match result.checked_mul(10) {
                Some(v) => v + con % 10,
                None => return 0
            };
            con = con / 10;
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reverse_integer() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-1231), -1321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1234567989), 0);
    }
}