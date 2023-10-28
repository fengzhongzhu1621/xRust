use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
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



#[test]
fn test_rc_borrow() {
    #[derive(Debug)]
    struct StudentNo {
        no: String,
        class: Rc<RefCell<SchoolClass>>, // 学生所在的班级
    }

    #[derive(Debug)]
    struct Student {
        no: StudentNo,                   // 学生编号
        name: String,                    // 学生名称
        age: u8,                         // 学生年纪
        class: Rc<RefCell<SchoolClass>>, // 学生所在的班级
    }

    impl fmt::Display for Student {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.pad(self.name.as_str())
        }
    }

    impl Borrow<StudentNo> for Student {
        fn borrow(&self) -> &StudentNo {
            &self.no
        }
    }

    #[derive(Debug)]
    struct SchoolClass {
        students: HashMap<String, Rc<Student>>,
        name: String,
    }

    impl SchoolClass {
        fn new(name: String) -> Rc<RefCell<SchoolClass>> {
            Rc::new(RefCell::new(SchoolClass {
                name: name,
                students: HashMap::new(),
            }))
        }

        /// 添加学生到班级
        fn add_student(&mut self, no: String, student: Rc<Student>) {
            self.students.insert(no, student);
        }

        /// 根据学生名称获得学生对象
        fn fetch_student(&self, student_name: String) -> Option<&Rc<Student>> {
            self.students.get(&student_name)
        }
    }

    impl fmt::Display for SchoolClass {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.pad(self.name.as_str())
        }
    }

    // 创建一个班级对象
    let class_name = "First class";
    let class = SchoolClass::new(class_name.to_string());

    // 创建一个学生对象
    let student_name = "bob";
    let no = "A001";
    let student = Student {
        no: StudentNo{no: no.to_string(), class: Rc::clone(&class)},
        name: student_name.to_string(),
        age: 18,
        class: Rc::clone(&class),
    };

    // 添加学生到班级中
    {
        class
            .borrow_mut()
            .add_student(student_name.to_string(), Rc::new(student));
    }

    // 根据学生名称查询学生
    // Note: 在使用了 std::borrow::Borrow的情况下，注意不能用 class.borrow(), 因为与 RefCell 的 borrow()冲突,所以使用try_borrow()替代
    let class_a= class.try_borrow().unwrap();
    let student_bob = class_a.fetch_student(student_name.to_string()).unwrap();
    assert_eq!(student_bob.name, student_name.to_string());


    // 使用 Borrow trait 获得学生的学号
    let student= student_bob.as_ref();
    let student_no: &StudentNo = student.borrow(); // 必须显示标注类型，否则会与默认的 Borrow Trait 冲突
    assert_eq!(student_no.no, no.to_string());
    
}
