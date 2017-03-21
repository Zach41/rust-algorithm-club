use super::BitSet;

#[test]
fn test_create() {
    let bitset = BitSet::new(66);

    assert_eq!(bitset.size, 66);
    assert!(bitset.all0());
    assert!(!bitset.any1());
    assert!(!bitset.all1());
    assert!(!bitset.test(0));
    assert_eq!(bitset.cardinality(), 0);
    assert_eq!(bitset[0], false);
}

#[test]
fn test_ops() {
    let mut bitset = BitSet::new(100);

    bitset.set_all();
    assert!(bitset.all1());
    assert!(bitset.any1());
    assert!(!bitset.all0());
    assert_eq!(bitset.cardinality(), 100);
    println!("{:?}", bitset);
    for i in 0..100 {
        assert!(bitset.test(i));
    }
    

    bitset.clear(0);
    bitset.clear(99);
    assert!(!bitset[0]);
    assert!(!bitset.test(99));
    assert_eq!(bitset.cardinality(), 98);
    assert!(bitset.any1());
    assert!(!bitset.all1());
    assert!(!bitset.all0());

    let flipped = bitset.flip(1);
    assert!(!flipped);
    assert!(!bitset[1]);
    assert_eq!(bitset.cardinality(), 97);

    bitset.clear_all();
    assert_eq!(bitset.cardinality(), 0);
    assert!(bitset.all0());
    assert!(!bitset.any1());
    assert!(!bitset.all1());
    for i in 0..100 {
        assert!(!bitset.test(i));
    }    
}

#[test]
fn test_operations() {
    let mut bita = BitSet::new(4);
    let mut bitb = BitSet::new(8);

    bita.set(0);
    bita.set(2);
    bita.set(3);

    bitb.set(1);

    let bit_or = bita.clone() | bitb.clone();
    assert_eq!(bit_or.cardinality(), 4);
    for i in 0..4 {
        assert!(bit_or.test(i));
    }

    let bit_and = bita.clone() & bitb.clone();
    assert_eq!(bit_and.cardinality(), 0);
    assert!(bit_and.all0());

    let bit_xor = bita.clone() ^ bitb.clone();
    assert_eq!(bit_xor, bit_or);

    let bit_not = !bita.clone();
    assert_eq!(bit_not.cardinality(), 1);
    assert!(bit_not.test(1));

    let mut a = bita.clone();
    a &= bitb.clone();
    assert_eq!(a, bit_and);

    a = bita.clone();
    a |= bitb.clone();
    assert_eq!(a, bit_or);

    a = bita.clone();
    a ^= bitb;
    assert_eq!(a, bit_xor);
    
}
