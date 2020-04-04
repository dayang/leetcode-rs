//! ##  接雨水 ##
//! [原题目地址](https://leetcode-cn.com/problems/trapping-rain-water/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
//! 
//! 示例:
//! 
//! 输入: [0,1,0,2,1,0,1,3,2,1,2,1]
//! 输出: 6

pub struct Solution;
use std::cmp::{max, min};
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let mut right = vec![0;height.len()];
        for i in (0..height.len() - 1).rev() {
            right[i] = max(height[i + 1], right[i + 1]);
        }

        let mut ans = 0;
        let mut left_max = 0;
        for i in 0..height.len() {
            if min(left_max, right[i]) > height[i] {
                ans += min(left_max, right[i]) - height[i];
            }

            left_max = max(left_max, height[i]);
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::trap(vec![2, 1, 0, 2]));
}
