//! ## 买卖股票的最佳时机 IV ## 
//! [原题目地址](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
//! 
//! 设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。
//! 
//! 注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
//! 
//! 示例 1:
//! 
//! 输入: [2,4,1], k = 2
//! 输出: 2
//! 解释: 在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
//! 示例 2:
//! 
//! 输入: [3,2,6,5,0,3], k = 2
//! 输出: 7
//! 解释: 在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
//!      随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。

pub struct Solution;

use std::cmp::max;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let k = k as usize;
        // 相当于没有次数限制
        if k > prices.len() / 2 {
            let mut ans = 0;
            for i in 1..prices.len() {
                if prices[i] > prices[i-1] {
                    ans += prices[i] - prices[i-1];
                }
            }
            
            ans
        } else {
            let mut asset = vec![vec![vec![0;2];k + 1];prices.len()];
            for i in 0..prices.len() {
                for j in (1..=k).rev() {
                    if i == 0 {
                        asset[i][0][0] = 0;
                        asset[i][k][0] = 0;
                        for i in 0..prices.len() {
                            asset[i][0][1] = -prices[0];
                        }

                        for x in 0..=k {
                            asset[i][x][1] = -prices[0];
                        }
                        
                        
                        continue;
                    }

                    asset[i][j][0] = max(asset[i - 1][j][0], asset[i-1][j][1] + prices[i]);
                    asset[i][j][1] = max(asset[i-1][j][1], asset[i-1][j-1][0] - prices[i]);
                }
            }

            asset[prices.len() - 1][k][0]
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(2, vec![2,4,1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3,2,6,5,0,3]), 7);
        assert_eq!(Solution::max_profit(2, vec![1,2,4,2,5,7,2,4,9,0]), 13);
    }
}