use std::borrow::Cow;

// The type Cow is a smart pointer providing clone-on-write functionality:
// it can enclose and provide immutable access to borrowed data,
// and clone the data lazily when mutation or ownership is required.
pub fn cow_example() {
    let origin = "hello world";
    // 转化为写时clone智能指针
    let mut cow = Cow::from(origin);
    assert_eq!(cow, "hello world");

    // Cow can be borrowed as a str
    // Cow<'a str> -> &str
    let s: &str = &cow;
    assert_eq!(s, "hello world");

    // Cow can be borrowed as a mut str
    // cow<'a str> -> &mut str
    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();
    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");

    // Cow can be cloned
    // clone 如果是Borrowd，不会真的拷贝数据
    let cow2 = cow.clone();
    assert_eq!(cow2, "HELLO WORLD");
    assert_eq!(origin, "hello world");

    // Cow can be converted to a String
    // Cow<'a str> -> String
    let s: String = cow.into();
    assert_eq!(s, "HELLO WORLD");
}

pub fn cow_example2() {
    fn abs_all(input: &mut Cow<'_, [i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // 如果尚未拥有，则克隆到 vector 中。
                input.to_mut()[i] = -v;
            }
        }
        println!("input is {:?}", input);
    }

    // 因为 `input` 不需要可变的，所以不会发生克隆。
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("slice is {:?}", slice);

    // 发生克隆是因为需要对 `input` 进行可变的。
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("slice is {:?}", slice);

    // 因为 `input` 已被拥有，所以不会发生克隆。
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(&slice);
    abs_all(&mut input);
    println!("slice is {:?}", slice);
}
