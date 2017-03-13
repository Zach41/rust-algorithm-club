use super::UnionFind;

#[test]
fn test_int() {
    let mut dsu_int: UnionFind<usize> = UnionFind::new();

    for i in 1..11 {
        dsu_int.add_set(i);
    }

    for i in 3..11 {
        if i % 2 == 0 {
            dsu_int.union_sets(&2, &i);
        } else {
            dsu_int.union_sets(&1, &i);
        }
    }

    assert!(dsu_int.same_set(&2, &4));
    assert!(dsu_int.same_set(&4, &6));
    assert!(dsu_int.same_set(&6, &8));
    assert!(dsu_int.same_set(&8, &10));

    assert!(dsu_int.same_set(&1, &3));
    assert!(dsu_int.same_set(&3, &5));
    assert!(dsu_int.same_set(&5, &7));
    assert!(dsu_int.same_set(&7, &9));

    assert!(!dsu_int.same_set(&1, &2));
    assert!(!dsu_int.same_set(&3, &6));
}

#[test]
fn test_string() {
    let mut dsu_string: UnionFind<String> = UnionFind::new();

    dsu_string.add_set("a".to_string());
    dsu_string.add_set("b".to_string());

    let words = vec!["awesome", "all", "after", "boy", "ball", "basket"];

    for word in words {
        let string = word.to_owned();
        dsu_string.add_set(string.clone());
        if word.starts_with("a") {
            dsu_string.union_sets(&"a".to_string(), &string);
        } else if word.starts_with("b") {
            dsu_string.union_sets(&"b".to_string(), &string);
        }
    }
    
    assert!(dsu_string.same_set(&"a".to_string(), &"awesome".to_string()));
    assert!(dsu_string.same_set(&"awesome".to_string(), &"all".to_string()));
    assert!(dsu_string.same_set(&"all".to_string(), &"after".to_string()));

    assert!(dsu_string.same_set(&"b".to_string(), &"boy".to_string()));
    assert!(dsu_string.same_set(&"boy".to_string(), &"ball".to_string()));
    assert!(dsu_string.same_set(&"ball".to_string(), &"basket".to_string()));

    assert!(!dsu_string.same_set(&"a".to_string(), &"b".to_string()));
    assert!(!dsu_string.same_set(&"ball".to_string(), &"after".to_string()));            
}
