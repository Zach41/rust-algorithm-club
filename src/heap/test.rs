extern crate env_logger;

use super::Heap;

#[test]
fn test_empty_heap() {
    let mut heap: Heap<usize> = Heap::new();

    assert_eq!(heap.count(), 0);
    assert_eq!(heap.peek(), None);
    assert_eq!(heap.remove(), None);
    assert!(heap.is_empty());
}

#[test]
fn test_is_empty() {
    let mut heap: Heap<usize> = Heap::new();
    assert!(heap.is_empty());
    heap.insert(1);
    assert!(!heap.is_empty());
    let _ = heap.remove();
    assert!(heap.is_empty());
}

#[test]
fn test_count() {
    let mut heap: Heap<usize> = Heap::new();
    assert_eq!(heap.count(), 0);
    heap.insert(1);
    assert_eq!(heap.count(), 1);
}

#[test]
fn test_create_heap() {
    let v = vec![1, 5, 4, 2, 3, 6, 7];
    
    let mut heap: Heap<usize> = Heap::with_array(&v);
    
    assert_eq!(heap.count(), 7);
    assert_eq!(heap.peek(), Some(7));
    assert_eq!(heap.remove(), Some(7));
    assert_eq!(heap.remove(), Some(6));
    assert_eq!(heap.remove(), Some(5));
    assert_eq!(heap.remove(), Some(4));
    assert_eq!(heap.remove(), Some(3));
    assert_eq!(heap.remove(), Some(2));
    assert_eq!(heap.remove(), Some(1));
    assert_eq!(heap.remove(), None);    
}

#[test]
fn test_create_heap2() {
    let v = vec![1, 5, 4, 2, 3, 6, 7];

    let mut heap: Heap<usize> = Heap::new();
    for value in v {
        heap.insert(value);
    }

    assert_eq!(heap.count(), 7);
    assert!(!heap.is_empty());
    assert_eq!(heap.peek(), Some(7));
    assert_eq!(heap.remove(), Some(7));
    assert_eq!(heap.remove(), Some(6));
    assert_eq!(heap.remove(), Some(5));
    assert_eq!(heap.remove(), Some(4));
    assert_eq!(heap.remove(), Some(3));
    assert_eq!(heap.remove(), Some(2));
    assert_eq!(heap.remove(), Some(1));
    assert_eq!(heap.remove(), None);
}

#[test]
fn test_remove() {
    let array = vec![100, 50, 70, 10, 20, 60, 65];
    let mut heap: Heap<usize> = Heap::with_array(&array);

    assert_eq!(heap.count(), 7);
    assert_eq!(heap.remove(), Some(100));
    assert_eq!(heap.count(), 6);
    assert_eq!(heap.remove(), Some(70));
    assert_eq!(heap.count(), 5);
    assert_eq!(heap.remove(), Some(65));
    assert_eq!(heap.count(), 4);
    assert_eq!(heap.remove(), Some(60));
    assert_eq!(heap.count(), 3);
    assert_eq!(heap.remove(), Some(50));
    assert_eq!(heap.count(), 2);
    assert_eq!(heap.remove(), Some(20));
    assert_eq!(heap.count(), 1);
    assert_eq!(heap.remove(), Some(10));
    assert_eq!(heap.count(), 0);
    assert_eq!(heap.remove(), None);
}
