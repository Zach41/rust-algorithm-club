use std::cell::Cell;
use std::ops::Drop;

use super::*;

struct DropCheck<'a>(&'a Cell<usize>);

impl<'a> Drop for DropCheck<'a> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn tree_works() {
    let mut new_counter = 0;
    let drop_counter = Cell::new(0);

    {
        let mut arena: Arena<(usize, DropCheck)> = Arena::new();
        
        macro_rules! new {
            () => {
                {
                    new_counter += 1;
                    arena.new_node((new_counter, DropCheck(&drop_counter)))
                }
            }

        };

        let node1 = new!();
        let node2 = new!();
        let node3 = new!();
        let node4 = new!();
        let node5 = new!();
        let node6 = new!();
        let node7 = new!();
        let node8 = new!();
        let node9 = new!();
        let node10 = new!();

        
        node1.append(&mut arena, node2);
        node1.append(&mut arena, node3);
        node1.prepend(&mut arena, node4);
        node5.append(&mut arena, node1);
        node1.insert_before(&mut arena, node6);
        node1.insert_before(&mut arena, node7);
        node1.insert_after(&mut arena, node8);
        node1.insert_after(&mut arena, node9);
        node5.append(&mut arena, node10);

        let mut tree_array: Vec<_> = node5.descendants(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(tree_array, [5, 6, 7, 1, 4, 2, 3, 9, 8, 10]);

        node1.detach(&mut arena);
        tree_array = node5.descendants(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(tree_array, [5, 6, 7, 9, 8, 10]);

        assert_eq!(arena[node7].parent(), Some(node5));
        assert_eq!(arena[node7].next_sibling(), Some(node9));
        assert_eq!(arena[node7].previous_sibling(), Some(node6));
        assert_eq!(arena[node5].first_child(), Some(node6));
        assert_eq!(arena[node5].last_child(), Some(node10));

        let children: Vec<_> = node5.children(&arena).map(|idx| arena[idx].data.0).collect();
        assert_eq!(children, [6, 7, 9, 8, 10]);

        let reverse_children: Vec<_> = node5.reverse_children(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(reverse_children, [10, 8, 9, 7, 6]);

        let preceding_siblings: Vec<_> = node8.preceding_siblings(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(preceding_siblings, [8, 9, 7, 6]);

        let following_siblings: Vec<_> = node6.following_siblings(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(following_siblings, [6, 7, 9, 8, 10]);

        node8.insert_after(&mut arena, node1);
        tree_array = node5.descendants(&arena)
            .map(|idx| arena[idx].data.0).collect();
        assert_eq!(tree_array, [5, 6, 7, 9, 8, 1, 4, 2, 3, 10]);

        let ancestors: Vec<_> = node2.ancestors(&arena).map(|idx| arena[idx].data.0).collect();
        assert_eq!(ancestors, [2, 1, 5]);        
        
    }
    assert_eq!(drop_counter.get(), 10);
}

#[test]
fn test_find() {
    let mut new_counter = 0;
    let mut arena: Arena<usize> = Arena::new();
    macro_rules! new {
        () => {
            {
                new_counter += 1;
                arena.new_node(new_counter)
            }
        }
    };

    let node1 = new!();
    let node2 = new!();
    let node3 = new!();
    let node4 = new!();
    let node5 = new!();
    let node6 = new!();
    let node7 = new!();
    let node8 = new!();
    let node9 = new!();
    let node10 = new!();

    
    node1.append(&mut arena, node2);
    node1.append(&mut arena, node3);
    node1.prepend(&mut arena, node4);
    node5.append(&mut arena, node1);
    node1.insert_before(&mut arena, node6);
    node1.insert_before(&mut arena, node7);
    node1.insert_after(&mut arena, node8);
    node1.insert_after(&mut arena, node9);
    node5.append(&mut arena, node10);

    let node_1 = node5.find_node(&mut arena, 1).unwrap();
    assert_eq!(node1, node_1);
    let node_2 = node5.find_node(&mut arena, 2).unwrap();
    assert_eq!(node_2, node2);
    let node_11 = node5.find_node(&mut arena, 11);
    assert!(node_11.is_none());
}
