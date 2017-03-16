use std::hash::Hash;
use std::collections::HashMap;

#[derive(Clone)]
struct TrieNode<T> {
    value: Option<T>,
    children: HashMap<T, Option<Box<TrieNode<T>>>>,
    parent: Option<Box<TrieNode<T>>>,
    terminate_flag: bool,
}

impl<T> TrieNode<T>
    where T: Hash + Eq + Copy {
    fn new() -> TrieNode<T> {
        TrieNode {
            value: None,
            children: HashMap::new(),
            parent: None,
            terminate_flag: false
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn add_child(&mut self, value: T) {
        if self.children.contains_key(&value) {
            return
        } else {
            let child = TrieNode {
                value: Some(value),
                children: HashMap::new(),
                parent: Some(Box::new(self.clone())),
                terminate_flag: false,
            };
            self.children.insert(value, Some(Box::new(child)));
        }
    }
}

use self::TrieNode as Node;

pub struct Trie<T> {
    root: Node<T>,
    word_cnt: usize,
}

struct Container<T> {
    inner: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Container<T> {
        Container {
            inner: Vec::new()
        }
    }

    fn append(&mut self, value: T) {
        self.inner.push(value)
    }

    fn drain(&mut self) -> Vec<T> {
        self.inner.drain(..).collect()
    }
}

impl<T> Trie<T>
    where T: Hash + Eq + Copy {
    pub fn is_empty(&self) -> bool {
        self.word_cnt == 0
    }

    pub fn count(&self) -> usize {
        self.word_cnt
    }

    // pub fn words(&self) -> Vec<String> {
    //     self.words_in_subtrie(self.root, "")
    // }

    // pub fn insert(&mut self, word: &str) {
    //     if word.is_empty() {
    //         return
    //     } else {
    //         let mut cur = self.root.clone();
            
    //     }
    // }
}
