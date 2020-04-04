//! ## 最佳买卖股票时机含冷冻期 ## 
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/) 难度：<b>中等</b>
//! ### 题目描述 ###

pub struct Solution;
use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut asset_0 = 0;
        let mut asset_1 = std::i32::MIN;
        let mut pre_0 = 0;
        for p in prices {
            let temp = asset_0;
            asset_0 = max(asset_0, asset_1 + p);
            asset_1 = max(asset_1, pre_0 - p);
            pre_0 = temp;
        }

        asset_0
    }
}

fn main() {
    println!("Hello, world!");
}
