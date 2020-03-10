//! ## 二叉树的中序遍历 ##
//! [原题目地址](https://leetcode-cn.com/problems/binary-tree-inorder-traversal/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给定一个二叉树，返回它的中序 遍历。
//! ```
//! 输入: [1,null,2,3]
//!   1
//!   \
//!    2
//!   /
//!  3
//!
//! 输出: [1,3,2]
//! ```
//! 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

/// Definition for a binary tree node.
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

/// 实现
pub struct Solution;

impl Solution {
    /// 递归解法
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = vec![];
        Solution::travel(&root, &mut vec);
        vec
    }

    pub fn travel(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(v) = root {
            Solution::travel(&v.as_ref().borrow().left, vec);
            vec.push(root.as_ref().unwrap().as_ref().borrow().val);
            Solution::travel(&v.as_ref().borrow().right, vec);
        }
    }

    /// 非递归解法
    /// 
    /// 使用一个栈stack，当前节点cur = root
    /// 1. 将cur及其左孩子入栈，直到cur = cur.left 没有左孩子
    /// 2. while 栈不为空时，弹出栈顶元素打印，此时让 cur 为栈顶元素右孩子，重复第1步
    pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut vec = vec![];

        // if let Some(v) = root {

        //     let mut stack = vec![];
        //     stack.push(v.clone());

        //     let mut cur = v;

        //     while let Some(ref left) = cur.clone().as_ref().borrow().left {
        //         stack.push(left.clone());
        //         cur = left.clone();
        //     }

        //     while let Some(node) = stack.pop() {
        //         vec.push(node.as_ref().borrow().val);
        //         if let Some(ref right) = node.as_ref().borrow().right {
        //             stack.push(right.clone());

        //             cur = right.clone();

        //             while let Some(ref left) = cur.clone().as_ref().borrow().left {
        //                 stack.push(left.clone());
        //                 cur = left.clone();
        //             }
        //         }
        //     }
        // }
        
        // vec

        // 优化代码，更精简
        let mut vec = vec![];

        let mut stack = vec![];
        let mut cur = root;

        while !stack.is_empty() || cur.is_some() {
            while let Some(v) = cur {
                stack.push(v.clone());
                cur = v.as_ref().borrow().left.clone();
            }

            let top = stack.pop().unwrap();
            vec.push(top.as_ref().borrow().val);
            cur = top.as_ref().borrow().right.clone();
        }
        
        vec
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_inorder_traversal() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(3)));

        node1.as_ref().borrow_mut().left = Some(node2);
        root.as_ref().borrow_mut().right = Some(node1);

        assert_eq!(Solution::inorder_traversal(Some(root.clone())), vec![1,3,2]);
        assert_eq!(Solution::inorder_traversal_v2(Some(root.clone())), vec![1,3,2]);
    }
}