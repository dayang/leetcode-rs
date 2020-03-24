//! ## 买卖股票的最佳时机 ## 
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
//! 
//! 如果你最多只允许完成一笔交易（即买入和卖出一支股票一次），设计一个算法来计算你所能获取的最大利润。
//! 
//! 注意：你不能在买入股票前卖出股票。
//! 
//!  
//! 
//! 示例 1:
//! ```
//! 输入: [7,1,5,3,6,4]
//! 输出: 5
//! 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
//!      注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格。
//! ```
//! 示例 2:
//! ```
//! 输入: [7,6,4,3,1]
//! 输出: 0
//! 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
//! ```

pub struct Solution;

use std::cmp::{min, max};
impl Solution {
    /// 遍历一遍，记录最低价格，用当前价格减最低价格得到利润，取最大利润
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut min_price = prices[0];
        let mut profit = 0;
        for i in 1..prices.len() {
            min_price = min(prices[i], min_price);
            profit = max(profit, prices[i] - min_price);
        }

        profit
    }
}

fn main() {
    println!("Hello, world!");
}
