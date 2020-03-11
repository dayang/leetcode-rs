//! ##  恢复二叉搜索树 ##
//! [原题目地址](https://leetcode-cn.com/problems/recover-binary-search-tree/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 二叉搜索树中的两个节点被错误地交换。
//!
//! 请在不改变其结构的情况下，恢复这棵树。
//! 
//! 示例 1:
//! ```
//! 输入: [1,3,null,null,2]
//! 
//!    1
//!   /
//!  3
//!   \
//!    2
//! 
//! 输出: [3,1,null,null,2]
//! 
//!    3
//!   /
//!  1
//!   \
//!    2
//! ```
//! 
//! 示例 2:
//! ```
//! 输入: [3,1,4,null,null,2]
//! 
//!   3
//!  / \
//! 1   4
//!    /
//!   2
//! 
//! 输出: [2,1,4,null,null,3]
//! 
//!   2
//!  / \
//! 1   4
//!    /
//!   3
//! ```
//! 进阶:
//! 
//! * 使用 O(n) 空间复杂度的解法很容易实现。
//! * 你能想出一个只使用常数空间的解决方案吗？

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

/// * 构造中序遍历序列。
/// * 在几乎排序的数组中查找两个交换的元素。
/// * 交换两个节点的值。
/// 前两步可以合在一起做
pub struct Solution;

impl Solution {
    /// 递归
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let x: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        let y: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        let pred: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        Solution::travel(root.clone(), x.clone(), y.clone(), pred.clone());
        Solution::swap(x, y);
    }

    pub fn travel(root: Option<Rc<RefCell<TreeNode>>>, x: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>>, y: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>>, pred: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>>){
        if root.is_none(){
            return;
        }

        Solution::travel(root.as_ref().unwrap().borrow().left.clone(), x.clone(), y.clone() ,pred.clone());

        if pred.borrow().is_some() && root.as_ref().unwrap().borrow().val < pred.borrow().as_ref().unwrap().borrow().val {
            *y.borrow_mut() = Some(root.as_ref().unwrap().clone());
            if x.borrow().is_none() {
                *x.borrow_mut() = Some(pred.borrow().as_ref().unwrap().clone());
            } else {
                return;
            }
        }

        *pred.borrow_mut() = Some(root.as_ref().unwrap().clone());
        Solution::travel(root.as_ref().unwrap().borrow().right.clone(), x, y ,pred);
    }

    pub fn swap(x: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>>, y: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>>) {
        let temp = x.borrow().as_ref().unwrap().borrow().val;
        x.borrow().as_ref().unwrap().borrow_mut().val = y.borrow().as_ref().unwrap().borrow().val;
        y.borrow().as_ref().unwrap().borrow_mut().val = temp;
    }

    /// 迭代
    pub fn recover_tree_v2(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // 保存第一个错误的节点
        let x: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        // 保存第二个错误的节点，
        let y: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        // 当前遍历节点的前一个节点，遍历时，第一次出现当前节点值 < 前一节点值时，则第一个错误节点为前一节点,
        // 第二次出现时，第二个错误节点是当前节点
        let pred: Rc<RefCell<Option<Rc<RefCell<TreeNode>>>>> = Rc::new(RefCell::new(None));
        
        let mut stack = vec![];
        let mut cur = root.clone();

        while !stack.is_empty() || cur.is_some() {
            while let Some(v) = cur {
                stack.push(v.clone());
                cur = v.borrow().left.clone();
            }

            let top = stack.pop().unwrap();
            if pred.borrow().is_some() && top.borrow().val < pred.borrow().as_ref().unwrap().borrow().val {
                *y.borrow_mut() = Some(top.clone());
                if x.borrow().is_none() {
                    *x.borrow_mut() = Some(pred.borrow().as_ref().unwrap().clone());
                } else {
                    break;
                }
            }

            *pred.borrow_mut() = Some(top.clone());      
            cur = top.borrow().right.clone();
        }

        Solution::swap(x, y);
    }

}

fn main() {
    println!("Hello, world!");
}
