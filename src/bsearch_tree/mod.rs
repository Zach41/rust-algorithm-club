use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum BSearchTree<T> {
    Empty,
    Leaf(T),
    Node(Box<BSearchTree<T>>, T, Box<BSearchTree<T>>),
}

impl<T: Ord> BSearchTree<T> {
    pub fn count(&self) -> usize {
        match *self {
            BSearchTree::Empty => 0,
            BSearchTree::Leaf(_) => 1,
            BSearchTree::Node(ref left, _, ref right) => 1 + left.count() + right.count()
        }
    }

    pub fn value(&self) -> Option<&T> {
        match *self {
            BSearchTree::Empty => None,
            BSearchTree::Leaf(ref v) | BSearchTree::Node(_, ref v, _) => Some(v),
        }
    } 

    pub fn height(&self) -> usize {
        match *self {
            BSearchTree::Empty => 0,
            BSearchTree::Leaf(_) => 1,
            BSearchTree::Node(ref left, _, ref right) => 1 + ::std::cmp::max(left.height(), right.height()),
        }
    }

    pub fn insert(mut self, value: T) -> BSearchTree<T> {
        match self {
            BSearchTree::Empty => {
                self = BSearchTree::Leaf(value);
                self
            },
            BSearchTree::Leaf(val) => {
                match value.cmp(&val) {
                    Ordering::Less => {
                        self = BSearchTree::Node(Box::new(BSearchTree::Leaf(value)),
                                                 val,
                                                 Box::new(BSearchTree::Empty));
                        self
                    },
                    _ => {
                        self = BSearchTree::Node(Box::new(BSearchTree::Empty),
                                                 val,
                                                 Box::new(BSearchTree::Leaf(value)));
                        self
                    }
                }
            },
            BSearchTree::Node(left, val, right) => {
                match value.cmp(&val) {
                    Ordering::Less => {
                        self = BSearchTree::Node(Box::new(left.insert(value)),
                                                 val, right);
                        self
                    },
                    _ => {
                        self = BSearchTree::Node(left, val, Box::new(right.insert(value)));
                        self
                    }
                }
            }
        }
    }

    pub fn search(&self, value: T) -> Option<&BSearchTree<T>> {
        match *self {
            BSearchTree::Empty => None,
            BSearchTree::Leaf(ref val) => {
                match value.cmp(val) {
                    Ordering::Equal => Some(self),
                    _ => None,
                }
            },
            BSearchTree::Node(ref left, ref val, ref right) => {
                match value.cmp(val) {
                    Ordering::Equal => Some(self),
                    Ordering::Less => left.search(value),
                    Ordering::Greater => right.search(value),
                }
            }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self.search(value) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn minimum(&self) -> Option<&BSearchTree<T>>{
        match *self {
            BSearchTree::Empty => None,
            BSearchTree::Leaf(_) => Some(self),
            BSearchTree::Node(ref left, _, _) => left.minimum()
        }
    }

    pub fn maximum(&self) -> Option<&BSearchTree<T>> {
        match *self {
            BSearchTree::Empty => None,
            BSearchTree::Leaf(_) => Some(self),
            BSearchTree::Node(_, _, ref right) => right.maximum(),
        }
    }
}

#[cfg(test)]
mod test;

