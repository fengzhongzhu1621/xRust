use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Into 不提供 From 实现（ From 却提供 Into）。因此，应该始终尝试先实现 From，如果 From 无法实现，则再尝试使用 Into。
struct Wrapper<T>(Vec<T>);

// Wrapper -> Vec<T>
impl<T> Into<Vec<T>> for Wrapper<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}


// i32 -> Number
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}


#[test]
fn test_from() {
    let num = Number::from(99);
    assert_eq!(num.value , 99);
}

#[test]
fn test_into() {
    let value = 99;
    let num: Number = value.into();
    assert_eq!(num.value , 99);
}

#[test]
fn test_into_trait() {
    let v: Vec<i32> = vec![1,2,3];
    let wrapper = Wrapper(v);
    let actual: Vec<i32> = wrapper.into();
    assert_eq!(actual, vec![1,2,3]);
}

#[test]
fn test_string_into() {
    let s = "hello".to_string();
    let v: Vec<u8> = s.into();
    assert_eq!(v, vec![104, 101, 108, 108, 111]);
}