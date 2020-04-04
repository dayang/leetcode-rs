//! ## 买卖股票的最佳时机 II ## 
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
//! 
//! 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
//! 
//! 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
//! 
//! 示例 1:
//! ```
//! 输入: [7,1,5,3,6,4]
//! 输出: 7
//! 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
//!      随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。
//! ```
//! 示例 2:
//! ```
//! 输入: [1,2,3,4,5]
//! 输出: 4
//! 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
//!      注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
//!      因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
//! ```
//! 示例 3:
//! ```
//! 输入: [7,6,4,3,1]
//! 输出: 0
//! 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
//! ```

pub struct Solution;

use std::cmp::max;
impl Solution {
    // 动态规划
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut asset_0 = 0;
        let mut asset_1 = std::i32::MIN;
        for i in 0..prices.len() {
            let temp = max(asset_0, asset_1 + prices[i]);
            asset_1 = max(asset_1, asset_0 - prices[i]);
            asset_0 = temp;
        }
        asset_0
    }

    /// 贪心
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i-1] {
                ans += prices[i] - prices[i-1];
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
