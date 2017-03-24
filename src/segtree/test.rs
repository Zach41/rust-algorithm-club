use super::{SegmentTree, ValueOp};
use test::Bencher;

impl ValueOp for usize {
    fn op(self, rhs: usize) -> usize {
        self + rhs
    }
}

#[test]
fn test_query() {
    use std::iter;
    
    let seg_tree: SegmentTree<usize> = SegmentTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(seg_tree.query(0, 7), 36);
    assert_eq!(seg_tree.query(2, 5), 18);
    assert_eq!(seg_tree.query(1, 1), 2);

    let array: Vec<_> = iter::repeat(1).take(10000).collect();
    let seg_tree: SegmentTree<usize> = SegmentTree::new(&array);
    for i in 0..1000-100 {
        assert_eq!(seg_tree.query(i, 100+i), 101);
    }
}

#[test]
fn test_replace() {
    let mut seg_tree: SegmentTree<usize> = SegmentTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(seg_tree.query(0, 4), 15);
    seg_tree.replace(2, 10);
    assert_eq!(seg_tree.query(0, 4), 22);
    seg_tree.replace(0, 5);
    assert_eq!(seg_tree.query(0, 0), 5);
}

#[bench]
fn bench_query_segtree(b: &mut Bencher) {
    b.iter(|| {
        use std::iter;
        let array: Vec<_> = iter::repeat(1).take(100000).collect();
        let seg_tree: SegmentTree<usize> = SegmentTree::new(&array);
        for i in 0..100000 - 1000 {
            assert_eq!(seg_tree.query(i, i+1000), 1001);
        }
    });
}
