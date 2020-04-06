//! ## 不同的二叉搜索树 II## 
//! [原题目地址](https://leetcode-cn.com/problems/unique-binary-search-trees-ii/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个整数 n，生成所有由 1 ... n 为节点所组成的二叉搜索树。
//! 
//! 示例:
//! ```
//! 输入: 3
//! 输出:
//! [
//!   [1,null,3,2],
//!   [3,2,null,1],
//!   [3,1,null,null,2],
//!   [2,1,3],
//!   [1,null,2,null,3]
//! ]
//! 解释:
//! 以上的输出对应以下 5 种不同结构的二叉搜索树：
//! 
//!    1         3     3      2      1
//!     \       /     /      / \      \
//!      3     2     1      1   3      2
//!     /     /       \                 \
//!    2     1         2                 3
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n < 1 {
            return vec![];
        }
        Self::dfs(1, n)
    }

    pub fn dfs(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l > r {
            return vec![None];
        }

        let mut trees = vec![];
        for i in l..=r {
            let left_trees = Self::dfs(l, i - 1);
            let right_trees = Self::dfs(i+1, r);
            for left in left_trees {
                for right in &right_trees {
                    let mut root = TreeNode::new(i);
                    root.left = left.clone();
                    root.right = right.clone();
                    trees.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }

        trees
    }
}

fn main() {
    println!("Hello, world!");
}
