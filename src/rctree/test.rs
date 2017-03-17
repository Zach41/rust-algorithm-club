use super::*;

struct DropChecker<'a>(&'a cell::Cell<u32>);
impl<'a> Drop for DropChecker<'a> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

struct TestChecker(Rc<cell::Cell<usize>>);

impl Clone for TestChecker {
    fn clone(&self) -> TestChecker {
        TestChecker(self.0.clone())
    }
}

impl Drop for TestChecker {
    fn drop(&mut self) {
        println!("DROP");
        self.0.set(self.0.get() + 1);        
    }
}

macro_rules! setup {
    ($counter_var: ident, $checker_var: ident, $new_method: ident) => {
        let mut $counter_var = 0;
        let $checker_var = cell::Cell::new(0);

        let mut $new_method = || {
            $counter_var += 1;
            NodeRef::new(($counter_var, DropChecker(&$checker_var)))
        };
    }
}

macro_rules! setup_tree {
    ($root: ident) => {
        let mut new_counter = 0;

        let mut new = || {
            new_counter += 1;
            NodeRef::new(new_counter)
        };

        let node = new(); // 1
        node.append(new()); // 2
        node.append(new()); // 3
        node.prepend(new()); // 4
        let $root = new(); // 5
        $root.append(node.clone());
        node.insert_before(new()); // 6
        node.insert_before(new()); // 7
        node.insert_after(new()); // 8
        node.insert_after(new()); // 9
        $root.append(new()); // 10

        $root.clone()
    }
}

#[test]
fn tree_works() {
    // let drop_counter = cell::Cell::new(0);
    // let mut new_counter = 0;

    // let mut new = || {
    //     new_counter += 1;
    //     NodeRef::new((new_counter, DropChecker(&drop_counter)))
    // };
    setup!(new_counter, drop_counter, new);

    {
        let a = new();
        a.append(new());
        a.append(new());
        a.prepend(new());
        let b = new();
        b.append(a.clone());
        a.insert_before(new());
        a.insert_before(new());
        a.insert_after(new());
        a.insert_after(new());
        let c = new();
        b.append(c.clone());
        assert_eq!(drop_counter.get(), 0);
        c.previous_sibling().unwrap().detach();
        assert_eq!(drop_counter.get(), 1);

        assert_eq!(b.descendants().map(|node| {
            let borrow = node.borrow();
            borrow.0
        }).collect::<Vec<_>>(), vec![5, 6, 7, 1, 4, 2, 3, 9, 10]);
    }
    assert_eq!(drop_counter.get(), 10);
}

#[test]
fn test_create_tree() {
    let root = NodeRef::new(0);
    assert!(root.parent().is_none());
    assert!(root.first_child().is_none());
    assert!(root.last_child().is_none());
    assert!(root.next_sibling().is_none());
    assert!(root.previous_sibling().is_none());
}

#[test]
fn test_tree_ops() {
    setup_tree!(root);
    let mut traversed_node = root.descendants().collect::<Vec<_>>();

    let mut values: Vec<_> = traversed_node.iter().map(|node| *node.borrow()).collect();
    assert_eq!(values, [5, 6, 7, 1, 4, 2, 3, 9, 8, 10]);

    let node1 = traversed_node[3].clone();
    node1.detach();
    traversed_node = root.descendants().collect::<Vec<_>>();
    values = traversed_node.iter().map(|node| *node.borrow()).collect();
    assert_eq!(values, [5, 6, 7, 9, 8, 10]);

    let node7 = traversed_node[2].clone();
    let node6 = traversed_node[1].clone();
    let node9 = traversed_node[3].clone();
    let node10 = traversed_node[5].clone();

    assert!(node7.same_node(&node9.previous_sibling().unwrap()));
    assert!(node9.same_node(&node7.next_sibling().unwrap()));
    assert!(node6.same_node(&root.first_child().unwrap()));
    assert!(node10.same_node(&root.last_child().unwrap()));
    assert!(root.same_node(&node10.parent().unwrap()));

    let children_values: Vec<_> = root.children().map(|node| *node.borrow()).collect();
    assert_eq!(children_values, [6, 7, 9, 8, 10]);

    let reverse_children: Vec<_> = root.reverse_children().map(|node| *node.borrow()).collect();
    assert_eq!(reverse_children, [10, 8, 9, 7, 6]);
    
    let node7followings: Vec<_> = node7.following_siblings().map(|node| *node.borrow()).collect();
    assert_eq!(node7followings, [7, 9, 8, 10]);

    let node9precedings: Vec<_> = node9.preceding_siblings().map(|node| *node.borrow()).collect();
    assert_eq!(node9precedings, [9, 7, 6]);

    let ancestors: Vec<_> = node6.ancestors().map(|node| *node.borrow()).collect();
    assert_eq!(ancestors, [6, 5]);

    let found_node7 = root.find_node(&7).unwrap();
    assert_eq!(*found_node7.borrow(), 7);
    assert!(found_node7.parent().unwrap().same_node(&root));
    assert!(found_node7.next_sibling().unwrap().same_node(&node9));
    assert!(found_node7.previous_sibling().unwrap().same_node(&node6));
}

