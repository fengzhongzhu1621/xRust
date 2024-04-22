use core_utils::iterator::normalized;
use std::iter::FromIterator;

#[test]
fn test_normalized() {
    let input =
        "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
    assert_eq!(
        &String::from_iter(normalized(input.chars())),
        "This is a string \n with \n some \n\n random newlines\n\n\n"
    );
    assert_eq!(
        normalized(input.chars()).collect::<String>(),
        "This is a string \n with \n some \n\n random newlines\n\n\n"
    );
}
