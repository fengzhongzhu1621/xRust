use std::str;

#[test]
fn test_find() {
    // create some strings
    let str1 = "Linux is the best operation!";
    let str2 = "Rust";
    let str3 = "Welcome to Linux";
    let str4 = "Programming";

    // create the matches
    let match1 = "is";
    let match2 = 'R';
    let match3 = "to";
    let match4 = "23";

    // find the matches and print byte indices
    assert_eq!(str1.find(match1), Some(6));
    assert_eq!(str2.find(match2), Some(0));
    assert_eq!(str3.find(match3), Some(8));
    assert_eq!(str4.find(match4), None);
}

#[test]
fn test_len() {
    let str1 = "1你好2";
    assert_eq!(str1.len(), 8);
}

#[test]
fn test_lines() {
    let text = "foo\r\nbar\n\nbaz\n";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());

    assert_eq!(None, lines.next());
    assert_eq!(None, lines.next());
}

#[test]
fn test_split() {
    let test = "cat;bird";
    let values: Vec<_> = test.split(';').collect();
    assert_eq!(values, vec!["cat", "bird"]);

    let values: Vec<_> = test.split("cat;").collect();
    assert_eq!(values, vec!["", "bird"]);

    let values: Vec<_> = test.split("xxx").collect();
    assert_eq!(values, vec!["cat;bird"]);

    fn whitespace_test(c: char) -> bool {
        return c == ' ' || c == '\n';
    }

    let test = "cat dog\nbird";
    let values: Vec<_> = test.split(whitespace_test).collect();
    assert_eq!(values, vec!["cat", "dog", "bird"]);
}

#[test]
fn test_split_whitespace() {
    let terms = "bird frog tree\n?\t!";
    let values: Vec<_> = terms.split_whitespace().collect();
    assert_eq!(values, vec!["bird", "frog", "tree", "?", "!"]);
}

#[test]
fn test_split_one() {
    let value = "left:right";
    let (left, right) = value.split_once(":").unwrap();
    assert_eq!(left, "left");
    assert_eq!(right, "right");
}

#[test]
fn test_replace() {
    let result = str::replace("Hello World!", "!", "?");
    assert_eq!(result, "Hello World?");

    let result = str::replace("Hello\tWorld!", '\t', "\\t");
    assert_eq!(result, r"Hello\tWorld!");

    let result: String = "Hello, world!"
        .chars()
        .map(|x| match x {
            '!' => '?',
            'A'..='Z' => 'X',
            'a'..='z' => 'x',
            _ => x,
        })
        .collect();
    assert_eq!(result, "Xxxxx, xxxxx?");
}

#[test]
fn test_print() {
    println!("{}", "'Hello' 'World!'");
    println!("{}", r#"'Hello' 'World!'"#);
    println!("{}", r#"'{{}}'"#);
}

#[test]
fn test_split_terminator() {
    let v: Vec<&str> = "A.B".split_terminator('.').collect();
    assert_eq!(v, ["A", "B"]);

    let v: Vec<&str> = "A.B.".split_terminator('.').collect();
    assert_eq!(v, ["A", "B"]);

    let v: Vec<&str> = "A..B..".split_terminator(".").collect();
    assert_eq!(v, ["A", "", "B", ""]);
}
