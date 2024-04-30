use core_utils::random::*;

#[test]
fn test_random_string() {
    let s = random_string(32);
    assert_eq!(s.len(), 32);
}

#[test]
fn test_random_string_2() {
    let s = random_string_2(32);
    assert_eq!(s.len(), 32);
}

#[test]
fn test_generate_iv() {
    let s = generate_iv();
    assert_eq!(s.len(), 16);
}

#[test]
fn test_generate_iv_2() {
    let s = generate_iv_2();
    assert_eq!(s.len(), 16);
}

#[test]
fn test_get_random_key16() {
    let s = get_random_key16();
    assert_eq!(s.len(), 16);
}

#[test]
fn test_get_random_key32() {
    let s = get_random_key32();
    assert_eq!(s.len(), 32);
}

#[test]
fn test_gen() {
    let mut gen = Gen::new(5);
    let x = gen.gen_range(100..1000);
    println!("{}", x);

    let y: bool = gen.gen();
    println!("{}", y);

    let z: u32 = gen.gen();
    println!("{}", z);
}
