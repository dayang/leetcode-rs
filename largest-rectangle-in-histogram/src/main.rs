//! ## 柱状图中最大的矩形 ## 
//! [原题目地址](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
//! 
//! 求在该柱状图中，能够勾勒出来的矩形的最大面积。
//! 示例:
//! ```
//! 输入: [2,1,5,6,2,3]
//! 输出: 10
//! ```

pub struct Solution;

impl Solution {
    /// 单调栈技巧，O(n)
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        // 左右哨兵
        let mut stack = vec![-1];
        heights.push(0);

        let mut ans = 0;

        for (i, v) in heights.iter().enumerate() {
            if stack.len() == 1 || v >= &heights[*stack.last().unwrap() as usize] {
                stack.push(i as i32);
            } else {
                while stack.len() > 1 && v < &heights[*stack.last().unwrap() as usize] {
                    let height = heights[stack.pop().unwrap() as usize];
                    ans = ans.max(height * (i as i32 - stack.last().unwrap() - 1));
                }
                stack.push(i as i32);
            }
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_largest_rectangle_area(){
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
        //assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
    }
}