//! ## 每日温度 ## 
//! [原题目地址](https://leetcode-cn.com/problems/daily-temperatures/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 根据每日 气温 列表，请重新生成一个列表，对应位置的输出是需要再等待多久温度才会升高超过该日的天数。如果之后都不会升高，请在该位置用 0 来代替。
// !
// !例如，给定一个列表 temperatures = [73, 74, 75, 71, 69, 72, 76, 73]，你的输出应该是 [1, 1, 4, 2, 1, 1, 0, 0]。
// !
// !提示：气温 列表长度的范围是 [1, 30000]。每个气温的值的均为华氏度，都是在 [30, 100] 范围内的整数。


pub struct Solution;

impl Solution {
    /// 用栈保存下标，如果栈顶下标i对应的温度比当前下标curr温度低，则ans[i] = curr - i,
    /// 栈顶下标出栈，知道栈顶下标对应的温度不比当前温度低，当前下标入栈
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0_i32;t.len()];
        let mut stack : Vec<usize> = vec![];
        for i in 0..t.len(){
            while let Some(index) = stack.last() {
                if t[i] > t[*index] {
                    ans[*index] = (i - *index) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
