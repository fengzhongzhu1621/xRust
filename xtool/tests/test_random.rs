use xtool::random::*;

#[test]
fn test_random_string() {
    let s = random_string(32);
    assert_eq!(s.len(), 32);
}
