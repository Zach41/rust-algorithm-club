use super::*;

struct DropChecker<'a>(&'a cell::Cell<u32>);
impl<'a> Drop for DropChecker<'a> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

macro_rules! setup {
    () => {
        let drop_counter = cell::Cell::new(0);
        let mut new_counter = 0;

        let mut new = || {
            new_counter += 1;
            NodeRef::new((new_counter, DropChecker(&drop_counter)))
        };
    }
}

#[test]
fn tree_works() {
    let drop_counter = cell::Cell::new(0);
    let mut new_counter = 0;

    let mut new = || {
        new_counter += 1;
        NodeRef::new((new_counter, DropChecker(&drop_counter)))
    };

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
