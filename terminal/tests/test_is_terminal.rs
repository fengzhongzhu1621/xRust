use terminal::terminal::*;

#[test]
fn test_is_terminal() {
    if std::io::stdout().is_terminal() {
        println!("stdout is a terminal")
    }
}
