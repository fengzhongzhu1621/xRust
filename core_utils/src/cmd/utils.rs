use bstr::ByteSlice;
use std;

fn format_bytes(
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

    let lines_total = data.as_bstr().lines_with_terminator().count();
    let multiline = 1 < lines_total;

    if LINES_MIN_OVERFLOW <= lines_total {
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

fn write_debug_bstrs<'a>(
    f: &mut impl std::fmt::Write,
    multiline: bool,
    mut lines: impl Iterator<Item = &'a [u8]>,
) -> std::fmt::Result {
    if multiline {
        writeln!(f, "```")?;
        for mut line in lines {
            let mut newline = false;
            if line.last() == Some(&b'\n') {
                line = &line[..line.len() - 1];
                newline = true;
            }
            let s = format!("{:?}", line.as_bstr());
            write!(
                f,
                "{}{}",
                &s[1..s.len() - 1],
                if newline { "\n" } else { "" }
            )?;
        }
        writeln!(f, "```")
    } else {
        write!(f, "{:?}", lines.next().unwrap_or(&[]).as_bstr())
    }
}

fn write_debug_bstrs<'a>(
    f: &mut impl std::fmt::Write,
    multiline: bool,
    mut lines: impl Iterator<Item = &'a [u8]>,
) -> std::fmt::Result {
    if multiline {
        writeln!(f, "```")?;
        for mut line in lines {
            let mut newline = false;
            if line.last() == Some(&b'\n') {
                line = &line[..line.len() - 1];
                newline = true;
            }
            let s = format!("{:?}", line.as_bstr());
            write!(
                f,
                "{}{}",
                &s[1..s.len() - 1],
                if newline { "\n" } else { "" }
            )?;
        }
        writeln!(f, "```")
    } else {
        // 打印迭代器的下一个值，迭代器的值是 [u8] 类型
        write!(f, "{:?}", lines.next().unwrap_or(&[]).as_bstr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_debug_bstrs() {}
}
