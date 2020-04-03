//! ## 统计全为 1 的正方形子矩阵 ##
//! [原题目地址](https://leetcode-cn.com/problems/count-square-submatrices-with-all-ones/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给你一个 m * n 的矩阵，矩阵中的元素不是 0 就是 1，请你统计并返回其中完全由 1 组成的 正方形 子矩阵的个数。
//! 
//! 示例 1：
//! ```
//! 输入：matrix =
//! [
//!   [0,1,1,1],
//!   [1,1,1,1],
//!   [0,1,1,1]
//! ]
//! 输出：15
//! 解释： 
//! 边长为 1 的正方形有 10 个。
//! 边长为 2 的正方形有 4 个。
//! 边长为 3 的正方形有 1 个。
//! 正方形的总数 = 10 + 4 + 1 = 15.
//! ```
//! 示例 2：
//! ```
//! 输入：matrix = 
//! [
//!   [1,0,1],
//!   [1,1,0],
//!   [1,1,0]
//! ]
//! 输出：7
//! 解释：
//! 边长为 1 的正方形有 6 个。 
//! 边长为 2 的正方形有 1 个。
//! 正方形的总数 = 6 + 1 = 7.
//! ```
//! 
//! 提示：
//! 
//! * 1 <= arr.length <= 300
//! * 1 <= arr[0].length <= 300
//! * 0 <= arr[i][j] <= 1

pub struct Solution;

impl Solution {
    // 动态规划O(m * n)
    // dp[i][j]表示以matrix[i][j] 为右下角的正方形个数，也是最大正方形的边长
    // 如果matrix[i][j] == 0，则dp[i][j] = 0
    // 如果matrix[i][j] == 1，则dp[i][j] = min{dp[i-1][j], dp[i][j-1], dp[i-1][j-1]} + 1，画个图可看出关系
    // dp元素求和即答案
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let m = matrix.len();
        let n = matrix[0].len();

        let mut dp = vec![vec![0;n+1];m+1];
        let mut ans = 0;
        for i in 1..=m{
            for j in 1..=n{
                if matrix[i-1][j-1] == 0 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = min(dp[i-1][j], min(dp[i][j-1], dp[i-1][j-1])) + 1;
                }
                ans += dp[i][j];
            }
        }

        ans
    }

    // 二维前缀和 O(n^3)
    pub fn count_squares_v2(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let max_square_len = std::cmp::min(m, n);

        let mut one_cnts = vec![vec![0;n+1];m+1];
        for i in 1..=m{
            for j in 1..=n{
                one_cnts[i][j] = one_cnts[i-1][j] + one_cnts[i][j-1] - one_cnts[i-1][j-1] + (matrix[i-1][j-1] & 1);
            }
        }

        let mut ans = 0;
        for i in 1..=m{
            for j in 1..=n{
                for k in 1..=max_square_len {
                    if k > i || k > j {
                        break;
                    }
                    if Self::get_matrix_ones(&one_cnts, i, j, k) == (k * k) as i32 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }

    pub fn get_matrix_ones(cnts: &Vec<Vec<i32>>, i: usize, j: usize, squre_len: usize) -> i32 {
        let lt_i = i - squre_len;
        let lt_j = j - squre_len;

        cnts[i][j] - cnts[i][lt_j] - cnts[lt_i][j] + cnts[lt_i][lt_j]
    }
}

fn main() {
    println!("{}", Solution::count_squares(vec![vec![0,1,1,1],vec![1,1,1,1],vec![0,1,1,1]]));
    println!("{}", Solution::count_squares(vec![vec![1,0,1],vec![1,1,0],vec![1,1,0]]));
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn test_count_squares() {
        assert_eq!(Solution::count_squares(vec![vec![0,1,1,1],vec![1,1,1,1],vec![0,1,1,1]]), 15);
        assert_eq!(Solution::count_squares(vec![vec![1,0,1],vec![1,1,0],vec![1,1,0]]), 7);
    }
}