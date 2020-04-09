//! ## 下降路径最小和 ## 
//! [原题目地址](https://leetcode-cn.com/problems/minimum-falling-path-sum/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个方形整数数组 A，我们想要得到通过 A 的下降路径的最小和。
//! 
//! 下降路径可以从第一行中的任何元素开始，并从每一行中选择一个元素。在下一行选择的元素和当前行所选元素最多相隔一列。
//! 
//! 示例：
//! ```
//! 输入：[[1,2,3],[4,5,6],[7,8,9]]
//! 输出：12
//! 解释：
//! 可能的下降路径有：
//! [1,4,7], [1,4,8], [1,5,7], [1,5,8], [1,5,9]
//! [2,4,7], [2,4,8], [2,5,7], [2,5,8], [2,5,9], [2,6,8], [2,6,9]
//! [3,5,7], [3,5,8], [3,5,9], [3,6,8], [3,6,9]
//! 和最小的下降路径是 [1,4,7]，所以答案是 12。
//! ```
//!  
//! 
//! 提示：
//! 
//! * 1 <= A.length == A[0].length <= 100
//! * -100 <= A[i][j] <= 100

pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(mut a: Vec<Vec<i32>>) -> i32 {
        if a.len() == 0 {
            return 0;
        }
        let mut ans = std::i32::MAX;
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                if i > 0 {
                    let mut m = a[i-1][j];
                    if j > 0 {
                        m = m.min(a[i-1][j-1]);
                    }
                    
                    if j + 1 < a[0].len() {
                        m = m.min(a[i-1][j+1]);
                    }
                    a[i][j] += m;
                }
                if i + 1 == a.len() {
                    ans = std::cmp::min(ans, a[i][j]);
                }
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::min_falling_path_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]));
}
