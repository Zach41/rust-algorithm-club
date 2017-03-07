use std::cmp::Ordering;

use super::PriorityQueue;

#[derive(Clone, Debug, PartialEq)]
struct Message {
    text: String,
    priority: usize,
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Message) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Message {
    fn new(text: &str, priority: usize) -> Message {
        Message {
            text: text.to_string(),
            priority: priority,
        }
    }
}

#[test]
fn testEmpty() {
    let mut queue: PriorityQueue<Message> = PriorityQueue::new();

    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
    assert_eq!(queue.peek(), None);
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn testOneElement() {
    let mut queue: PriorityQueue<Message> = PriorityQueue::new();

    queue.enqueue(Message::new("Zach", 12));
    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 1);
    assert_eq!(queue.peek(), Some(Message::new("Zach", 12)));

    let result = queue.dequeue();
    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
    assert_eq!(queue.peek(), None);
    assert_eq!(result, Some(Message::new("Zach", 12)));
}

#[test]
fn testTwoElement() {
    let mut queue: PriorityQueue<Message> = PriorityQueue::new();

    queue.enqueue(Message::new("Zach", 12));
    queue.enqueue(Message::new("Zach2", 13));

    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 2);
    assert_eq!(queue.peek(), Some(Message::new("Zach2", 13)));

    let mut result = queue.dequeue();
    assert_eq!(result, Some(Message::new("Zach2", 13)));
    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 1);
    assert_eq!(queue.peek(), Some(Message::new("Zach", 12)));

    result = queue.dequeue();
    assert_eq!(result, Some(Message::new("Zach", 12)));
    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
    assert_eq!(queue.peek(), None);
}

#[test]
fn testOutOfOrder() {
    let mut queue: PriorityQueue<Message> = PriorityQueue::new();

    queue.enqueue(Message::new("Zach2", 13));
    queue.enqueue(Message::new("Zach", 12));

    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 2);
    assert_eq!(queue.peek(), Some(Message::new("Zach2", 13)));

    let mut result = queue.dequeue();
    assert_eq!(result, Some(Message::new("Zach2", 13)));
    assert!(!queue.is_empty());
    assert_eq!(queue.count(), 1);
    assert_eq!(queue.peek(), Some(Message::new("Zach", 12)));

    result = queue.dequeue();
    assert_eq!(result, Some(Message::new("Zach", 12)));
    assert!(queue.is_empty());
    assert_eq!(queue.count(), 0);
    assert_eq!(queue.peek(), None);
}

