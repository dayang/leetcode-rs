//! ## 比特位计数 ## 
//! [原题目地址](https://leetcode-cn.com/problems/counting-bits/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个非负整数 num。对于 0 ≤ i ≤ num 范围中的每个数字 i ，计算其二进制数中的 1 的数目并将它们作为数组返回。
//! 
//! 示例 1:
//! ```
//! 输入: 2
//! 输出: [0,1,1]
//! ```
//! 示例 2:
//! ```
//! 输入: 5
//! 输出: [0,1,1,2,1,2]
//! ```
//! 进阶:
//! 
//! * 给出时间复杂度为O(n*sizeof(integer))的解答非常容易。但你可以在线性时间O(n)内用一趟扫描做到吗？
//! * 要求算法的空间复杂度为O(n)。
//! * 你能进一步完善解法吗？要求在C++或任何其他语言中不使用任何内置函数（如 C++ 中的 __builtin_popcount）来执行此操作。

pub struct Solution;

impl Solution {
    /// 最低位，n 与 n >> 1 中1的个数相差n的最后一位是否为1
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut dp = vec![0_i32; (num as usize) + 1];
        dp[0] = 0;
        for i in 1..=num{
            dp[i as usize] = dp[(i >> 1) as usize] + (i & 1);
        }

        dp
    }

    /// 最后置位，n & (n-1) 是去掉n最后一位1，那么f(n) = f(n&(n-1)) + 1
    pub fn count_bits_v2(num: i32) -> Vec<i32> {
        let mut dp = vec![0_i32; (num as usize) + 1];
        dp[0] = 0;
        for i in 1..=num{
            dp[i as usize] = dp[(i & (i-1)) as usize] + 1;
        }

        dp
    }

    /// 直接暴力
    pub fn count_bits_v3(num: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..=num {
            let mut cnt = 0;
            let mut n = i;
            while n != 0 {
                cnt += 1;
                n &= n - 1;
            }
            ans.push(cnt);
        }
        ans
    }

    /// 最高位，
    /// * [0,1] 最左边加个1(2^1) -> [2,3] 
    /// * [0,1,2,3] 最左边加个1(2^2) -> [4,5,6,7] 
    /// * [0,1,2,3,4,5,6,7] 最左边加个1(2^3) -> [8,9,10,11,12,13,14,15] 
    /// * f(n + 2^m) = f(n) + 1, 2^m > n
    pub fn count_bits_v4(num: i32) -> Vec<i32> {
        let mut dp = vec![0_i32; (num as usize) + 1];
        let mut i = 0;
        // b=2^m
        let mut b = 1;
        // 计算[0, 2^m) + 2^m的1的个数
        while b <= num {
            // generate [b, 2b) or [b, num) from [0, b)
            while i < b && i + b <= num {
                dp[(i + b) as usize] = dp[i as usize] + 1;
                i += 1;
            }
            i = 0;   // reset i
            b <<= 1; // b = 2b = 2^(m+1)
        }

        dp
    }
}

fn main() {
    println!("Hello, world!");
}
