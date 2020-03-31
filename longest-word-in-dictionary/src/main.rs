//! ## 词典中最长的单词 ## 
//! [原题目地址](https://leetcode-cn.com/problems/longest-word-in-dictionary/) 难度：<b>简单</b>
//! ### 题目描述 ###
//! 给出一个字符串数组words组成的一本英语词典。从中找出最长的一个单词，该单词是由words词典中其他单词逐步添加一个字母组成。若其中有多个可行的答案，则返回答案中字典序最小的单词。
//! 
//! 若无答案，则返回空字符串。
//! 
//! 示例 1:
//! ```
//! 输入: 
//! words = ["w","wo","wor","worl", "world"]
//! 输出: "world"
//! 解释: 
//! 单词"world"可由"w", "wo", "wor", 和 "worl"添加一个字母组成。
//! ```
//! 示例 2:
//! ```
//! 输入: 
//! words = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
//! 输出: "apple"
//! 解释: 
//! "apply"和"apple"都能由词典中的单词组成。但是"apple"得字典序小于"apply"。
//! ```
//! 注意:
//! 
//! * 所有输入的字符串都只包含小写字母。
//! * words数组长度范围为[1,1000]。
//! * words[i]的长度范围为[1,30]。

use std::collections::HashMap;

pub struct Solution;

pub struct Node {
    is_end: bool,
    nexts: HashMap<u8, Box<Node>>
}

impl Node{
    fn new() -> Self{
        Node {
            is_end: false,
            nexts: HashMap::new()
        }
    }
}

impl Solution {
    /// 先将所有单词加入一个前缀树(字典树)，然后判断一个单词的路径上所有的节点是否都是
    /// 尾结点，如果是则说明每一个节点都是一个字符串的结尾，那么该单词符合条件。
    pub fn longest_word(mut words: Vec<String>) -> String {
        let mut root = Node::new();
        words.sort_by(|a,b| b.len().cmp(&a.len()).then(a.cmp(b)));
        for word in &words {
            Self::insert(&mut root, word.as_bytes(), 0);
        }

        for word in &words {
            if Self::is_full_path(&root, word.as_bytes(), 0) {
                return word.to_string();
            }
        }

        String::new()
    }

    pub fn insert(node: &mut Node, s: &[u8], d: usize){
        if d == s.len() {
            return;
        }

        let cur_node = node.nexts.entry(s[d]).or_insert_with(|| Box::new(Node::new()));
        if d == s.len() - 1 {
            cur_node.is_end = true;
        }
        Self::insert(cur_node, s, d + 1);
    }

    pub fn is_full_path(node: &Node, s: &[u8], d: usize) -> bool {
        if d == s.len() {
            return true;
        }

        let cur_node = node.nexts.get(&s[d]).unwrap();
        if !cur_node.is_end {
            return false;
        }

        Self::is_full_path(cur_node, s, d + 1)
    }
}

fn main() {
    println!("Hello, world!");
}
