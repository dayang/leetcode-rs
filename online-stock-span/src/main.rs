//! ## 股票价格跨度 ## 
//! [原题目地址](https://leetcode-cn.com/problems/online-stock-span/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 编写一个 StockSpanner 类，它收集某些股票的每日报价，并返回该股票当日价格的跨度。
//! 
//! 今天股票价格的跨度被定义为股票价格小于或等于今天价格的最大连续日数（从今天开始往回数，包括今天）。
//! 
//! 例如，如果未来7天股票的价格是 [100, 80, 60, 70, 60, 75, 85]，那么股票跨度将是 [1, 1, 1, 2, 1, 4, 6]。
//! 
//! 示例：
//! ```
//! 输入：["StockSpanner","next","next","next","next","next","next","next"], [[],[100],[80],[60],[70],[60],[75],[85]]
//! 输出：[null,1,1,1,2,1,4,6]
//! 解释：
//! 首先，初始化 S = StockSpanner()，然后：
//! S.next(100) 被调用并返回 1，
//! S.next(80) 被调用并返回 1，
//! S.next(60) 被调用并返回 1，
//! S.next(70) 被调用并返回 2，
//! S.next(60) 被调用并返回 1，
//! S.next(75) 被调用并返回 4，
//! S.next(85) 被调用并返回 6。
//! 
//! 注意 (例如) S.next(75) 返回 4，因为截至今天的最后 4 个价格
//! (包括今天的价格 75) 小于或等于今天的价格。
//! ```
//! 
//! 提示：
//! 
//! * 调用 StockSpanner.next(int price) 时，将有 1 <= price <= 10^5。
//! * 每个测试用例最多可以调用  10000 次 StockSpanner.next。
//! * 在所有测试用例中，最多调用 150000 次 StockSpanner.next。
//! * 此问题的总时间限制减少了 50%。


#![allow(dead_code)]
struct StockSpanner {
    stack: Vec<(i32, i32)>,
    next_index: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        StockSpanner{
            stack: vec![(0, 0)],
            next_index: 1,
        }
    }
    
    /// 单调栈
    fn next(&mut self, price: i32) -> i32 {
        
        while self.stack.len() > 1 && price >= self.stack.last().unwrap().1 {
            self.stack.pop();
        }
        let span = self.next_index - self.stack.last().unwrap().0;
        self.stack.push((self.next_index, price));
        self.next_index += 1;
        return span;
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_stock_spanner(){
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1);
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4);
        assert_eq!(stock_spanner.next(85), 6);
    }
}