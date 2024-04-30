use unicode_segmentation::UnicodeSegmentation;

#[test]
fn test_unicode_segmentation() {
    let s = "a̐éö̲\r\n";
    let g = s.graphemes(true).collect::<Vec<&str>>();
    let b: &[_] = &["a̐", "é", "ö̲", "\r\n"];
    assert_eq!(g, b);

    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
    let w = s.unicode_words().collect::<Vec<&str>>();
    let b: &[_] = &[
        "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet",
        "right",
    ];
    assert_eq!(w, b);

    let s = "The quick (\"brown\")  fox";
    let w = s.split_word_bounds().collect::<Vec<&str>>();
    let b: &[_] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", "  ", "fox",
    ];
    assert_eq!(w, b);

    // Returns an iterator over the grapheme clusters of self and their byte offsets. See graphemes() for more information.
    let gr_inds = UnicodeSegmentation::grapheme_indices("a̐éö̲\r\n", true)
        .collect::<Vec<(usize, &str)>>();
    let b: &[_] = &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")];
    assert_eq!(&gr_inds[..], b);
}
