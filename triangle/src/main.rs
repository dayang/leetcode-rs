//! ##  三角形最小路径和 ##
//! [原题目地址](https://leetcode-cn.com/problems/triangle/) 难度：<b>中等</b>
//! ### 题目描述 #
//! 给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。
//! 
//! 例如，给定三角形：
//! ```
//! [
//!      [2],
//!     [3,4],
//!    [6,5,7],
//!   [4,1,8,3]
//! ]
//! 自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。
//! ```
//! 说明：
//! 
//! * 如果你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题，那么你的算法会很加分。

pub struct Solution;

use std::cmp::min;
impl Solution {
    /// 空间O(n^2 / 2)
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![];
        let mut ans = std::i32::MAX;
        for r in 0..triangle.len() {
            dp.push(vec![]);
            for col in 0..triangle[r].len() {
                dp[r].push(std::i32::MAX);
                if r > 0 {
                    if r != col {
                        dp[r][col] = dp[r-1][col] + triangle[r][col];
                    }

                    if col > 0 {
                        dp[r][col] = min(dp[r][col], dp[r-1][col - 1] + triangle[r][col]);
                    }
                } else {
                    dp[r][col] = triangle[r][col];
                }

                if r == triangle.len() - 1 {
                    ans = min(ans, dp[r][col]);
                }
            }
        }

        ans
    }

    /// 这里从三角形顶部到底部算，还可以从三角形底部到顶部算
    /// 空间O(n)
    pub fn minimum_total_v2(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0;triangle.len()];
        let mut ans = std::i32::MAX;
        for r in 0..triangle.len() {
            for col in (0..triangle[r].len()).rev() {
                if r > 0 {
                    if col == r {
                        dp[col] = dp[col - 1] + triangle[r][col];
                    } else if col > 0 {
                        dp[col] = min(dp[col], dp[col - 1]) + triangle[r][col];
                    } else if col == 0 {
                        dp[col] = dp[col] + triangle[r][col];
                    }
                } else {
                    dp[col] = triangle[r][col];
                }

                if r == triangle.len() - 1 {
                    ans = min(ans, dp[col]);
                }
            }
        }

        ans
    }

}

fn main() {
    println!("Hello, world!");
}
