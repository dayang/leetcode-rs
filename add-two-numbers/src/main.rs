//! ## 两数相加 ##
//! [原题目地址](https://leetcode-cn.com/problems/add-two-numbers/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
//! 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
//! 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//! ```
//! 示例:
//! 输入: (2 -> 4 -> 3) + (5 -> 6 -> 4)
//! 输出: 7 -> 0 -> 8
//! 原因: 342 + 465 = 807
//! ```

// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// 实现
pub struct Solution;

impl Solution {
    /// 循环两个链表节点相加，注意进位即可
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        let mut head = None;
        let mut curr = &mut head;
        
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }

            *curr = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            curr = &mut curr.as_mut().unwrap().next;
        }

        head
    }

    /// 第一版
    pub fn add_two_numbers_v1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut head = None;
        let mut curr = &mut head;
        
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if l1.is_some() {
                sum += l1.as_ref().unwrap().val;
                l1 = &l1.as_ref().unwrap().next;
            }

            if l2.is_some() {
                sum += l2.as_ref().unwrap().val;
                l2 = &l2.as_ref().unwrap().next;
            }

            *curr = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            curr = &mut curr.as_mut().unwrap().next;
        }

        head
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add_two_numbers() {
        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let mut l2 = Some(Box::new(ListNode::new(5)));
        l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
        l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        match Solution::add_two_numbers(l1, l2) {
            Some(node1) => {
                assert_eq!(node1.val, 7);
                match node1.next {
                    Some(node2) => {
                        assert_eq!(node2.val, 0);
                        match node2.next {
                            Some(node3) => {
                                assert_eq!(node3.val, 8);
                            }   
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _=>assert!(false)
        };
    }
}