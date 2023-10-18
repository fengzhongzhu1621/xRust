use logger::eprintln_locked;

/// cargo expand --test test_eprintln_locked
#[test]
fn test_eprintln_locked() {
    eprintln_locked!("{} + {} = {}", 1, 2, 3);
}
