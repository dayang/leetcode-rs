//! ## 单词替换 ## 
//! [原题目地址](https://leetcode-cn.com/problems/replace-words/) 难度：<b>中等</b>
//! ### 题目描述 ###
//! 在英语中，我们有一个叫做 词根(root)的概念，它可以跟着其他一些词组成另一个较长的单词——我们称这个词为 继承词(successor)。例如，词根an，跟随着单词 other(其他)，可以形成新的单词 another(另一个)。
//! 
//! 现在，给定一个由许多词根组成的词典和一个句子。你需要将句子中的所有继承词用词根替换掉。如果继承词有许多可以形成它的词根，则用最短的词根替换它。
//! 
//! 你需要输出替换之后的句子。
//! 
//! 示例 1:
//! ```
//! 输入: dict(词典) = ["cat", "bat", "rat"]
//! sentence(句子) = "the cattle was rattled by the battery"
//! 输出: "the cat was rat by the bat"
//! ```
//! 注:
//! 
//! * 输入只包含小写字母。

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
    /// 先将单词表构建一个字典树，
    /// 然后检查句子中的每个单词的前缀是否在字典树中，如果存在则用前缀替换该单词即可
    pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut root = Node::new();

        for word in &dict {
            Self::insert(&mut root, word.as_bytes(), 0);
        }

        sentence.split_ascii_whitespace().map(|s| Self::shortest_prefix_of(&root, s.as_bytes(), 0).unwrap_or(s.to_string())).collect::<Vec<String>>().join(" ")
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

    pub fn shortest_prefix_of(node: &Node, s: &[u8], d: usize) -> Option<String> {
        if d == s.len() {
            return None;
        }
        match node.nexts.get(&s[d]) {
            Some(n) => { 
                let mut string = String::new();
                string.push(s[d] as char);
                if n.is_end {
                    return Some(string);
                }

                match Self::shortest_prefix_of(n, s, d + 1) {
                    Some(s) => Some(string + &s),
                    None => None
                }
            },
            None => None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
