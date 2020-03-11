//! ## 验证二叉搜索树 ##
//! > 动态规划
//! [原题目地址](https://leetcode-cn.com/problems/unique-binary-search-trees/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个整数 n，求以 1 ... n 为节点组成的二叉搜索树有多少种？
//! 
//! 示例:
//! ```
//! 输入: 3
//! 输出: 5
//! 解释:
//! 给定 n = 3, 一共有 5 种不同结构的二叉搜索树:
//! 
//!    1         3     3      2      1
//!     \       /     /      / \      \
//!      3     2     1      1   3      2
//!     /     /       \                 \
//!    2     1         2                 3
//! ```

use std::collections::HashMap;
pub struct Solution;

impl Solution {
    /// 典型的动态规划
    /// 递归解法
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = HashMap::new();

        Solution::helper(n, &mut dp)
    }

    pub fn helper(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        match n {
            0 => return 1,
            1 => return 1,
            _ => ()
        };

        if dp.contains_key(&n) {
            return *dp.get(&n).unwrap();
        }

        let mut result = 0;
        for i in 0..n {
            result += Solution::helper(i, dp) * Solution::helper(n - i - 1, dp);
        }

        dp.insert(n, result);

        result
    }

    /// 非递归解法
    pub fn num_trees_v2(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            for j in 0..i {
                dp[i as usize] += dp[j as usize] * dp[(i -j -1) as usize];
            }
        }

        dp[n as usize]
    } 
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_num_trees(){
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees_v2(3), 5);
    }
}