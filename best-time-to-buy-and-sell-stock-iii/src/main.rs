//! ## 买卖股票的最佳时机 III ## 
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iii/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
//! 
//! 设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。
//! 
//! 注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
//! 
//! 示例 1:
//! ```
//! 输入: [3,3,5,0,0,3,1,4]
//! 输出: 6
//! 解释: 在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
//!      随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。
//! ```
//! 示例 2:
//! ```
//! 输入: [1,2,3,4,5]
//! 输出: 4
//! 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。   
//!      注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。   
//!      因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
//! ```
//! 示例 3:
//! ```
//! 输入: [7,6,4,3,1] 
//! 输出: 0 
//! 解释: 在这个情况下, 没有交易完成, 所以最大利润为 0。
//! ```

pub struct Solution;

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut asset_i10 = 0;
        let mut asset_i11 = std::i32::MIN;
        let mut asset_i20 = 0;
        let mut asset_i21 = std::i32::MIN;
        for p in prices {
            asset_i20 = max(asset_i20, asset_i21 + p);
            asset_i21 = max(asset_i21, asset_i10 - p);
            asset_i10 = max(asset_i10, asset_i11 + p);
            asset_i11 = max(asset_i11, -p);
        }
        
        asset_i20
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(tes)]
mod test{
    use super::*;
    #[test]
    fn test_max_profit(){
        assert_eq!(Solution::max_profit(vec![3,3,5,0,0,3,1,4]), 6);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1] ), 0);
    }
}