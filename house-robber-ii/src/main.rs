//! ## 打家劫舍II ## 
//! [原题目地址](https://leetcode-cn.com/problems/house-robber-ii/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都围成一圈，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
//! 
//! 给定一个代表每个房屋存放金额的非负整数数组，计算你在不触动警报装置的情况下，能够偷窃到的最高金额。
//! 
//! 示例 1:
//! ```
//! 输入: [2,3,2]
//! 输出: 3
//! 解释: 你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
//! ```
//! 示例 2:
//! ```
//! 输入: [1,2,3,1]
//! 输出: 4
//! 解释: 你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
//!      偷窃到的最高金额 = 1 + 3 = 4 。
//! ```

pub struct Solution;

impl Solution {
    /// 由于收尾两家是相邻的，不能同时抢，所以做两次动态规划化，取大。
    /// [0,n-2](必不抢最后一家)和[1,n-1](必不抢第一家)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        std::cmp::max(Solution::rob_range(&nums, 0, n - 2), Solution::rob_range(&nums, 1, n - 1))
    }

    pub fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut m1 = 0;
        let mut m2 = 0;
        for i in start..=end{
            let temp = std::cmp::max(nums[i] + m1, m2);
            m1 = m2;
            m2 = temp;
        }
        m2
    }
}

fn main() {
    println!("Hello, world!");
}
