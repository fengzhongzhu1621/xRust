use xtool::numbers::Numbers;

#[test]
fn test_numbers() {
    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut numbers = Numbers::new(&xs);
    while let Some(x) = numbers.next_even() {
        println!("{}", x);
    }
}
