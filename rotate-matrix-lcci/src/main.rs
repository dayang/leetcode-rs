//! ## 旋转矩阵 ##
//! [原题目地址](https://leetcode-cn.com/problems/rotate-matrix-lcci/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给你一幅由 N × N 矩阵表示的图像，其中每个像素的大小为 4 字节。请你设计一种算法，将图像旋转 90 度。
//! 
//! 不占用额外内存空间能否做到？
//! 
//! 示例 1:
//! 
//! 给定 matrix = 
//! [
//!   [1,2,3],
//!   [4,5,6],
//!   [7,8,9]
//! ],
//! 
//! 原地旋转输入矩阵，使其变为:
//! [
//!   [7,4,1],
//!   [8,5,2],
//!   [9,6,3]
//! ]

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n-i-1 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n-j-1][i];
                matrix[n-j-1][i] = matrix[n-i-1][n-j-1];
                matrix[n-i-1][n-j-1] = matrix[j][n-i-1];
                matrix[j][n-i-1] = temp;

                // let mut temp = matrix[j][n-i-1];
                // matrix[j][n-i-1] = matrix[i][j];
                // matrix[i][j] = temp;

                // temp = matrix[n-i-1][n-j-1];
                // matrix[n-i-1][n-j-1] = matrix[i][j];
                // matrix[i][j] = temp;

                // temp = matrix[n-j-1][i];
                // matrix[n-j-1][i] = matrix[i][j];
                // matrix[i][j] = temp;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
