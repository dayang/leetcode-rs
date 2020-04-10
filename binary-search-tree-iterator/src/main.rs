//! ##  二叉搜索树迭代器 ##
//! [原题目地址](https://leetcode-cn.com/problems/binary-search-tree-iterator/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 实现一个二叉搜索树迭代器。你将使用二叉搜索树的根节点初始化迭代器。
//! 
//! 调用 next() 将返回二叉搜索树中的下一个最小的数。
//! ```
//!        7
//!      /  \
//!     3   15
//!        /  \
//!      9    20
//! ```
//!         
//! 
//! BSTIterator iterator = new BSTIterator(root);
//! iterator.next();    // 返回 3
//! iterator.next();    // 返回 7
//! iterator.hasNext(); // 返回 true
//! iterator.next();    // 返回 9
//! iterator.hasNext(); // 返回 true
//! iterator.next();    // 返回 15
//! iterator.hasNext(); // 返回 true
//! iterator.next();    // 返回 20
//! iterator.hasNext(); // 返回 false

#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;
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

struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        Self::push(&mut stack, root);
        BSTIterator{
            stack,
        }
    }

    fn push(stack: &mut Vec<Option<Rc<RefCell<TreeNode>>>>, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while root.is_some() {
            let left = root.as_ref().unwrap().borrow().left.clone();
            stack.push(root);
            root = left;
        }
    }
    
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let next = self.stack.pop().unwrap();
        let node = next.as_ref().unwrap().borrow();
        Self::push(&mut self.stack, node.right.clone());
        node.val
    }
    
    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    println!("Hello, world!");
}
