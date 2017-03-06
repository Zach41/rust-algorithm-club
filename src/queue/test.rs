use super::Queue;

#[test]
fn test_empty() {
    let mut queue: Queue<usize> = Queue::new();

    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
}

#[test]
fn test_ops() {
    let mut queue: Queue<usize> = Queue::new();

    queue.enqueue(1);
    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 1);
    assert_eq!(queue.front(), Some(1));
    assert_eq!(queue.dequeue(), Some(1));
    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
    assert_eq!(queue.dequeue(), None);

    for i in 0..100 {
        queue.enqueue(i);
    }
    assert_eq!(queue.count(), 100);
    assert_eq!(queue.front(), Some(0));

    for i in 0..100 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    assert_eq!(queue.count(), 0);
    assert!(queue.is_empty());
    assert_eq!(queue.dequeue(), None);
}
