//! ## 矩阵区域和 ## 
//! [原题目地址](https://leetcode-cn.com/problems/matrix-block-sum/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给你一个 m * n 的矩阵 mat 和一个整数 K ，请你返回一个矩阵 answer ，其中每个 answer[i][j] 是所有满足下述条件的元素 mat[r][c] 的和： 
//! 
//! * i - K <= r <= i + K, j - K <= c <= j + K 
//! * (r, c) 在矩阵内。
//!  
//! 
//! 示例 1：
//! ```
//! 输入：mat = [[1,2,3],[4,5,6],[7,8,9]], K = 1
//! 输出：[[12,21,16],[27,45,33],[24,39,28]]
//! ```
//! 示例 2：
//! ```
//! 输入：mat = [[1,2,3],[4,5,6],[7,8,9]], K = 2
//! 输出：[[45,45,45],[45,45,45],[45,45,45]]
//! ```
//! 
//! 提示：
//! 
//! * m == mat.length
//! * n == mat[i].length
//! * 1 <= m, n, K <= 100
//! * 1 <= mat[i][j] <= 100
use std::cmp::{min, max};
pub struct Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        // 初始化矩阵前缀和
        let mut pm = vec![vec![0_i32;n + 1]; m + 1];
        for i in 1..=m{
            for j in 1..=n {
                pm[i][j] = mat[i-1][j-1] + pm[i-1][j] + pm[i][j-1] - pm[i-1][j-1];
            }
        }

        let mut ans = vec![vec![0_i32;n]; m];
        for i in 0..m {
            for j in 0..n{
                ans[i][j] = Solution::get_pm(&pm, i as i32 + k + 1, j as i32 + k + 1)
                        - Solution::get_pm(&pm, i as i32 - k, j as i32 + k + 1)
                        - Solution::get_pm(&pm, i as i32 + k + 1, j as i32 - k)
                        + Solution::get_pm(&pm, i as i32 - k, j as i32 - k);
                //pm[i + k + 1][j + k + 1] - pm[i - k][j + k + 1] - pm[i + k + 1][j - k] + pm[i - k][j - k];
            }
        }

        ans
    }

    pub fn get_pm(pm: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let mut i = max(i, 0) as usize;
        let mut j = max(j, 0) as usize;
        i = min(i, pm.len() - 1);
        j = min(j, pm[0].len() - 1);
        pm[i][j]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_matrix_block_sum() {
        assert_eq!(Solution::matrix_block_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 2), vec![vec![45,45,45],vec![45,45,45],vec![45,45,45]]);
        assert_eq!(Solution::matrix_block_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], 1), vec![vec![12,21,16],vec![27,45,33],vec![24,39,28]]);
    }
}