//! ## 机器人的运动范围 ## 
//! [原题目地址](https://leetcode-cn.com/problems/ji-qi-ren-de-yun-dong-fan-wei-lcof/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 地上有一个m行n列的方格，从坐标 [0,0] 到坐标 [m-1,n-1] 。一个机器人从坐标 [0, 0] 的格子开始移动，
//! 它每次可以向左、右、上、下移动一格（不能移动到方格外），也不能进入行坐标和列坐标的数位之和大于k的格子。
//! 例如，当k为18时，机器人能够进入方格 [35, 37] ，因为3+5+3+7=18。但它不能进入方格 [35, 38]，因为3+5+3+8=19。
//! 请问该机器人能够到达多少个格子？
//! 
//! 示例 1：
//! ```
//! 输入：m = 2, n = 3, k = 1
//! 输出：3
//! ```
//! 示例 2：
//! ```
//! 输入：m = 3, n = 1, k = 0
//! 输出：1
//! ```
//! 提示：
//! 
//! * 1 <= n,m <= 100
//! * 0 <= k <= 20

pub struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut vis = vec![vec![false;n as usize];m as usize];
        Self::dfs(0, 0, m, n, &mut vis, k)
    }

    fn dfs(i: i32, j: i32, m: i32, n: i32, vis: &mut Vec<Vec<bool>>, k: i32) -> i32 {
        vis[i as usize][j as usize] = true;
        let dir = vec![(0,1), (1,0), (0,-1), (-1, 0)];
        let mut sum = 0;
        for d in 0..4 {
            let x = i + dir[d].0;
            let y = j + dir[d].1;
            if x >= 0 && y >= 0 && x < m  && y < n
                    && !vis[x as usize][y as usize] && Self::calc_sum(x, y) <= k {
                        sum += Self::dfs(x, y, m, n, vis, k);
                    }
        }

        sum + 1
    }

    fn calc_sum(mut r: i32, mut c: i32) -> i32 {
        let mut sum = 0;
        while r != 0 || c !=0 {
            sum += r % 10;
            sum += c % 10;
            r /= 10;
            c /= 10;
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_moving_count(){
        assert_eq!(Solution::moving_count(2, 3, 1), 3);
        assert_eq!(Solution::moving_count(3, 1, 0), 1);
    }
}