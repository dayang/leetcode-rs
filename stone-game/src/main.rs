//! ## 石子游戏 ## 
//! [原题目地址](https://leetcode-cn.com/problems/stone-game/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 亚历克斯和李用几堆石子在做游戏。偶数堆石子排成一行，每堆都有正整数颗石子 piles[i] 。
//! 
//! 游戏以谁手中的石子最多来决出胜负。石子的总数是奇数，所以没有平局。
//! 
//! 亚历克斯和李轮流进行，亚历克斯先开始。 每回合，玩家从行的开始或结束处取走整堆石头。 这种情况一直持续到没有更多的石子堆为止，此时手中石子最多的玩家获胜。
//! 
//! 假设亚历克斯和李都发挥出最佳水平，当亚历克斯赢得比赛时返回 true ，当李赢得比赛时返回 false 。
//! 
//!  
//! 
//! 示例：
//! ```
//! 输入：[5,3,4,5]
//! 输出：true
//! 解释：
//! 亚历克斯先开始，只能拿前 5 颗或后 5 颗石子 。
//! 假设他取了前 5 颗，这一行就变成了 [3,4,5] 。
//! 如果李拿走前 3 颗，那么剩下的是 [4,5]，亚历克斯拿走后 5 颗赢得 10 分。
//! 如果李拿走后 5 颗，那么剩下的是 [3,4]，亚历克斯拿走后 4 颗赢得 9 分。
//! 这表明，取前 5 颗石子对亚历克斯来说是一个胜利的举动，所以我们返回 true 。
//! ```
//! 提示：
//! 
//! * 2 <= piles.length <= 500
//! * piles.length 是偶数。
//! * 1 <= piles[i] <= 500
//! * sum(piles) 是奇数。


pub struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![0_i32;n];n];
        for i in 0..n {
            dp[i][i] = piles[i];
        }

        let mut j;
        // 斜着遍历，offset是列相对行的偏移量
        for offset in 1..n {
            for i in 0..n{
                j = i + offset;
                if j == n {
                    break;
                }
                dp[i][j] = std::cmp::max(piles[i] - dp[i+1][j], piles[j] - dp[i][j-1]);
            }
        }

        println!("{}", dp[0][n-1]);

        dp[0][n-1] > 0
    }

    
}

fn main() {
    println!("Hello, world!");
    Solution::stone_game(vec![5,3,4,5]);
    Solution::stone_game(vec![5,3,4,1]);
    // Solution::stone_game_v2(vec![5,3,4,5]);
    // Solution::stone_game_v2(vec![5,3,4,1]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_stone_game() {
        assert_eq!(Solution::stone_game(vec![5,3,4,5]), true);
    }
}