//! ## 统计有序矩阵中的负数 ##
//! [原题目地址](https://leetcode-cn.com/problems/count-negative-numbers-in-a-sorted-matrix/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给你一个 m * n 的矩阵 grid，矩阵中的元素无论是按行还是按列，都以非递增顺序排列。 
//! 
//! 请你统计并返回 grid 中 负数 的数目。
//! 
//! 示例 1：
//! ```
//! 输入：grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
//! 输出：8
//! 解释：矩阵中共有 8 个负数。
//! ```
//! 示例 2：
//! ```
//! 输入：grid = [[3,2],[1,0]]
//! 输出：0
//! ```
//! 示例 3：
//! ```
//! 输入：grid = [[1,-1],[-1,-1]]
//! 输出：3
//! ```
//! 示例 4：
//! ```
//! 输入：grid = [[-1]]
//! 输出：1
//! ```

pub struct Solution;

impl Solution {
    /// 直接暴力吧，O(n*m)
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for row in grid {
            for n in row {
                if n < 0 {
                    ans += 1;
                }
            }
        }

        ans
    }

    /// O(n*logm)
    /// 分治
    /// 找到某一行的第一个负数，则这个位置左上矩阵全正，右下矩阵全负，只需再次寻找左下和右上矩阵就行
    pub fn count_negatives_v2(grid: Vec<Vec<i32>>) -> i32 {
        Solution::solve(0, grid.len() - 1, 0, grid[0].len() - 1, &grid)
    }

    pub fn solve(l: usize, r: usize, L: usize, R: usize, grid: &Vec<Vec<i32>>) -> i32 {
        if r < l {
            return 0;
        }

        let mut ans = 0;
        let mid_row = (l + r) >> 1;
        let mut pos = -1;
        for i in L..=R {
            if grid[mid_row][i] < 0 {
                pos = i as i32;
                break;
            }
        }

        if pos == -1 {
            ans = Solution::solve(mid_row + 1, r, L, R, grid);
        } else {
            ans += grid[0].len() as i32 - pos;
            if mid_row > 0 {
                ans += Solution::solve(l, mid_row - 1, pos as usize, R, grid);
            }
            ans += Solution::solve(mid_row + 1, r, L, pos as usize, grid);
        }

        ans
    }

    /// 倒序遍历
    /// 根据矩阵性质，只要找到第i行第一个负数位置pos(i)就行
    /// 而因为矩阵按列也是非递增的，所以下一行的pos(i)位置也为负数，则下一行从pos(i)向前遍历就行,即pos(i+1) < pos(i)
    /// 总的来说时间等于遍历一行的时间
    /// O(m)
    pub fn count_negatives_v3(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut col = grid[0].len() as i32 - 1;
        for row in 0..grid.len() {
            while col >= 0 && grid[row][col as usize] < 0 {
                col -= 1;
            }
            ans += (grid[0].len() as i32 - col - 1) as i32;
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_negatives() {
        assert_eq!(Solution::count_negatives(vec![vec![1,-1],vec![-1,-1]]), 3);
        assert_eq!(Solution::count_negatives(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]), 8);

        assert_eq!(Solution::count_negatives_v2(vec![vec![1,-1],vec![-1,-1]]), 3);
        assert_eq!(Solution::count_negatives_v2(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]), 8);

        assert_eq!(Solution::count_negatives_v3(vec![vec![1,-1],vec![-1,-1]]), 3);
        assert_eq!(Solution::count_negatives_v3(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]), 8);
    }
}