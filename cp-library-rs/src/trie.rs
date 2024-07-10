//! トライ木

use std::fmt::Debug;

// 定数
const ORIGIN: char = 'a'; // 基準となる文字
const ORIGIN_ID: usize = ORIGIN as u32 as usize; // 基準となる文字のID
const KINDS: usize = 26; // 文字の種類数
type NodePointer<T> = Option<Box<TrieNode<T>>>;

/// 何番目の文字かを判定する
fn ord(c: char) -> usize {
    let num = c as u32 as usize;
    num - ORIGIN_ID
}

/// i番目の文字を返す
fn chr(i: usize) -> char {
    (ORIGIN_ID + i) as u8 as char
}

/// # TrieNode
/// - トライ木のノード
#[derive(Debug, Clone)]
struct TrieNode<T> {
    data: Option<T>,
    children: Vec<NodePointer<T>>,
}

impl<T> TrieNode<T>
where
    T: Clone,
{
    pub fn new(data: Option<T>) -> Self {
        Self {
            data,
            children: vec![NodePointer::None; KINDS],
        }
    }
}

/// # Trie
/// - トライ木の実装
#[derive(Debug)]
pub struct Trie<T> {
    size: usize,
    root: NodePointer<T>,
}

impl<T> Trie<T>
where
    T: Clone + Debug,
{
    // self.originを基準にした文字の番号を返す
    // fn ord()

    pub fn new() -> Self {
        Trie {
            size: 0,
            root: Some(Box::new(TrieNode {
                data: None,
                children: vec![NodePointer::None; KINDS],
            })),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, key: &str, data: T) -> Option<T> {
        let res = self.get_or_insert_mut(key).replace(data);
        if res.is_none() {
            self.size += 1;
        }
        res
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;
        for c in key.chars().map(ord) {
            node = &node.as_ref()?.children[c];
        }
        node.as_deref()?.data.as_ref()
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        let mut node = &mut self.root;
        for c in key.chars().map(ord) {
            node = node.as_mut()?.children.get_mut(c).unwrap();
        }
        node.as_deref_mut()?.data.as_mut()
    }

    pub fn get_or_insert_mut(&mut self, key: &str) -> &mut Option<T> {
        let mut node = &mut self.root;
        for c in key.chars().map(ord).chain(KINDS..=KINDS) {
            // データの挿入
            if c == KINDS {
                if node.as_ref().is_none() {
                    *node = Some(Box::new(TrieNode::new(None)));
                }
                break;
            }
            if node.as_ref().is_none() {
                *node = Some(Box::new(TrieNode::new(None)));
            }
            node = node.as_mut().unwrap().children.get_mut(c).unwrap();
        }
        &mut node.as_deref_mut().unwrap().data
    }

    pub fn traverse(&self) -> Vec<(String, &T)> {
        let mut res = vec![];
        let mut cur = String::new();
        traverse_inner(&self.root, &mut cur, &mut res);
        res
    }
}

/// trieを順に探索する
fn traverse_inner<'a, T>(
    node: &'a NodePointer<T>,
    cur: &mut String,
    list: &mut Vec<(String, &'a T)>,
) {
    if let Some(value) = node.as_ref().unwrap().data.as_ref() {
        let key = cur.clone();
        list.push((key, value));
    }
    if let Some(node) = node.as_deref() {
        for (i, child) in node.children.iter().enumerate() {
            if child.as_ref().is_some() {
                cur.push(chr(i));
                traverse_inner(child, cur, list);
                cur.pop();
            }
        }
    }
}
