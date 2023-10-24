use std::borrow::Borrow;
use std::collections::HashMap;

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

    fn check<K>(s: K) 
    where K: Borrow<str> {
        let borrowed: &str = s.borrow();
        assert_eq!("Hello", borrowed);
    }

    #[test]
    fn test_borrow_as_param() {
        let s = "Hello".to_string();
        check(s);
        
        let s = "Hello";
        check(s);

        let s = Person{name: "Hello".to_string(), age: 18};
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
}
