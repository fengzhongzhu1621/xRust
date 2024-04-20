#[test]
fn test_iter() {
    let x = Some(4);
    assert_eq!(x.iter().next(), Some(&4));

    let x: Option<u32> = None;
    assert_eq!(x.iter().next(), None);
    assert_eq!(x.iter().next(), None);

    let y = Some("abc");
    let mut leaves: Vec<char> = vec![];
    leaves.extend(y.iter().flat_map(|s| s.chars()));
    assert_eq!(leaves, ['a', 'b', 'c',]);

    let _y: Option<u32> = None;
    // Err
    // leaves.extend(y.iter().flat_map(|s| s.chars()));
}

#[test]
fn test_unwrap_or_default() {
    let x: Option<u32> = None;
    let y: Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 12);
}

#[test]
fn test_map() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);
}
