//! ## 递增子序列 ## 
//! [原题目地址](https://leetcode-cn.com/problems/increasing-subsequences/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个整型数组, 你的任务是找到所有该数组的递增子序列，递增子序列的长度至少是2。
//! 
//! 示例:
//! ```
//! 输入: [4, 6, 7, 7]
//! 输出: [[4, 6], [4, 7], [4, 6, 7], [4, 6, 7, 7], [6, 7], [6, 7, 7], [7,7], [4,7,7]]
//! ```
//! 说明:
//! 
//! * 给定数组的长度不会超过15。
//! * 数组中的整数范围是 [-100,100]。
//! * 给定数组中可能包含重复数字，相等的数字应该被视为递增的一种情况。

use std::collections::HashSet;
pub struct Solution;

impl Solution {
    // dfs
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans : Vec<Vec<i32>> = vec![];
        Self::dfs(&nums, 0, &mut vec![], &mut ans);

        ans
    }

    pub fn dfs(nums: &Vec<i32>, offset: usize, seq: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        let mut set = HashSet::new();
        for i in offset..nums.len() {
            if ((seq.len() == 0 || nums[i] >= *seq.last().unwrap()) && set.insert(nums[i])) {
                seq.push(nums[i]);
                if seq.len() >= 2 {
                    res.push(seq.clone());
                }
                
                Self::dfs(nums, i + 1, seq, res);

                seq.pop();
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::find_subsequences(vec![1,2,3,4,5,6,7,8,9,10,1,1,1,1,1]));
}
