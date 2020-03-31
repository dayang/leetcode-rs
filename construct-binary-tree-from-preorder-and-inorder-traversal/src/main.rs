//! ## 从前序与中序遍历序列构造二叉树 ##
//! [原题目地址](https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 根据一棵树的前序遍历与中序遍历构造二叉树。
//! 
//! 注意:
//! 你可以假设树中没有重复的元素。
//! 
//! 例如，给出
//! ```
//! 前序遍历 preorder = [3,9,20,15,7]
//! 中序遍历 inorder = [9,3,15,20,7]
//! ```
//! 返回如下的二叉树：
//! ```
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_helper(&preorder[..], &inorder[..])
    }

    pub fn build_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);
        for i in 0..inorder.len() {
            if inorder[i] == preorder[0] {
                root.left = Self::build_helper(&preorder[1..i+1], &inorder[0..i]);
                root.right = Self::build_helper(&preorder[i+1..], &inorder[i+1..]);
                break;
            }
        }

        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {
    println!("Hello, world!");
}
