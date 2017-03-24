enum SegTree<T> {
    Leaf(Node<T>),
    NotLeaf(Box<SegTree<T>>, Node<T>, Box<SegTree<T>>),
}

struct Node<T> {
    value: T,
    left_bound: usize,
    right_bound: usize,
}

pub trait ValueOp {
    fn op(self, rhs: Self) -> Self;
}

impl<T> Node<T> {
    fn new(value: T, left: usize, right: usize) -> Node<T> {
        Node {
            value: value,
            left_bound: left,
            right_bound: right,
        }
    }
}

impl<T: Copy> SegTree<T> {
    fn value(&self) -> T {
        match *self {
            SegTree::Leaf(ref node) | SegTree::NotLeaf(_, ref node, _) => node.value
        }
    }
}

impl<T> SegTree<T> {
    fn left_bound(&self) -> usize {
        match *self {
            SegTree::Leaf(ref node) | SegTree::NotLeaf(_, ref node, _) => node.left_bound
        }
    }

    fn right_bound(&self) -> usize {
        match *self {
            SegTree::Leaf(ref node) | SegTree::NotLeaf(_, ref node, _) => node.right_bound
        }
    }
}

impl<T: Copy + ValueOp> SegTree<T> {
    fn new(array: &[T], left: usize, right: usize) -> SegTree<T> {
        if left == right {
            SegTree::Leaf(Node::new(array[left], left, right))
        } else {
            let middle = left + (right - left) / 2;
            let left_child = SegTree::new(array, left, middle);
            let right_child = SegTree::new(array, middle+1, right);
            let node = Node::new(ValueOp::op(left_child.value(), right_child.value()), left, right);
            SegTree::NotLeaf(Box::new(left_child), node, Box::new(right_child))
        }
    }


    fn query(&self, left: usize, right: usize) -> T {
        match *self {
            SegTree::NotLeaf(ref left_child, ref node, ref right_child) => {
                if node.left_bound == left && node.right_bound == right {
                    node.value
                } else if left_child.right_bound() < left {
                    right_child.query(left, right)
                } else if right_child.left_bound() > right {
                    left_child.query(left, right)
                } else {
                    let left_val = left_child.query(left, left_child.right_bound());
                    let right_val = right_child.query(right_child.left_bound(), right);
                    ValueOp::op(left_val, right_val)
                }
            },
            SegTree::Leaf(ref node) => {
                assert_eq!(left, right);
                node.value
            }
        }
    }

    fn replace(&mut self, index: usize, item: T) {
        match *self {
            SegTree::Leaf(ref mut node) => {
                node.value = item;
            },
            SegTree::NotLeaf(ref mut left, ref mut node, ref mut right) => {
                if left.right_bound() < index {
                    right.replace(index, item);
                } else {
                    left.replace(index, item);
                }
                node.value = ValueOp::op(left.value(), right.value());
            }
        }
    }
}

pub struct SegmentTree<T> {
    root: SegTree<T>,
}

impl<T: Copy + ValueOp> SegmentTree<T> {
    pub fn new(array: &[T]) -> SegmentTree<T> {
        let root = SegTree::new(array, 0, array.len() - 1);
        SegmentTree {
            root: root,
        }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        assert!(left <= right, "left can't larger than right");
        assert!(left >= self.root.left_bound(), "invalid left bound");
        assert!(right <= self.root.right_bound(), "invalid right bound");
        self.root.query(left, right)
    }

    pub fn replace(&mut self, index: usize, item: T) {
        assert!(index <= self.root.right_bound() &&
                index >= self.root.left_bound(), "invalid index value");
        self.root.replace(index, item);
    }
}

#[cfg(test)]
mod test;

