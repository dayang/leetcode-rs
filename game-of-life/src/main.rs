//! ## 生命游戏 ## 
//! [原题目地址](https://leetcode-cn.com/problems/game-of-life/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 根据 百度百科 ，生命游戏，简称为生命，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。
//! 
//! 给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态：1 即为活细胞（live），或 0 即为死细胞（dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：
//! 
//! 如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
//! 如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
//! 如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
//! 如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
//! 根据当前状态，写一个函数来计算面板上所有细胞的下一个（一次更新后的）状态。下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。
//! 
//!  
//! 
//! 示例：
//! ```
//! 输入： 
//! [
//!   [0,1,0],
//!   [0,0,1],
//!   [1,1,1],
//!   [0,0,0]
//! ]
//! 输出：
//! [
//!   [0,0,0],
//!   [1,0,1],
//!   [0,1,1],
//!   [0,1,0]
//! ]
//! ```
//! 
//! 进阶：
//! 
//! * 你可以使用原地算法解决本题吗？请注意，面板上所有格子需要同时被更新：你不能先更新某些格子，然后使用它们的更新后的值再更新其他格子。

pub struct Solution;

impl Solution {
    // 扩展每个格子的状态，使之包含现在的状态和下一个状态，
    // 题目中只有状态只有0和1，一位二进制，第一次遍历可以扩展为2位二进制，低位位表示现在状态，高位表示下一轮状态。
    // 第二次遍历，每个格子赋值为高位
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let d = vec![1, 0, -1];
        for row in 0..board.len() as i32 {
            for col in 0..board[0].len() as i32 {
                let mut lives = 0;
                // 遍历8个方向
                for x in &d {
                    for y in &d {
                        if !(*x == 0 && *y == 0) {
                            if row + x >= 0 && row + x < board.len() as i32
                                && col + y >= 0 && col + y < board[0].len() as i32 {
                                    lives += board[(row + x) as usize][(col + y) as usize] & 1;
                                }
                        }
                    }
                }

                // < 2 或 > 3 高位为0，不用判断，
                // == 3 必活
                // == 2 与本轮状态相同
                if lives == 3 {
                    board[row as usize][col as usize] |= 0b10;
                } else if lives == 2 {
                    board[row as usize][col as usize] |= board[row as usize][col as usize] << 1;
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                board[row][col] = board[row][col] >> 1;
            }
        }
    }
}

fn main() {
    let mut board = vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]];
    Solution::game_of_life(&mut board);
    println!("{:?}", board);
}
