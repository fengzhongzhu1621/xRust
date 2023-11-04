pub type Value<'a> = str;

///  定义返回 key value 格式的 trait
pub trait KVs<'a> {
    fn into_kvs(self) -> Option<&'a [(&'a str, &'a Value<'a>)]>;
}

/// Types for the `kv` argument.
/// KVs for Option<&[(&str, &Value)]>
impl<'a> KVs<'a> for &'a [(&'a str, &'a Value<'a>)] {
    #[inline]
    fn into_kvs(self) -> Option<&'a [(&'a str, &'a Value<'a>)]> {
        Some(self)
    }
}

/// KVs for ()
impl<'a> KVs<'a> for () {
    #[inline]
    fn into_kvs(self) -> Option<&'a [(&'a str, &'a Value<'a>)]> {
        None
    }
}
