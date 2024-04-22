/// 迭代器的包装类，用于添加一些额外的参数
struct Normalized<I> {
    iter: I,
    prev_was_cr: bool,
}

#[inline]
pub fn normalized(
    iter: impl Iterator<Item = char>,
) -> impl Iterator<Item = char> {
    Normalized { iter, prev_was_cr: false }
}

/// Normalized 支持迭代
impl<I> Iterator for Normalized<I>
where
    I: Iterator<Item = char>,
{
    // 元素的类型
    type Item = char;

    // 将\r\n 替换为 \n，将其余的\r替换为\n
    fn next(&mut self) -> Option<char> {
        match self.iter.next() {
            Some('\n') if self.prev_was_cr => {
                self.prev_was_cr = false;
                match self.iter.next() {
                    Some('\r') => {
                        self.prev_was_cr = true;
                        Some('\n')
                    }
                    any => {
                        self.prev_was_cr = false;
                        any
                    }
                }
            }
            Some('\r') => {
                self.prev_was_cr = true;
                Some('\n')
            }
            any => {
                self.prev_was_cr = false;
                any
            }
        }
    }
}
