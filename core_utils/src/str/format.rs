use bstr::ByteSlice;
use std;

#[derive(Debug)]
pub struct DebugBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> DebugBytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        DebugBytes { bytes }
    }
}

impl<'a> std::fmt::Display for DebugBytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_bytes(self.bytes, f)
    }
}

#[derive(Debug)]
pub struct DebugBuffer {
    buffer: bstr::BString,
}

impl DebugBuffer {
    pub fn new(buffer: Vec<u8>) -> Self {
        // Vec<u8> -> bstr::BString
        DebugBuffer { buffer: buffer.into() }
    }
}

impl std::fmt::Display for DebugBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_bytes(&self.buffer, f)
    }
}

/// 打印字符串，当行数过多时自动忽略部分行
pub fn format_bytes(
    data: &[u8],
    f: &mut impl std::fmt::Write,
) -> std::fmt::Result {
    #![allow(clippy::assertions_on_constants)]

    const LINES_MIN_OVERFLOW: usize = 80;
    const LINES_MAX_START: usize = 20;
    const LINES_MAX_END: usize = 40;
    const LINES_MAX_PRINTED: usize = LINES_MAX_START + LINES_MAX_END;

    const BYTES_MIN_OVERFLOW: usize = 8192;
    const BYTES_MAX_START: usize = 2048;
    const BYTES_MAX_END: usize = 2048;
    const BYTES_MAX_PRINTED: usize = BYTES_MAX_START + BYTES_MAX_END;

    assert!(LINES_MAX_PRINTED < LINES_MIN_OVERFLOW);
    assert!(BYTES_MAX_PRINTED < BYTES_MIN_OVERFLOW);

    // 计算行数
    let lines_total = data.as_bstr().lines_with_terminator().count();
    let multiline = 1 < lines_total;

    if LINES_MIN_OVERFLOW <= lines_total {
        // 需要忽略的行数，不打印
        let lines_omitted = lines_total - LINES_MAX_PRINTED;
        let start_lines =
            data.as_bstr().lines_with_terminator().take(LINES_MAX_START);
        let end_lines = data
            .as_bstr()
            .lines_with_terminator()
            .skip(LINES_MAX_START + lines_omitted);
        writeln!(f, "<{} lines total>", lines_total)?;
        write_debug_bstrs(f, true, start_lines)?;
        writeln!(f, "<{} lines omitted>", lines_omitted)?;
        write_debug_bstrs(f, true, end_lines)
    } else if BYTES_MIN_OVERFLOW <= data.len() {
        write!(
            f,
            "<{} bytes total>{}",
            data.len(),
            if multiline { "\n" } else { "" }
        )?;
        write_debug_bstrs(
            f,
            multiline,
            data[..BYTES_MAX_START].lines_with_terminator(),
        )?;
        write!(
            f,
            "<{} bytes omitted>{}",
            data.len() - BYTES_MAX_PRINTED,
            if multiline { "\n" } else { "" }
        )?;
        write_debug_bstrs(
            f,
            multiline,
            data[data.len() - BYTES_MAX_END..].lines_with_terminator(),
        )
    } else {
        write_debug_bstrs(f, multiline, data.lines_with_terminator())
    }
}

/// 打印 bstr 字符串
pub fn write_debug_bstrs<'a>(
    f: &mut impl std::fmt::Write,
    multiline: bool,
    mut lines: impl Iterator<Item = &'a [u8]>, // 字节迭代器
) -> std::fmt::Result {
    if multiline {
        writeln!(f, "```")?;
        for mut line in lines {
            let mut newline = false;
            // 判断最后一个字符是否是换行符
            if line.last() == Some(&b'\n') {
                // 去掉换行符
                line = &line[..line.len() - 1];
                newline = true;
            }
            // 打印行
            let s = format!("{:?}", line.as_bstr());
            write!(
                f,
                "{}{}",
                &s[1..s.len() - 1], // 去掉双引号
                if newline { "\n" } else { "" }
            )?;
        }
        writeln!(f, "```")
    } else {
        // 打印字节迭代器的下一个值
        write!(f, "{:?}", lines.next().unwrap_or(&[]).as_bstr())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_bytes() {
        let mut s = String::new();
        for i in 0..80 {
            s.push_str(&format!("{}\n", i));
        }

        let mut buf = String::new();
        format_bytes(s.as_bytes(), &mut buf).unwrap();

        assert_eq!(
            "<80 lines total>
```
0
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
```
<20 lines omitted>
```
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
```
",
            buf
        );
    }

    #[test]
    fn test_no_trailing_newline() {
        let s = "no\ntrailing\nnewline";

        let mut buf = String::new();
        format_bytes(s.as_bytes(), &mut buf).unwrap();

        assert_eq!(
            "```
no
trailing
newline```
",
            buf
        );
    }
}
