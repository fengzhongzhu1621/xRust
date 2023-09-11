use xtool::unsafe_link::*;

#[test]
fn test_unsafe_link_pop() {
    let mut link = UnSafeLink::new();
    assert_eq!(link.pop(), None);

    link.push('a');
    link.push('b');
    link.push('c');

    assert_eq!(link.pop(), Some('a'));
    assert_eq!(link.pop(), Some('b'));

    link.push('d');
    link.push('e');

    assert_eq!(link.pop(), Some('c'));
    assert_eq!(link.pop(), Some('d'));
    assert_eq!(link.pop(), Some('e'));
    assert_eq!(link.pop(), None);
}


#[test]
fn test_unsafe_link_into_iter() {
    let mut link = UnSafeLink::new();
    link.push('a');
    link.push('b');
    link.push('c');

    let mut iter = link.into_iter();
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), Some('b'));
    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), None);

    // 所有已转移到into_iter
    // assert_eq!(link.pop(), None);
}

#[test]
fn test_unsafe_link_iter() {
    let mut link = UnSafeLink::new();
    link.push('a');
    link.push('b');
    link.push('c');

    let mut iter = link.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));

    assert_eq!(link.peek(), Some(&'a'));
}

#[test]
fn test_unsafe_link_iter_mut() {
    let mut link = UnSafeLink::new();
    link.push('a');
    link.push('b');
    link.push('c');

    let mut iter = link.iter_mut();
    assert_eq!(iter.next(), Some(&mut 'a'));
    assert_eq!(iter.next(), Some(&mut 'b'));
    assert_eq!(iter.next(), Some(&mut 'c'));

    assert_eq!(link.peek(), Some(&'a'));
}

#[test]
fn test_unsafe_link_peek_mut() {
    let mut link = UnSafeLink::new();
    link.push('a');
    link.push('b');
    link.push('c');

    link.peek_mut().map(|x| *x = 'd');
    assert_eq!(link.peek(), Some(&'d'));
}
