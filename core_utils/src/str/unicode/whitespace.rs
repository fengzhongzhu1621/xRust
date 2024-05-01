use regex_automata::{dfa::Automaton, Anchored, Input};

use super::fsm::{
    whitespace_anchored_fwd::WHITESPACE_ANCHORED_FWD,
    whitespace_anchored_rev::WHITESPACE_ANCHORED_REV,
};

/// 获取第一个非空字符的索引
/// Return the first position of a non-whitespace character.
pub fn whitespace_len_fwd(slice: &[u8]) -> usize {
    let input = Input::new(slice).anchored(Anchored::Yes);
    WHITESPACE_ANCHORED_FWD
        .try_search_fwd(&input)
        .unwrap()
        .map_or(0, |hm| hm.offset())
}

/// 获得最后一个非空字符的索引 + 1
/// Return the last position of a non-whitespace character.
pub fn whitespace_len_rev(slice: &[u8]) -> usize {
    let input = Input::new(slice).anchored(Anchored::Yes);
    WHITESPACE_ANCHORED_REV
        .try_search_rev(&input)
        .unwrap()
        .map_or(slice.len(), |hm| hm.offset())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_len_fwd() {
        let str = b"  a bc ";
        assert_eq!(whitespace_len_fwd(str), 2); // a的索引
        assert_eq!(str[2], b"a"[0]);

        let str = b"a bc ";
        assert_eq!(whitespace_len_fwd(str), 0); // a的索引
        assert_eq!(str[0], b"a"[0]);
    }

    #[test]
    fn test_whitespace_len_rev() {
        assert_eq!(whitespace_len_rev(b"  a bc "), 6); // c 后面的索引
        assert_eq!(whitespace_len_rev(b"  a bc"), 6); // 数组的长度
    }
}
