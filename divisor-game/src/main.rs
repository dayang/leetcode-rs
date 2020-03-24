//! ## 除数博弈 ## 
//! [原题目地址](https://leetcode-cn.com/problems/divisor-game/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 爱丽丝和鲍勃一起玩游戏，他们轮流行动。爱丽丝先手开局。
//! 
//! 最初，黑板上有一个数字 N 。在每个玩家的回合，玩家需要执行以下操作：
//! 
//! 选出任一 x，满足 0 < x < N 且 N % x == 0 。
//! 用 N - x 替换黑板上的数字 N 。
//! 如果玩家无法执行这些操作，就会输掉游戏。
//! 
//! 只有在爱丽丝在游戏中取得胜利时才返回 True，否则返回 false。假设两个玩家都以最佳状态参与游戏。
//! 
//!  
//! 
//! 示例 1：
//! ```
//! 输入：2
//! 输出：true
//! 解释：爱丽丝选择 1，鲍勃无法进行操作。
//! ```
//! 示例 2：
//! ```
//! 输入：3
//! 输出：false
//! 解释：爱丽丝选择 1，鲍勃也选择 1，然后爱丽丝无法进行操作。
//! ```
//! 
//! 提示：
//! 
//! * 1 <= N <= 1000

pub struct Solution;

impl Solution {
    /// 简单的动态规划
    /// 如果轮到某人，只要黑板上的N有一个因子x使得 (N - x) 不能赢(只要选择x，对手必输)，那么该人就赢 ，否则该人必输
    /// 用dp数组将n之前的情况记录下来
    pub fn divisor_game(n: i32) -> bool {
        let mut dp  = vec![false];
        dp.push(false);
        dp.push(true);
        for i in 3..=n as usize {
            let mut win = false;
            for j in 1..i {
                if i % j == 0 && !dp[i - j] {
                    win = true;
                    break;
                }
            }
            dp.push(win);
        }

        return dp[n as usize];
    }
}

fn main() {
    println!("Hello, world!");
}
