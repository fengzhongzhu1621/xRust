use flurry;


pub fn flurry_hashset() {
    // Initialize a new hash set.
    let books = flurry::HashSet::new();
    let guard = books.guard();

    // Add some books
    books.insert("Fight Club", &guard);
    books.insert("Three Men In A Raft", &guard);
    books.insert("The Book of Dust", &guard);
    books.insert("The Dry", &guard);

    // Check for a specific one.
    if !books.contains(&"The Drunken Botanist", &guard) {
        println!("We don't have The Drunken Botanist.");
    }

    // Remove a book.
    books.remove(&"Three Men In A Raft", &guard);

    // Iterate over everything.
    for book in books.iter(&guard) {
        println!("{}", book);
    }
}