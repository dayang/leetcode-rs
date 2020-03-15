//! ## 相同的树 ##
//! [原题目地址](https://leetcode-cn.com/problems/same-tree/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定两个二叉树，编写一个函数来检验它们是否相同。
//!
//!如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
//! 
//! 示例 1:
//! ```
//! 输入:
//!  1         1
//! / \       / \
//! 2   3     2   3
//! 
//! [1,2,3],   [1,2,3]
//! 
//! 输出: true
//! ```
//! 
//! 示例 2:
//! ```
//! 输入:      
//!  1          1
//! /           \
//! 2             2
//! 
//! [1,2],     [1,null,2]
//! 
//! 输出: false
//! ```
//! 
//! 示例 3:
//! ```
//! 输入:      
//!  1         1
//! / \       / \
//! 2   1     1   2
//! 
//! [1,2,1],   [1,1,2]
//! 
//! 输出: false
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

pub struct Solution;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(v1), Some(v2)) => {
                return (v1.borrow().val == v1.borrow().val) && 
                    Solution::is_same_tree(v1.borrow().left.clone(), v2.borrow().left.clone()) && 
                    Solution::is_same_tree(v1.borrow().right.clone(), v2.borrow().right.clone());
            },
            (None, None) => return true,
            (_, _) => return false
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_same_tree() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(3)));

        root.borrow_mut().left = Some(node1);
        root.borrow_mut().right = Some(node2);

        let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(2)));
        root2.borrow_mut().left = Some(node3);

        assert_eq!(Solution::is_same_tree(Some(root.clone()), Some(root.clone())), true);
        assert_eq!(Solution::is_same_tree(Some(root.clone()), Some(root2.clone())), false);
    }
}