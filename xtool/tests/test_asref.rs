use std::fmt;

struct MyStruct {
    pub data: String,
}

/// 实现 MyStruct -> &MyStruct
impl AsRef<MyStruct> for MyStruct {
    fn as_ref(&self) -> &Self {
        self
    }
}

struct MyStruct2 {
    pub id: String,
    pub data: MyStruct,
}

/// 实现 MyStruct2 -> &MyStruct
impl AsRef<MyStruct> for MyStruct2 {
    fn as_ref(&self) -> &MyStruct {
        &self.data
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyStruct: {}", self.data)
    }
}

/// cargo test --package xtool --test test_asref -- test_as_ref --exact --nocapture
#[test]
fn test_as_ref() {
    let mydata = MyStruct { data: "hello".to_string() };

    let mydata2 = MyStruct2 {
        id: "abc".to_string(),
        data: MyStruct { data: "world".to_string() },
    };

    fn use_as_ref(val: impl AsRef<MyStruct>) {
        println!("ref is: {}", val.as_ref());
        println!("data is: {}", val.as_ref().data);
    }

    // ref is: MyStruct: hello
    // data is: hello
    use_as_ref(mydata);

    // ref is: MyStruct: world
    // data is: world
    use_as_ref(mydata2);
}
