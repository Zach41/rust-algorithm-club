use super::Trie;

#[test]
fn test_create() {
    let trie = Trie::new();

    assert!(trie.is_empty());
    assert_eq!(trie.words_count(), 0);
}

#[test]
fn test_insert() {
    let mut trie = Trie::new();
    trie.insert("cute");
    trie.insert("cutie");
    trie.insert("fred");
    assert!(trie.contains("cute"));
    assert!(!trie.contains("cut"));
    trie.insert("cut");
    assert!(trie.contains("cut"));
    assert_eq!(trie.words_count(), 4);
}

#[test]
fn test_remove() {
    let mut trie = Trie::new();    trie.insert("cute");
    trie.insert("cut");
    assert_eq!(trie.words_count(), 2);
    trie.remove("cute");
    assert!(trie.contains("cut"));
    assert!(!trie.contains("cute"));
    assert_eq!(trie.words_count(), 1);
}

#[test]
fn test_words() {
    let mut trie = Trie::new();
    let words = trie.words();
    assert!(words.is_empty());
    trie.insert("foobar");
    let words = trie.words();
    assert_eq!(words[0], "foobar".to_string());
    assert_eq!(words.len(), 1);

    trie.insert("foobaz");
    trie.insert("foobaa");
    trie.insert("bafoo");

    assert!(trie.contains("bafoo"));
    assert!(trie.contains("foobar"));
    assert!(trie.contains("foobaz"));
    assert!(trie.contains("foobaa"));
    
    let words = trie.words();
    println!("{:?}", words);
    assert_eq!(words.len(), 4);
    assert!(words.contains(&"foobar".to_string()));
    assert!(words.contains(&"foobaz".to_string()));
    assert!(words.contains(&"foobaa".to_string()));
    assert!(words.contains(&"bafoo".to_string()));
}

#[test]
fn test_find_prefix() {
    let mut trie = Trie::new();

    trie.insert("foobar");
    trie.insert("foobaa");
    trie.insert("foobaz");
    trie.insert("fozzzz");
    trie.insert("fz");

    trie.insert("bb");
    assert_eq!(trie.words_count(), 6);
    
    let words = trie.find_with_prefix("fo");
    assert_eq!(words.len(), 4);
    assert!(words.contains(&"foobar".to_string()));
    assert!(words.contains(&"foobaa".to_string()));
    assert!(words.contains(&"foobaz".to_string()));
    assert!(words.contains(&"fozzzz".to_string()));

    let words = trie.find_with_prefix("f");
    assert_eq!(words.len(), 5);
    assert!(words.contains(&"foobar".to_string()));
    assert!(words.contains(&"foobaa".to_string()));
    assert!(words.contains(&"foobaz".to_string()));
    assert!(words.contains(&"fozzzz".to_string()));
    assert!(words.contains(&"fz".to_string()));
}
