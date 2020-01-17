//! ## 两数之和 ##
//! [原题目地址](https://leetcode-cn.com/problems/two-sum/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
//! 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
//! ```
//! 示例:
//! 给定 nums = [2, 7, 11, 15], target = 9
//! 因为 nums[0] + nums[1] = 2 + 7 = 9
//! 所以返回 [0, 1]
//! ```

/// 实现
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    /// 遍历数组，判断 target - curr_val 值在HashMap中是否存在，不存在则插入当前值和下标(curr_val, index)，
    /// 存在则结束，返回两个下标
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();
        for (index, curr_value) in nums.iter().enumerate() {
            match map.get(&(target - curr_value)) {
                Some(i) => {
                    result.push(*i as i32);
                    result.push(index as i32);
                    break;
                }
                _ => map.insert(*curr_value, index)
            };
        }
        result
    }
}

fn main() {
    
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_two_sum(){
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 18), vec![1, 2]);
    }
}
