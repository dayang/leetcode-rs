//! ##  非递减数列 ##
//! [原题目地址](https://leetcode-cn.com/problems/non-decreasing-array/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个长度为 n 的整数数组，你的任务是判断在最多改变 1 个元素的情况下，该数组能否变成一个非递减数列。
//! 
//! 我们是这样定义一个非递减数列的： 对于数组中所有的 i (1 <= i < n)，满足 array[i] <= array[i + 1]。
//! 
//! 示例 1:
//! ```
//! 输入: [4,2,3]
//! 输出: True
//! 解释: 你可以通过把第一个4变成1来使得它成为一个非递减数列。
//! ```
//! 示例 2:
//! ```
//! 输入: [4,2,1]
//! 输出: False
//! 解释: 你不能在只改变一个元素的情况下将其变为非递减数列。
//! 说明:  n 的范围为 [1, 10,000]。
//! ```
//! 

pub struct Solution;

impl Solution {
    /// 虽然题目难度是简单，但是一开始还是没想明白，看了别人的题解才想明白。
    /// 
    /// 首先，我们遍历找到第一个逆序对 `a[i] > a[i + 1]`, 此时`a[0]~a[i]`非递减的。
    /// 如果要让 `a[0] ~ a[i + 1]` 非递减的话，
    /// 要么我们修改`a[i]`，要么修改`a[i+1]`。这时分情况：
    /// * `a[i + 1] >= a[i - 1]`，则只需让`a[i] = a[i + 1]`即可
    /// * 如果`a[i + 1] < a[i - 1]`，则让`a[i + 1] = a[i]`即可
    /// 
    /// 如果i = 0，则直接让a[i] = a[i + 1]
    /// 
    /// 此时找到第一个必须修改的元素，接下来接着遍历，看是否还有逆序对，如果有则还有需要修改
    /// 的元素，返回false，如果没有返回true
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut n = 0;
        for i in 0..nums.len()-1 {
            if nums[i] > nums[i + 1] {
                n += 1;
                if n >= 2 {
                    return false;
                }

                if i == 0 || nums[i+1] > nums[i-1] {
                    nums[i] = nums[i+1];
                } else {
                    nums[i+1] = nums[i];
                }
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_check_possibility() {
        assert_eq!(Solution::check_possibility(vec![4,2,3]), true);
        assert_eq!(Solution::check_possibility(vec![1,2,2,1,3]), true);
        assert_eq!(Solution::check_possibility(vec![4,2,1]), false);
    }
}