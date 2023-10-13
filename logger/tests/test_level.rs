extern crate logger;

pub use logger::*;

#[test]
fn test_parse() {
    let actual = "ERROR".parse::<Level>();
    let expect = Ok(Level::Error);
    assert_eq!(actual, expect);

    let actual = "OFF".parse::<Level>();
    let expect = Err(ParseLevelError(()));
    assert_eq!(actual, expect);
}
