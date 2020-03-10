//! ##  对称二叉树 ##
//! [原题目地址](https://leetcode-cn.com/problems/symmetric-tree/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给定一个二叉树，检查它是否是镜像对称的。
//!
//! 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
//!```
//!     1
//!    / \
//!    2   2
//!    / \ / \
//!    3  4 4  3
//! ```
//! 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
//! ```
//!     1
//!    / \
//!   2   2
//!    \   \
//!    3    3
//! ```
//! 说明:
//! 
//! 如果你可以运用递归和迭代两种方法解决这个问题，会很加分。
//! 

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
    /// 递归方法，同时对root的左右子树已相反的方向遍历，如左子树 ‘先左后右’，则对右子树 ‘先右后左’
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::travel(&root, &root)
    }

    pub fn travel(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (Some(n1), Some(n2)) => {
                (n1.borrow().val == n2.borrow().val) && Solution::travel(&n1.borrow().left, &n2.borrow().right)
                && Solution::travel(&n1.borrow().right, &n2.borrow().left)
            },
            (None, None) => true,
            (_, _) => false
        }
    }

    /// 非递归方法1, 相反方向的层序遍历，用了两个队列
    pub fn is_symmetric_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let mut queue1 = vec![r.clone()];
            let mut queue2 = vec![r.clone()];

            while !queue1.is_empty() && !queue2.is_empty() {
                let (n1, n2) = (queue1.remove(0), queue2.remove(0));
                if n1.borrow().val != n2.borrow().val {
                    return false;
                }

                match (&n1.borrow().left, &n1.borrow().right, &n2.borrow().left, &n2.borrow().right) {
                    (None, None, None, None) => continue,
                    (Some(left1), None, None, Some(right2)) => {
                        queue1.push(left1.clone());
                        queue2.push(right2.clone());
                    },
                    (None, Some(right1), Some(left2), None) => {
                        queue1.push(right1.clone());
                        queue2.push(left2.clone());
                    },
                    (Some(left1), Some(right1), Some(left2), Some(right2)) => {
                        queue1.push(left1.clone());
                        queue2.push(right2.clone());
                        queue1.push(right1.clone());
                        queue2.push(left2.clone());
                    },
                    (_,_,_,_) => return false,
                };
            }
        }

        true
    }

    /// 非递归方法2, 相反方向的层序遍历，用一个队列，成对的弹出节点
    pub fn is_symmetric_v3(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = vec![root.clone(), root.clone()];
        while !queue.is_empty() {
            match (queue.remove(0), queue.remove(0)) {
                (None, None) => continue,
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    }
                    queue.push(n1.borrow().left.clone());
                    queue.push(n2.borrow().right.clone());
                    queue.push(n1.borrow().right.clone());
                    queue.push(n2.borrow().left.clone());
                },
                (_, _) => return false
            };
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_symmetric() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));

        root.borrow_mut().left = Some(node1);
        root.borrow_mut().right = Some(node2);

        let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(3)));
        root2.borrow_mut().left = Some(node3);
        root2.borrow_mut().left = Some(node4);

        assert_eq!(Solution::is_symmetric(Some(root.clone())), true);
        assert_eq!(Solution::is_symmetric(Some(root2.clone())), false);

        assert_eq!(Solution::is_symmetric_v2(Some(root.clone())), true);
        assert_eq!(Solution::is_symmetric_v2(Some(root2.clone())), false);

        assert_eq!(Solution::is_symmetric_v3(Some(root.clone())), true);
        assert_eq!(Solution::is_symmetric_v3(Some(root2.clone())), false);
    }
}