/// A short-hand constructor for building a `&[u8]`.

#[allow(non_snake_case)]
#[inline]
pub fn B<'a, B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> &'a [u8] {
    bytes.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b() {
        let strs = vec![B("a"), B(b"xy")];
        println!("{:?}", strs); // [[97], [120, 121]]
    }
}
