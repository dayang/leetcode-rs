//! ## 有效数字 ## 
//! [原题目地址](https://leetcode-cn.com/problems/valid-number/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 验证给定的字符串是否可以解释为十进制数字。
//! 
//! 例如:
//! 
//! "0" => true
//! " 0.1 " => true
//! "abc" => false
//! "1 a" => false
//! "2e10" => true
//! " -90e3   " => true
//! " 1e" => false
//! "e3" => false
//! " 6e-1" => true
//! " 99e2.5 " => false
//! "53.5e93" => true
//! " --6 " => false
//! "-+3" => false
//! "95a54e53" => false
//! 
//! 说明: 我们有意将问题陈述地比较模糊。在实现代码之前，你应当事先思考所有可能的情况。这里给出一份可能存在于有效十进制数字中的字符列表：
//! 
//! 数字 0-9
//! 指数 - "e"
//! 正/负号 - "+"/"-"
//! 小数点 - "."
//! 当然，在输入中，这些字符的上下文也很重要。
//! 
//! 
//! ### 解法思路：###
//! 使用有限状态机DFA，状态转换表格如下
//! 
//! .3 或3. 也认为是正确的数字
//! 
//! |state/input  |  e   |     +/-	  |     digit(0..9) | dot(.)   |  other|
//! |--|--|--|--|--|--|
//! |start      |   end     |  signed 	|   number    |  dot       | end|
//! |signed     |   end     |  end	    |   number    |  dot       | end|
//! |dot        |   end     |  end      |  float      |  end       | end|
//! |number	    |   e       | end	    |   number    |  float_dot | end|
//! |float      |   e       |  end      |  float      |  end       | end|
//! |e          |   end     |  e_signed |  e_number   |  end       | end|
//! |e_signed   |   end     |  end      |  e_number   |  end       | end|
//! |e_number   |   end     |  end      |  e_number   |  end       | end|
//! |end	    |   end     |  end	    |    end	  |  end       | end|
//! 
//! 先去掉首尾空格，减少两个状态，最终状态为number/e_number/float/float_dot时是有效数字,end时提前结束

pub struct Solution;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Start,
    Signed,
    Dot,
    Number,
    Float,
    E,
    ESigned,
    ENumber,
    End,
}

use std::collections::HashMap;
use State::*;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state_table = HashMap::new();
        state_table.insert(Start,   vec![End, Signed, Number, Dot, End]);
        state_table.insert(Signed,  vec![End, End, Number, Dot, End]);
        state_table.insert(Dot,     vec![End, End, Float, End, End]);
        state_table.insert(Number,  vec![E, End, Number, Float, End]);
        state_table.insert(Float,   vec![E, End, Float, End, End]);
        state_table.insert(E,       vec![End, ESigned, ENumber, End, End]);
        state_table.insert(ESigned, vec![End, End, ENumber, End, End]);
        state_table.insert(ENumber, vec![End, End, ENumber, End, End]);
        // state_table.insert(End,     vec![End, End, End, End, End]);

        let mut state = Start;
        for c in s.trim().as_bytes() {
            let input_col = match c {
                b'e' => 0,
                b'+' | b'-' => 1,
                n if n.is_ascii_digit() => 2,
                b'.' => 3,
                _ => 4
            };

            state = state_table.get(&state).unwrap()[input_col];
            if state == End {
                return false;
            }
        }

        state == ENumber || state == Number || state == Float
    }
}

fn main() {
    println!("{}", -10e-01);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::is_number("42".to_string()), true);
        assert_eq!(Solution::is_number("    -42".to_string()), true);
        assert_eq!(Solution::is_number("   - 32".to_string()), false);
        assert_eq!(Solution::is_number(" 4193 with words".to_string()), false);
        assert_eq!(Solution::is_number("-91283472332       ".to_string()), true);
        assert_eq!(Solution::is_number("-3e8 ".to_string()), true);
        assert_eq!(Solution::is_number("  -3e7 a".to_string()), false);
        assert_eq!(Solution::is_number("   3e-7   ".to_string()), true);
        assert_eq!(Solution::is_number("0.89e-7".to_string()), true);
        assert_eq!(Solution::is_number("3e0.7".to_string()), false);
        assert_eq!(Solution::is_number("--6".to_string()), false);
        assert_eq!(Solution::is_number(" 6e".to_string()), false);
        assert_eq!(Solution::is_number("e8".to_string()), false);
        assert_eq!(Solution::is_number(" abc".to_string()), false);
        assert_eq!(Solution::is_number(" .12".to_string()), true);
        assert_eq!(Solution::is_number(" 3.".to_string()), true);
        assert_eq!(Solution::is_number(" 3.e3".to_string()), true);
        assert_eq!(Solution::is_number(" .3e5".to_string()), true);
    }
}