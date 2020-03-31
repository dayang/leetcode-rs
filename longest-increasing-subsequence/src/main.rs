//! ## 最长上升子序列 ## 
//! [原题目地址](https://leetcode-cn.com/problems/longest-increasing-subsequence/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个无序的整数数组，找到其中最长上升子序列的长度。
//! 
//! 示例:
//! ```
//! 输入: [10,9,2,5,3,7,101,18]
//! 输出: 4 
//! 解释: 最长的上升子序列是 [2,3,7,101]，它的长度是 4。
//! ```
//! 说明:
//! 
//! * 可能会有多种最长上升子序列的组合，你只需要输出对应的长度即可。
//! * 你算法的时间复杂度应该为 O(n2) 。
//! 
//! *进阶*: 你能将算法的时间复杂度降低到 O(n log n) 吗?

pub struct Solution;

use std::cmp::max;
impl Solution {
    /// 动态规划 时间复杂度O(n^2)
    /// 设f(k)为以nums[k]结尾的最长上升子序列长度，
    /// 则f(k) = max(f[i] if nums[k] > nums[i]) + 1, i in [0..k-1]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = vec![1;nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
            ans = max(ans, dp[i]);
        }

        ans
    }

    /// 优化， O(n*lg(n))
    /// 用一个数组lens保存长度为len的最长上升子序列的最后一个数字的最小值，该数组单调递增。
    /// 
    /// 则只需要查找下一个数字在lens数组中应该插入的位置 i，就用该数字将该位置上的数字替换掉
    /// 
    pub fn length_of_lis_v2(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut lens = vec![nums[0]];
        for num in nums {
            if num > lens[lens.len() - 1] {
                lens.push(num);
            } else {
                let k = lens.binary_search(&num).unwrap_or_else(|i| i);
                lens[k] = num;
            }
        }

        lens.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
