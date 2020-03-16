//! ##  链表中倒数第k个节点 ##
//! [原题目地址](https://leetcode-cn.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 输入一个链表，输出该链表中倒数第k个节点。为了符合大多数人的习惯，本题从1开始计数，即链表的尾节点是倒数第1个节点。例如，一个链表有6个节点，从头节点开始，它们的值依次是1、2、3、4、5、6。这个链表的倒数第3个节点是值为4的节点。
//! 
//! 示例：
//! ```
//! 给定一个链表: 1->2->3->4->5, 和 k = 2.
//! 
//! 返回链表 4->5.
//! ```

// Definition for singly-linked list.
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

pub struct Solution;

impl Solution {
    /// 使用快慢指针，快指针先走k步，两个指针再同时走，快指针到结尾时，慢指针即为倒数第k个节点
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (mut quick, mut slow) = (&head, &head);
        for _ in 1..k {
            quick = &quick.as_ref().unwrap().next;
        }

        while quick.as_ref().unwrap().next.is_some() {
            quick = &quick.as_ref().unwrap().next;
            slow =  &slow.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

fn main() {
    println!("Hello, world!");
}