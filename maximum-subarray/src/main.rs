//! ## 最大子序和 ## 
//! [原题目地址](https://leetcode-cn.com/problems/maximum-subarray/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
//! 
//! 示例:
//! 
//! 输入: [-2,1,-3,4,-1,2,1,-5,4],
//! 输出: 6
//! 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。

pub struct Solution;

impl Solution {
    /// 贪心思想，只有当子序列的和为正时才继续(如果子序列和为负，会使接下来的序列和变小)，否则从当前重新元素开始下一个子序列
    /// 用一个变量记录当前子序列和，并一直取最大值即为结果
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = std::i32::MIN;
        let mut sum = 0;
        for num in nums {
            if sum < 0 {
                sum = num;
            } else {
                sum += num;
            }
            res = std::cmp::max(res, sum);
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
