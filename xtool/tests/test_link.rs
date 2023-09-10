use xtool::link::LinkList;

#[test]
fn test_list() {
    let mut list = LinkList::new();

    list.append("a".to_string());
    list.append("b".to_string());
    list.append("c".to_string());
    assert_eq!(list.get_length(), 3);

    assert_eq!(list.pop(), Some("a".to_string()));
    assert_eq!(list.get_length(), 2);
    assert_eq!(list.pop(), Some("b".to_string()));
    assert_eq!(list.get_length(), 1);
    assert_eq!(list.pop(), Some("c".to_string()));
    assert_eq!(list.get_length(), 0);

    assert_eq!(list.pop(), None);
    assert_eq!(list.get_length(), 0);
    assert_eq!(list.pop(), None);
    assert_eq!(list.get_length(), 0);
}
