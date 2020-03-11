//! ## 验证二叉搜索树 ##
//! [原题目地址](https://leetcode-cn.com/problems/validate-binary-search-tree/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个二叉树，判断其是否是一个有效的二叉搜索树。
//! 
//! 假设一个二叉搜索树具有如下特征：
//! 
//! * 节点的左子树只包含小于当前节点的数。
//! * 节点的右子树只包含大于当前节点的数。
//! * 所有左子树和右子树自身必须也是二叉搜索树。
//! 
//! 示例 1:
//! ```
//! 输入:
//!     2
//!    / \
//!   1   3
//! 输出: true
//! ```
//! 示例 2:
//! ```
//! 输入:
//!     5
//!    / \
//!   1   4
//!      / \
//!     3   6
//! 输出: false
//! 解释: 输入为: [5,1,4,null,null,3,6]。
//!      根节点的值为 5 ，但是其右子节点值为 4 。
//! ```


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

/// 直接中序遍历，判定严格单调递增即可
pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut pred: Option<i32> = None;

        let mut stack = vec![];
        let mut cur = root.clone();

        while !stack.is_empty() || cur.is_some() {
            while let Some(v) = cur {
                stack.push(v.clone());
                cur = v.borrow().left.clone();
            }

            let r = stack.pop().unwrap();
            if pred.is_some() && r.borrow().val <= pred.unwrap() {
                return false;
            }

            pred = Some(r.borrow().val);
            cur = r.borrow().right.clone();
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
