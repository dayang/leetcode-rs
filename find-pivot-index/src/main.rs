//! ## 寻找数组的中心索引 ## 
//! [原题目地址](https://leetcode-cn.com/problems/find-pivot-index/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个整数类型的数组 nums，请编写一个能够返回数组“中心索引”的方法。
//! 
//! 我们是这样定义数组中心索引的：数组中心索引的左侧所有元素相加的和等于右侧所有元素相加的和。
//! 
//! 如果数组不存在中心索引，那么我们应该返回 -1。如果数组有多个中心索引，那么我们应该返回最靠近左边的那一个。
//! 
//! 示例 1:
//! ```
//! 输入: 
//! nums = [1, 7, 3, 6, 5, 6]
//! 输出: 3
//! 解释: 
//! 索引3 (nums[3] = 6) 的左侧数之和(1 + 7 + 3 = 11)，与右侧数之和(5 + 6 = 11)相等。
//! 同时, 3 也是第一个符合要求的中心索引。
//! ```
//! 示例 2:
//! ```
//! 输入: 
//! nums = [1, 2, 3]
//! 输出: -1
//! 解释: 
//! 数组中不存在满足此条件的中心索引。
//! ```
//! 说明:
//! 
//! * nums 的长度范围为 [0, 10000]。
//! * 任何一个 nums[i] 将会是一个范围在 [-1000, 1000]的整数。

pub struct Solution;

impl Solution {
    /// 前缀和，三种不同版本，简洁程度有所不同
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sl = 0;
        let mut sr = 0;
        for i in (0..nums.len()).rev() {
            sr += nums[i];
        }        
        for i in 0..nums.len() {
            sl += nums[i];
            if sl == sr { return i as i32; }
            sr -= nums[i];            
        }
        -1
    }

    /// 
    pub fn pivot_index_v2(nums: Vec<i32>) -> i32 {
        let mut right_sum = vec![0_i32; nums.len()];
        for i in (0..nums.len()).rev() {
            if i + 1 < nums.len() {
                right_sum[i] = nums[i + 1] + right_sum[i + 1];
            } 
        }

        let mut left_sum = 0;
        for i in 0..nums.len() {
            if i > 0 {
                left_sum += nums[i - 1];
            }
            
            if left_sum == right_sum[i] {
                return i as i32;
            }
        }

        -1
    }

    pub fn pivot_index_v3(nums: Vec<i32>) -> i32 {
        let mut sl = 0;
        let sum : i32 = nums.iter().sum();     
        for i in 0..nums.len() {
            if sl == sum - nums[i] - sl { return i as i32; } 
            sl += nums[i];           
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
