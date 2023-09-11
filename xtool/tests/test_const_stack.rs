use xtool::const_stack::*;

#[test]
fn test_const_stack() {
    let mut stack = ConstStack::new();

    // 仅返回栈顶元素，不出栈
    assert_eq!(stack.head(), None);

    // 压栈
    let stack = stack.prepend('a');
    let stack = stack.prepend('b');
    let stack = stack.prepend('c');
    assert_eq!(stack.head(), Some(&'c'));

    // 出栈
    let stack = stack.tail();
    assert_eq!(stack.head(), Some(&'b'));
    let stack = stack.tail();
    assert_eq!(stack.head(), Some(&'a'));
    let stack = stack.tail();
    assert_eq!(stack.head(), None);
    let stack = stack.tail();
    assert_eq!(stack.head(), None);
}

#[test]
fn test_const_stack_iter() {
    let mut stack = ConstStack::new();

    // 压栈
    let stack = stack.prepend('a').prepend('b').prepend('c');

    let mut iter = stack.iter();
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'a'));
}
