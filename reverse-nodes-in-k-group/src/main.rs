//! ## K 个一组翻转链表 ##
//! [原题目地址](https://leetcode-cn.com/problems/reverse-nodes-in-k-group/) 难度：<b>困难</b>
//! ### 题目描述 ###
//! 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
//! 
//! k 是一个正整数，它的值小于或等于链表的长度。
//! 
//! 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
//! 
//! 示例：
//! ```
//! 给你这个链表：1->2->3->4->5
//! 
//! 当 k = 2 时，应当返回: 2->1->4->3->5
//! 
//! 当 k = 3 时，应当返回: 3->2->1->4->5
//! ```
//!  
//! 
//! 说明：
//! 
//! * 你的算法只能使用常数的额外空间。
//! * <b>你不能只是单纯的改变节点内部的值</b>，而是需要实际进行节点交换。
//! 
//! c#代码，rust链表搞死个人啊，这题放弃了先
//! 
//! 关键是每次翻转k个节点后，要记下反转后的尾结点，要与下一次反转后的头结点连在一起
//! 
//! 以及，第一次翻转后的头结点是整个链表的头结点。
//! 
//! 用一个指针向前走k步判断余下的链表是否够k个长度，够则翻转，不够则不翻转
//! ```
//! /**
//!  * Definition for singly-linked list.
//!  * public class ListNode {
//!  *     public int val;
//!  *     public ListNode next;
//!  *     public ListNode(int x) { val = x; }
//!  * }
//!  */
//! public class Solution {
//!     public ListNode ReverseKGroup(ListNode head, int k) {
//!         int cnt;
//!         ListNode cursor;
//! 
//!         ListNode pre = null;
//!         ListNode curr = head;
//!         ListNode next = null;
//! 
//!         ListNode pEnd = head;
//! 
//!         bool bFirst = true;
//! 
//!         ListNode newHead = null;
//! 
//!         while (true){ 
//!             cnt = k;
//!             // 判断是否够k个节点
//!             cursor = curr;
//!             while ( cursor != null && cnt-- > 0 ) {
//!                 cursor = cursor.next;
//!             }
//! 
//!             if (cnt > 0) {
//!                 // 不足 k 个节点
//!                 if (!bFirst){
//!                     pEnd.next = curr;
//!                 } else {
//!                     newHead = head;
//!                 }
//!                 break;
//!             }
//! 
//!             cnt = k;
//! 
//!             pre = null;
//!             ListNode newEnd = curr;
//!             // 用三个指正，翻转链表
//!             while (curr != null && cnt-- > 0) {
//!                 next = curr.next;
//!                 curr.next = pre;
//!                 pre = curr;
//!                 curr = next;
//!             }
//! 
//!             if (!bFirst) {
//!                 // 将翻转后的k个节点的链表与上次尾节点连接
//!                 pEnd.next = pre;
//!             }
//!             
//!             // 记下翻转后的尾结点
//!             pEnd = newEnd;
//! 
//!             if (bFirst){
//!                 bFirst = false;
//!                 newHead = pre;
//!             }
//!         }
//! 
//!         return newHead;
//!     }
//! }
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        None
    }
}

fn main() {
    println!("Hello, world!");
}
