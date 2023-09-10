use xtool::stack::*;

#[test]
fn test_stack_pop() {
    let mut stack = Stack::new();
    assert_eq!(stack.pop(), None);

    stack.push('a');
    stack.push('b');
    stack.push('c');

    assert_eq!(stack.pop(), Some('c'));
    assert_eq!(stack.pop(), Some('b'));

    stack.push('d');
    stack.push('e');

    assert_eq!(stack.pop(), Some('e'));
    assert_eq!(stack.pop(), Some('d'));
    assert_eq!(stack.pop(), Some('a'));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.peek_mut(), None);

    stack.push('a');
    stack.push('b');
    stack.push('c');

    assert_eq!(stack.peek(), Some(&'c'));
    assert_eq!(stack.peek_mut(), Some(&mut 'c'));

    stack.peek_mut().map(|value| *value = 'd');

    assert_eq!(stack.peek(), Some(&'c'));
    assert_eq!(stack.pop(), Some('c'));
}

#[test]
fn test_into_iter() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');

    let mut iter = stack.into_iter();
    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), Some('b'));
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), None);

    // 所有已转移到into_iter
    // assert_eq!(stack.pop(), None);
}

#[test]
fn test_iter() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');

    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'a'));

    assert_eq!(stack.peek(), Some(&'c'));
}

#[test]
fn test_iter_mut() {
    let mut stack = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');

    let mut iter = stack.iter_mut();
    assert_eq!(iter.next(), Some(&mut 'c'));
    assert_eq!(iter.next(), Some(&mut 'b'));
    assert_eq!(iter.next(), Some(&mut 'a'));

    assert_eq!(stack.peek(), Some(&'c'));
}
