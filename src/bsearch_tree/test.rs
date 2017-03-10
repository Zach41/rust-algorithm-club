use super::BSearchTree;

#[test]
fn test_empty() {
    let tree: BSearchTree<usize> = BSearchTree::Empty;

    assert_eq!(tree.count(), 0);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
}

#[test]
fn test_leaf() {
    let tree: BSearchTree<usize> = BSearchTree::Leaf(12);

    assert_eq!(tree.count(), 1);
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.minimum(), Some(&BSearchTree::Leaf(12)));
    assert_eq!(tree.maximum(), Some(&BSearchTree::Leaf(12)));
}

#[test]
fn test_insert() {
    let mut tree: BSearchTree<usize> = BSearchTree::Leaf(8);

    tree = tree.insert(5);
    tree = tree.insert(4);
    tree = tree.insert(6);
    tree = tree.insert(11);
    tree = tree.insert(13);
    tree = tree.insert(9);
    tree = tree.insert(14);

    assert_eq!(tree.count(), 8);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.minimum(), Some(&BSearchTree::Leaf(4)));
    assert_eq!(tree.maximum(), Some(&BSearchTree::Leaf(14)));

    let node1 = tree.search(11).unwrap();
    assert_eq!(node1.value(), Some(&11));

    let node2 = tree.search(12);
    assert_eq!(node2, None);
    assert!(tree.contains(13));
    assert!(!tree.contains(15));
}

struct Writer<T> {
    buf: Vec<T>
}

impl<T> Writer<T> {
    fn append(&mut self, value: T) {
        self.buf.push(value);
    }

    fn flush(&mut self) -> Vec<T> {
        self.buf.drain(..).collect()
    }
}

fn in_order_traverse<T>(tree: &BSearchTree<T>,
                        result: &mut Writer<T>)
    where T: Clone + Ord {
    match *tree {
        BSearchTree::Empty => (),
        BSearchTree::Leaf(ref val) => result.append(val.clone()),
        BSearchTree::Node(ref left, ref val, ref right) => {
            in_order_traverse(left, result);
            result.append(val.clone());
            in_order_traverse(right, result);
        }
    }
}

#[test]
fn test_in_order() {
    let mut tree: BSearchTree<usize> = BSearchTree::Leaf(8);

    tree = tree.insert(5);
    tree = tree.insert(4);
    tree = tree.insert(6);
    tree = tree.insert(11);
    tree = tree.insert(13);
    tree = tree.insert(9);
    tree = tree.insert(14);

    let mut result: Writer<usize> = Writer { buf: Vec::new() };
    in_order_traverse(&tree, &mut result);
    assert_eq!(result.flush(), vec![4, 5, 6, 8, 9, 11, 13, 14]);
}



