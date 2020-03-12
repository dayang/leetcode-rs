//! ## 有多少小于当前数字的数字 ## 
//! [原题目地址](https://leetcode-cn.com/problems/how-many-numbers-are-smaller-than-the-current-number/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给你一个数组 nums，对于其中每个元素 nums[i]，请你统计数组中比它小的所有数字的数目。
//! 
//! 换而言之，对于每个 nums[i] 你必须计算出有效的 j 的数量，其中 j 满足 j != i 且 nums[j] < nums[i] 。
//! 
//! 以数组形式返回答案。
//! 
//! 示例 1：
//! ```
//! 输入：nums = [8,1,2,2,3]
//! 输出：[4,0,1,1,3]
//! 解释： 
//! 对于 nums[0]=8 存在四个比它小的数字：（1，2，2 和 3）。 
//! 对于 nums[1]=1 不存在比它小的数字。
//! 对于 nums[2]=2 存在一个比它小的数字：（1）。 
//! 对于 nums[3]=2 存在一个比它小的数字：（1）。 
//! 对于 nums[4]=3 存在三个比它小的数字：（1，2 和 2）。
//! ```
//! 示例 2：
//! ```
//! 输入：nums = [6,5,4,8]
//! 输出：[2,1,0,3]
//! ```
//! 示例 3：
//! ```
//! 输入：nums = [7,7,7,7]
//! 输出：[0,0,0,0]
//! ```
//! 
//! 提示：
//! 
//! * 2 <= nums.length <= 500
//! * 0 <= nums[i] <= 100

pub struct Solution;

impl Solution {
    /// 类似桶排序
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0;101];
        for i in &nums {
            cnt[*i as usize] += 1;
        }

        for i in 1..cnt.len() {
            cnt[i as usize] += cnt[i-1];
        }

        let mut res = vec![];
        for i in &nums {
            if *i > 0 {
                res.push(cnt[(*i as usize) - 1]);
            } else {
                res.push(0);
            }
        }
        res
    }

    /// 暴力
    pub fn smaller_numbers_than_current_v2(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..nums.len() {
            let mut n = 0;
            for j in 0..nums.len() {
                if i !=j && nums[i] > nums[j] {
                    n = n + 1;
                }
            }
            res.push(n);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_smaller_numbers_than_current() {
        assert_eq!(Solution::smaller_numbers_than_current(vec![8,1,2,2,3]), vec![4,0,1,1,3]);
        assert_eq!(Solution::smaller_numbers_than_current_v2(vec![8,1,2,2,3]), vec![4,0,1,1,3]);
    }
}