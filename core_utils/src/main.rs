use core_utils::convert::*;

fn main() {
    println!("Hello, world!");
    let s = Some("ok");
    let _ = ok_or(s, "error");
}
