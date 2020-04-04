//! ##  买卖股票的最佳时机含手续费 ##
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。
//! 
//! 你可以无限次地完成交易，但是你每次交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
//! 
//! 返回获得利润的最大值。
//! 
//! 示例 1:
//! ```
//! 输入: prices = [1, 3, 2, 8, 4, 9], fee = 2
//! 输出: 8
//! 解释: 能够达到的最大利润:  
//! 在此处买入 prices[0] = 1
//! 在此处卖出 prices[3] = 8
//! 在此处买入 prices[4] = 4
//! 在此处卖出 prices[5] = 9
//! 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
//! ```
//! 注意:
//! 
//! * 0 < prices.length <= 50000.
//! * 0 < prices[i] < 50000.
//! * 0 <= fee < 50000.

pub struct Solution;

use std::cmp::max;
impl Solution {
    // 暴力穷举任意两点之间最大利润，超时 O(n^3)
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = vec![vec![0;prices.len()];prices.len()];

        for len in 1..prices.len() {
            for i in 0..prices.len() {
                let end = i + len;
                if end == prices.len() {
                    break;
                }

                let mut mf = 0;
                for k in i+1..end{
                    mf = max(mf, dp[i][k] + dp[k][end]);
                }
                mf = max(mf, prices[end] - prices[i] - fee);
                dp[i][end] = mf;
            }
        }

        dp[0][prices.len() - 1]
    }

    // 动态规划
    pub fn max_profit_v2(prices: Vec<i32>, fee: i32) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut asset = vec![vec![0;2];prices.len()];
        for i in 0..prices.len() {
            if i == 0 {
                asset[i][0] = 0;
                asset[i][1] = -prices[0] - fee;
                continue;
            }

            asset[i][0] = max(asset[i-1][0], asset[i-1][1] + prices[i]);
            asset[i][1] = max(asset[i-1][1], asset[i-1][0] - prices[i] - fee);
        }

        asset[prices.len() - 1][0]
    }

    // 动态规划优化
    pub fn max_profit_v3(prices: Vec<i32>, fee: i32) -> i32 {
        if prices.len() == 0 {
            return 0;
        }

        let mut asset_0 = 0;
        let mut asset_1 = -prices[0] - fee;
        for i in 1..prices.len() {
            let temp = max(asset_0, asset_1 + prices[i]);
            asset_1 = max(asset_1, asset_0 - prices[i] - fee);
            asset_0 = temp;
        }

        asset_0
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 8));
}
