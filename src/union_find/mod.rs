use std::hash::Hash;
use std::collections::HashMap;

pub struct UnionFind<T: Hash + Eq> {
    index: HashMap<T, usize>,
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl<T: Hash + Eq> UnionFind<T> {
    pub fn new() -> UnionFind<T> {
        UnionFind {
            index: HashMap::new(),
            parent: Vec::new(),
            size: Vec::new(),
        }
    }

    pub fn add_set(&mut self, value: T) {
        let new_idx = self.parent.len();
        self.index.insert(value, new_idx);
        self.parent.push(new_idx);
        self.size.push(1);
    }

    pub fn find_set(&mut self, value: &T) -> Option<usize> {
        if self.index.contains_key(value) {
            let idx = self.index[value];
            Some(self.set_by_idx(idx))
        } else {
            None
        }
    }

    fn set_by_idx(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            idx
        } else {
            let parent = self.parent[idx];
            self.parent[idx] = self.set_by_idx(parent);
            self.parent[idx]
        }
    }

    pub fn same_set(&mut self, first: &T, second: &T) -> bool {
        if let Some(first_set) = self.find_set(first) {
            if let Some(second_set) = self.find_set(second) {
                return first_set == second_set
            }
        }
        false
    }

    pub fn union_sets(&mut self, first: &T, second: &T) {
        if let Some(first_set) = self.find_set(first) {
            if let Some(second_set) = self.find_set(second) {
                let size1 = self.size[first_set];
                let size2 = self.size[second_set];

                if size1 > size2 {
                    self.parent[second_set] = first_set;
                    self.size[first_set] += size2;
                } else {
                    self.parent[first_set] = second_set;
                    self.size[second_set] += size1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test;
