use xrust::search;

#[test]
fn one_result() {
    let query = "one";
    let contents = "\
one
two
three";
    let actual = search(query, contents);
    let expect = vec!["one"];
    assert_eq!(actual, expect);
}
