#[cfg(test)]
extern crate typed_arena;

use std::ops::Drop;
use std::cell::Cell;

#[cfg(test)]
use self::typed_arena::Arena;

use super::*;

struct DropChecker<'a>(&'a Cell<usize>);

impl<'a> Drop for DropChecker<'a> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn test_create_tree() {
    let root = ArenaNode::new(0);

    assert!(root.parent().is_none());
    assert!(root.first_child().is_none());
    assert!(root.last_child().is_none());
    assert!(root.previous_sibling().is_none());
    assert!(root.next_sibling().is_none());
    assert_eq!(root.data, 0);
    assert!(root.same_node(&root));
}

struct Foo<'a>(&'a Cell<usize>);
#[test]
fn tree_workds() {
    let mut new_counter = 0;
    let drop_counter = Cell::new(0);

    {
        let arena: Arena<ArenaNode<(usize, DropChecker)>> = Arena::new();
        let mut new = || {
            new_counter += 1;
            arena.alloc(ArenaNode::new((new_counter, DropChecker(&drop_counter))))
        };
        
        let a = new(); // 1
        let node2 = new();
        let node3 = new();
        let node4 = new();
        a.append(node2); // 2
        a.append(node3); // 3
        a.prepend(node4); // 4
        let root = new(); // 5
        root.append(a);
        let mut tree_array: Vec<_> = root.descendants().map(|node| node.data.0).collect();
        assert_eq!(tree_array, [5, 1, 4, 2, 3]);
        let node6 = new();
        let node7 = new();
        let node8 = new();
        let node9 = new();
        a.insert_before(node6); // 6
        a.insert_before(node7); // 7
        a.insert_after(node8); // 8
        a.insert_after(node9); // 9
        let c = new(); // 10
        root.append(c);
        tree_array = root.descendants().map(|node| node.data.0).collect();
        assert_eq!(tree_array, vec![5, 6, 7, 1, 4, 2, 3, 9, 8, 10]);

        a.detach();

        tree_array = root.descendants().map(|node| node.data.0).collect();
        assert_eq!(tree_array, vec![5, 6, 7, 9, 8, 10]);

        
        assert!(root.first_child().unwrap().same_node(&node6));
        assert!(root.last_child().unwrap().same_node(&c));
        assert!(node7.next_sibling().unwrap().same_node(&node9));
        assert!(node9.previous_sibling().unwrap().same_node(&node7));
        assert!(node8.parent().unwrap().same_node(&root));
        let children_array: Vec<_> = root.children().map(|node| node.data.0).collect();
        assert_eq!(children_array, [6, 7, 9, 8, 10]);

        let reverse_children_arr: Vec<_> = root.reverse_children().map(|node| node.data.0).collect();
        assert_eq!(reverse_children_arr, [10, 8, 9, 7, 6]);        
    }
    assert_eq!(drop_counter.get(), 10);        
}

#[test]
fn find_node() {
    let mut new_counter = 0;
    let arena: Arena<ArenaNode<usize>> = Arena::new();
    let mut new = || {
        new_counter += 1;
        arena.alloc(ArenaNode::new(new_counter))
    };

    let node1 = new(); // 1
    let node2 = new();
    let node3 = new();
    let node4 = new();
    node1.append(node2); // 2
    node1.append(node3); // 3;
    node1.prepend(node4); // 4;
    let root = new();
    root.append(node1);
    node1.insert_before(new()); // 6
    let node7 = new();
    node1.insert_before(node7); // 7
    node1.insert_after(new()); // 8
    let node9 = new();
    node1.insert_after(node9); // 9
    root.append(new()); // 10

    let node_1 = root.find_node(&1).unwrap();
    assert_eq!(node_1.data, 1);
    assert!(node_1.same_node(node1));
    assert!(node_1.parent().unwrap().same_node(root));
    assert!(node_1.next_sibling().unwrap().same_node(node9));
    assert!(node_1.previous_sibling().unwrap().same_node(node7));
    assert_eq!(node_1.children().map(|node| node.data).collect::<Vec<_>>(), [4, 2, 3]);

    let node_2 = node_1.find_node(&2).unwrap();
    assert_eq!(node_2.data, 2);
    assert!(node_2.same_node(&node2));
    assert!(node_2.parent().unwrap().same_node(node1));
    assert!(node_2.next_sibling().unwrap().same_node(node3));
    assert!(node_2.previous_sibling().unwrap().same_node(node4));

    let node_11 = root.find_node(&11);
    assert!(node_11.is_none());
}

