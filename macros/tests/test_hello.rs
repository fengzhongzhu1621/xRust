use macros::hello;

#[test]
fn test_hello() {
    hello!("world!");
    hello!();
}
