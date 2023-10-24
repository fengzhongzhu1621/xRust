use std::borrow::Borrow;

#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

impl Borrow<str> for Person {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_origin_check() {
        fn origin_check(s: &String) {
            assert_eq!("Hello", s);
        }

        let s = "Hello".to_string();
        origin_check(&s);
    }

    #[test]
    fn test_origin_check_1() {
        fn origin_check(s: &String) {
            let borrowed: &str = s.borrow();
            assert_eq!("Hello", borrowed);
        }

        let s = "Hello".to_string();
        origin_check(&s);
    }

    fn check<K>(s: K)
    where
        K: Borrow<str>,
    {
        let borrowed: &str = s.borrow();
        assert_eq!("Hello", borrowed);
    }

    #[test]
    fn test_borrow_as_param() {
        let s = "Hello".to_string();
        check(s);

        let s = "Hello";
        check(s);

        let s = Person { name: "Hello".to_string(), age: 18 };
        check(s);
    }

    #[test]
    fn test_get_hash_value() {
        let mut map = HashMap::new();

        let key = "Foo".to_string();
        let value = 1;

        map.insert(key, value);

        let value = map.get("Foo");

        assert_eq!(value, Some(&1));
    }

    #[test]
    fn test_default_borrow() {
        struct MyType;

        let my_type = MyType;
        let _: &MyType = my_type.borrow();
        let _: &MyType = (&my_type).borrow();

        let my_type = &MyType;
        let _: &MyType = my_type.borrow();
        let _: &MyType = (*my_type).borrow();

        let my_type = &mut MyType;
        let _: &MyType = my_type.borrow();
        let _: &MyType = (*my_type).borrow();
    }

    #[test]
    fn test_string_borrow() {
        let s = "hello".to_string();
        let _: &str = s.borrow();
    }
}
