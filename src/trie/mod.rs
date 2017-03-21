use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use slab::{Slab, VacantEntry, Entry as SlabEntry};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NodeId {
    index: usize
}

struct Node<T> {
    parent: Option<NodeId>,
    children: HashMap<T, NodeId>,
    terminal: bool,
    
    value: Option<T>,
}

struct Arena<T> {
    nodes: Slab<Node<T>>
}

impl<T> Arena<T>
    where T: Eq + Hash {
    fn new() -> Arena<T> {
        Arena {
            nodes: Slab::with_capacity(256),
        }
    }

    fn append_node(&mut self, data: Option<T>) -> NodeId {
        // let idx = self.nodes.len();
        // self.nodes.push(
        //     Node {
        //         parent: None,
        //         children: HashMap::new(),
        //         terminal: false,
        //         value: data,
        //     }
        // );
        // NodeId { index: idx }

        let node = Node {
            parent: None,
            children: HashMap::new(),
            terminal: false,
            value: data,
        };

        let idx = match self.nodes.insert(node) {
            Ok(idx) => idx,
            Err(node) => {
                {
                    let len = self.nodes.len();
                    self.nodes.reserve_exact(len * 2);
                }
                match self.nodes.insert(node) {
                    Ok(idx) => idx,
                    Err(_) => unreachable!()
                }
            }
        };
        NodeId { index: idx }
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Node<T>  {
        self.nodes.get(index.index).unwrap()
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Node<T> {
        self.nodes.get_mut(index.index).unwrap()
    }
}

impl<T: Eq + Hash> Node<T> {
    fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl NodeId {
    fn add<T: Eq + Hash + Clone>(self, value: T, arena: &mut Arena<T>) -> Option<NodeId> {
        let is_exits = {
            let node = &arena[self];
            node.children.contains_key(&value)
        };
        if !is_exits {
            let idx = arena.append_node(Some(value.clone()));
            arena[self].children.insert(value, idx);
            Some(idx)
        } else {
            None
        }
    }
}

pub struct Trie {
    word_cnt: usize,
    root: NodeId,
    arena: Arena<char>,
}

impl Trie {
    pub fn words_count(&self) -> usize {
        self.word_cnt
    }

    pub fn is_empty(&self) -> bool {
        self.word_cnt == 0
    }

    pub fn new() -> Trie {
        let mut arena: Arena<char> = Arena::new();
        let root = arena.append_node(None);
        Trie {
            word_cnt: 0,
            root: root,
            arena: arena,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let word = word.to_lowercase();
        let mut root = self.root;
        for c in word.chars() {
            if self.arena[root].children.contains_key(&c) {
                root = self.arena[root].children[&c];
            } else {
                let child = root.add(c, &mut self.arena).unwrap();
                self.arena[child].parent = Some(root);
                root = child;
            }
        }
        self.arena[root].terminal = true;
        self.word_cnt += 1;
    }

    pub fn remove(&mut self, word: &str) {
        let word = word.to_lowercase();
        let node = self.find_last_node(word.as_str()).unwrap();
        if self.arena[node].is_leaf() {
            self.delete_node(node)
        } else {
            self.arena[node].terminal = false
        }
        self.word_cnt -= 1;
    }

    pub fn find_with_prefix(&self, prefix: &str) -> Vec<String> {
        assert!(prefix.len() > 0);
        let prefix = prefix.to_lowercase();
        let mut words: Vec<String> = Vec::new();
        let node = self.find_last_node(prefix.as_str());
        if node.is_some() {
            self.words_in_subtrie(node.unwrap(), prefix.as_str(), &mut words);
        }
        words
    }

    pub fn contains(&self, word: &str) -> bool {
        let word = word.to_lowercase();
        let mut root = self.root;
        for c in word.chars() {
            if self.arena[root].children.contains_key(&c) {
                root = self.arena[root].children[&c];
            } else {
                return false
            }
        }
        self.arena[root].terminal
    }

    pub fn words(&self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        self.words_in_subtrie(self.root, "", &mut words);
        words
    }
}

impl Trie {
    fn find_last_node(&self, word: &str) -> Option<NodeId> {
        let mut root = self.root;
        for c in word.chars() {
            if self.arena[root].children.contains_key(&c) {
                root = self.arena[root].children[&c];
            } else {
                return None
            }
        }
        Some(root)
    }

    fn delete_node(&mut self, node: NodeId) {
        let mut last_node = node;

        while self.arena[last_node].is_leaf() {
            if let Some(parent) = self.arena[last_node].parent() {
                let c = self.arena[last_node].value.unwrap();
                let removed = self.arena[parent].children.remove(&c);
                
                self.arena.nodes.remove(last_node.index).unwrap();
                assert_eq!(removed, Some(last_node));

                last_node = parent;
                if self.arena[last_node].terminal {
                    break;
                }
            }
        }
    }

    fn words_in_subtrie(&self, root: NodeId, partial: &str, acc: &mut Vec<String>) {
        let mut partial_str = partial.to_owned();

        if self.arena[root].terminal {
            acc.push(partial_str.clone());
        }

        for (k, v) in &self.arena[root].children {            
            partial_str.push(*k);
            let child = *v;
            self.words_in_subtrie(child, partial_str.as_str(), acc);
            partial_str.pop().unwrap();
        }
    }
}

#[cfg(test)]
mod test;

