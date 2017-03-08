use super::Stack;

#[test]
fn test_empty() {
    let mut stack: Stack<usize> = Stack::new();
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.top(), None);
}

#[test]
fn test_one_element() {
    let mut stack: Stack<usize> = Stack::new();
    stack.push(1);
    assert_eq!(stack.len(), 1);
    assert!(!stack.is_empty());
    assert_eq!(stack.top(), Some(&1));

    let result = stack.pop();
    assert_eq!(result, Some(1));
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.top(), None);
}

#[test]
fn test_two_element() {
    let mut stack: Stack<usize> = Stack::new();
    stack.push(123);
    stack.push(456);

    assert_eq!(stack.len(), 2);
    assert!(!stack.is_empty());
    assert_eq!(stack.top(), Some(&456));

    let mut top = stack.pop();
    assert_eq!(top, Some(456));
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.top(), Some(&123));
    assert!(!stack.is_empty());

    top = stack.pop();
    assert_eq!(top, Some(123));
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.top(), None);
    assert!(stack.is_empty());
}
