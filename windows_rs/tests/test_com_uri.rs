use windows::core::*;
use windows::Win32::System::Com::*;

/// Creates a new IUri instance, and initializes it from a Uniform Resource Identifier (URI) string.
/// CreateUri also normalizes and validates the URI.
/// STDAPI CreateUri(
///  _In_       LPCWSTR   pwzURI,
///  _In_       DWORD     dwFlags = Uri_CREATE_CANONICALIZE,
///  _Reserved_ DWORD_PTR dwReserved,
///  _Out_      IUri      **ppURI
/// );
///
/// pwzuri LPCWSTR 是一个指向unicode编码字符串的32位指针，所指向字符串是wchar型，而不是char型。
/// typedef const wchar_t* LPCWSTR;
///
/// dwReserved  Reserved. Must be set to 0.
#[test]
fn test_create_uri() -> windows::core::Result<()> {
    unsafe {
        let uri = CreateUri(
            w!("http://kennykerr.ca"),
            URI_CREATE_FLAGS::default(),
            0,
        )?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        assert_eq!(domain, "kennykerr.ca");
        assert_eq!(port, 80);

        Ok(())
    }
}
