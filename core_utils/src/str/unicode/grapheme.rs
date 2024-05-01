use super::fsm::{
    grapheme_break_fwd::GRAPHEME_BREAK_FWD,
    grapheme_break_rev::GRAPHEME_BREAK_REV,
    regional_indicator_rev::REGIONAL_INDICATOR_REV,
};
use crate::str::{ext_slice::ByteSlice, utf8};
use regex_automata::{dfa::Automaton, Anchored, Input};

/// 创建一个字节切片的迭代器
/// An iterator over grapheme clusters in a byte string.
#[derive(Clone, Debug)]
pub struct Graphemes<'a> {
    // Graphemes 对象的作用域 < bs的作用域
    bs: &'a [u8],
}

impl<'a> Graphemes<'a> {
    pub(crate) fn new(bs: &'a [u8]) -> Graphemes<'a> {
        // Graphemes 对象的作用域 < bs的作用域
        Graphemes { bs }
    }

    /// View the underlying data as a subslice of the original data.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bs
    }
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str; // 迭代器的元素类型

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        // 将 self.bs 转换为迭代器并返回第一个分片
        let (grapheme, size) = decode_grapheme(self.bs);
        if size == 0 {
            return None;
        }
        self.bs = &self.bs[size..];
        Some(grapheme)
    }
}

/// 支持双向迭代
impl<'a> DoubleEndedIterator for Graphemes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let (grapheme, size) = decode_last_grapheme(self.bs);
        if size == 0 {
            return None;
        }
        self.bs = &self.bs[..self.bs.len() - size];
        Some(grapheme)
    }
}

/// An iterator over grapheme clusters in a byte string and their byte index
/// positions.
#[derive(Clone, Debug)]
pub struct GraphemeIndices<'a> {
    bs: &'a [u8],
    forward_index: usize, // 记录索引的位置
    reverse_index: usize,
}

impl<'a> GraphemeIndices<'a> {
    pub(crate) fn new(bs: &'a [u8]) -> GraphemeIndices<'a> {
        GraphemeIndices { bs, forward_index: 0, reverse_index: bs.len() }
    }

    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bs
    }
}

impl<'a> Iterator for GraphemeIndices<'a> {
    type Item = (usize, usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, &'a str)> {
        let index = self.forward_index;
        // 将 self.bs 转换为迭代器并返回第一个分片
        let (grapheme, size) = decode_grapheme(self.bs);
        if size == 0 {
            return None;
        }
        self.bs = &self.bs[size..];
        self.forward_index += size;
        // 返回分片所在的索引范围 (index, index + size]
        Some((index, index + size, grapheme))
    }
}

/// 支持双向迭代
impl<'a> DoubleEndedIterator for GraphemeIndices<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, usize, &'a str)> {
        let (grapheme, size) = decode_last_grapheme(self.bs);
        if size == 0 {
            return None;
        }
        self.bs = &self.bs[..self.bs.len() - size];
        self.reverse_index -= size;
        Some((self.reverse_index, self.reverse_index + size, grapheme))
    }
}

/// Decode a grapheme from the given byte string.
///
/// This returns the resulting grapheme (which may be a Unicode replacement
/// codepoint if invalid UTF-8 was found), along with the number of bytes
/// decoded in the byte string. The number of bytes decoded may not be the
/// same as the length of grapheme in the case where invalid UTF-8 is found.
pub fn decode_grapheme(bs: &[u8]) -> (&str, usize) {
    if bs.is_empty() {
        ("", 0)
    } else if bs.len() >= 2
        && bs[0].is_ascii()
        && bs[1].is_ascii()
        && !bs[0].is_ascii_whitespace()
    {
        // FIXME: It is somewhat sad that we have to special case this, but it
        // leads to a significant speed up in predominantly ASCII text. The
        // issue here is that the DFA has a bit of overhead, and running it for
        // every byte in mostly ASCII text results in a bit slowdown. We should
        // re-litigate this once regex-automata 0.3 is out, but it might be
        // hard to avoid the special case. A DFA is always going to at least
        // require some memory access.

        // Safe because all ASCII bytes are valid UTF-8.
        let grapheme = unsafe { bs[..1].to_str_unchecked() };
        (grapheme, 1)
    } else if let Some(hm) = {
        let input = Input::new(bs).anchored(Anchored::Yes);
        GRAPHEME_BREAK_FWD.try_search_fwd(&input).unwrap()
    } {
        // Safe because a match can only occur for valid UTF-8.
        let grapheme = unsafe { bs[..hm.offset()].to_str_unchecked() };
        (grapheme, grapheme.len())
    } else {
        const INVALID: &'static str = "\u{FFFD}";
        // No match on non-empty bytes implies we found invalid UTF-8.
        let (_, size) = utf8::decode_lossy(bs);
        (INVALID, size)
    }
}

fn decode_last_grapheme(bs: &[u8]) -> (&str, usize) {
    if bs.is_empty() {
        ("", 0)
    } else if let Some(hm) = {
        let input = Input::new(bs).anchored(Anchored::Yes);
        GRAPHEME_BREAK_REV.try_search_rev(&input).unwrap()
    } {
        let start = adjust_rev_for_regional_indicator(bs, hm.offset());
        // Safe because a match can only occur for valid UTF-8.
        let grapheme = unsafe { bs[start..].to_str_unchecked() };
        (grapheme, grapheme.len())
    } else {
        const INVALID: &'static str = "\u{FFFD}";
        // No match on non-empty bytes implies we found invalid UTF-8.
        let (_, size) = utf8::decode_last_lossy(bs);
        (INVALID, size)
    }
}

/// Return the correct offset for the next grapheme decoded at the end of the
/// given byte string, where `i` is the initial guess. In particular,
/// `&bs[i..]` represents the candidate grapheme.
///
/// `i` is returned by this function in all cases except when `&bs[i..]` is
/// a pair of regional indicator codepoints. In that case, if an odd number of
/// additional regional indicator codepoints precedes `i`, then `i` is
/// adjusted such that it points to only a single regional indicator.
///
/// This "fixing" is necessary to handle the requirement that a break cannot
/// occur between regional indicators where it would cause an odd number of
/// regional indicators to exist before the break from the *start* of the
/// string. A reverse regex cannot detect this case easily without look-around.
fn adjust_rev_for_regional_indicator(mut bs: &[u8], i: usize) -> usize {
    // All regional indicators use a 4 byte encoding, and we only care about
    // the case where we found a pair of regional indicators.
    if bs.len() - i != 8 {
        return i;
    }
    // Count all contiguous occurrences of regional indicators. If there's an
    // even number of them, then we can accept the pair we found. Otherwise,
    // we can only take one of them.
    //
    // FIXME: This is quadratic in the worst case, e.g., a string of just
    // regional indicator codepoints. A fix probably requires refactoring this
    // code a bit such that we don't rescan regional indicators.
    let mut count = 0;
    while let Some(hm) = {
        let input = Input::new(bs).anchored(Anchored::Yes);
        REGIONAL_INDICATOR_REV.try_search_rev(&input).unwrap()
    } {
        bs = &bs[..hm.offset()];
        count += 1;
    }
    if count % 2 == 0 {
        i
    } else {
        i + 4
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphemes() {
        let mut graphemes = Graphemes::new(b"abc");
        assert_eq!(graphemes.next(), Some("a"));
        assert_eq!(graphemes.next(), Some("b"));
        assert_eq!(graphemes.next(), Some("c"));
        assert_eq!(graphemes.next(), None);

        let mut graphemes = Graphemes::new("a̐éö̲\r\n".as_bytes());
        assert_eq!(graphemes.next(), Some("a̐"));
        assert_eq!(graphemes.next(), Some("é"));
        assert_eq!(graphemes.next(), Some("ö̲"));
        assert_eq!(graphemes.next(), Some("\r\n"));
        assert_eq!(graphemes.next(), None);

        let mut graphemes = Graphemes::new("a你好b".as_bytes());
        assert_eq!(graphemes.next(), Some("a"));
        assert_eq!(graphemes.next(), Some("你"));
        assert_eq!(graphemes.next(), Some("好"));
        assert_eq!(graphemes.next(), Some("b"));
        assert_eq!(graphemes.next(), None);

        let mut graphemes = Graphemes::new("a你好b".as_bytes());
        assert_eq!(graphemes.next_back(), Some("b"));
        assert_eq!(graphemes.next_back(), Some("好"));
    }

    #[test]
    fn test_graphemeIndices() {
        let mut graphemes = GraphemeIndices::new("a你好b".as_bytes());
        assert_eq!(graphemes.next(), Some((0, 1, "a"))); // 占一个字节
        assert_eq!(graphemes.next(), Some((1, 4, "你"))); // 占三个字节
        assert_eq!(graphemes.next(), Some((4, 7, "好"))); // 占三个字节
        assert_eq!(graphemes.next(), Some((7, 8, "b"))); // 占一个字节
        assert_eq!(graphemes.next(), None);
    }
}