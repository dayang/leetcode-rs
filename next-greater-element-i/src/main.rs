//! ## 下一个更大元素 I ## 
//! [原题目地址](https://leetcode-cn.com/problems/next-greater-element-i/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定两个没有重复元素的数组 nums1 和 nums2 ，其中nums1 是 nums2 的子集。找到 nums1 中每个元素在 nums2 中的下一个比其大的值。
//! 
//! nums1 中数字 x 的下一个更大元素是指 x 在 nums2 中对应位置的右边的第一个比 x 大的元素。如果不存在，对应位置输出-1。
//! 
//! 示例 1:
//! ```
//! 输入: nums1 = [4,1,2], nums2 = [1,3,4,2].
//! 输出: [-1,3,-1]
//! 解释:
//!     对于num1中的数字4，你无法在第二个数组中找到下一个更大的数字，因此输出 -1。
//!     对于num1中的数字1，第二个数组中数字1右边的下一个较大数字是 3。
//!     对于num1中的数字2，第二个数组中没有下一个更大的数字，因此输出 -1。
//! ```
//! 示例 2:
//! ```
//! 输入: nums1 = [2,4], nums2 = [1,2,3,4].
//! 输出: [3,-1]
//! 解释:
//!     对于num1中的数字2，第二个数组中的下一个较大数字是3。
//!     对于num1中的数字4，第二个数组中没有下一个更大的数字，因此输出 -1。
//! ```
//! 注意:
//! 
//! * nums1和nums2中所有元素是唯一的。
//! * nums1和nums2 的数组大小都不超过1000。

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums1.iter() {
            map.insert(*n, -1);
        }

        let mut ans = vec![];
        let mut stack = vec![];
        for n in nums2.iter() {
            while !stack.is_empty() && n > stack.last().unwrap() {
                let top = stack.pop().unwrap();
                map.entry(top).and_modify(|e| *e = *n);
            }
            stack.push(*n);
        }

        for n in nums1.iter() {
            ans.push(*map.get(n).unwrap());
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_next_greater_element(){
        assert_eq!(Solution::next_greater_element(vec![4,1,2], vec![1,3,4,2]), [-1,3,-1]);
        assert_eq!(Solution::next_greater_element(vec![2,4], vec![1,2,3,4]), [3,-1]);
    }
}