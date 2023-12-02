use std::io;

fn not_supported<T>() -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "operation not supported on this platform",
    ))
}
