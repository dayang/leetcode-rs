//! ## 不相交的线 ## 
//! [原题目地址](https://leetcode-cn.com/problems/uncrossed-lines/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 我们在两条独立的水平线上按给定的顺序写下 A 和 B 中的整数。
//! 
//! 现在，我们可以绘制一些连接两个数字 A[i] 和 B[j] 的直线，只要 A[i] == B[j]，且我们绘制的直线不与任何其他连线（非水平线）相交。
//! 
//! 以这种方法绘制线条，并返回我们可以绘制的最大连线数。

pub struct Solution;

impl Solution {
    /// 即求最长公共子序列
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; b.len() + 1];a.len() + 1];
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i-1] == b[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i-1][j], dp[i][j-1]);
                }
            }
        }

        dp[a.len()][b.len()]
    }
}

fn main() {
    println!("Hello, world!");
}
