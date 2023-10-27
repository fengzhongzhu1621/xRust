use core::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[test]
fn test_new() {
    let c = RefCell::new(5);
    assert_eq!(*c.borrow(), 5);
}

/// 获得RefCell内部数据的不可变引用
#[test]
fn test_borrow() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    assert_eq!(*s1, "hello, world");
}

/// 获得RefCell内部数据的可变引用
#[test]
fn test_borrow_mut() {
    // 不可变引用r1和可变引用r2在同一时刻同时存在，不符合借用规则，虽然可以编译通过，但在运行时会panic。
    // let mut v = RefCell::new(5);
    // let r1 = v.borrow();
    // let mut r2 = v.borrow_mut();
    // assert_eq!(*r1, 5);
    // assert_eq!(*r2, 5);

    let s = RefCell::new(String::from("hello, world"));
    let _s2 = s.borrow_mut();

    let s = Rc::new(RefCell::new("hello".to_string()));
    let s2 = s.clone();
    s2.borrow_mut().push_str(", world");

    assert_eq!(*s.borrow(), "hello, world");
}

/// 测试 RefCell 内部可变对象执行方法
#[test]
fn test_borrow_mut_method() {
    #[derive(Debug, PartialEq)]
    struct Employee {
        name: String,
    }

    impl Employee {
        pub fn change_name(&mut self, name: String) {
            self.name = name;
        }
    }

    let employee = RefCell::new(Employee { name: String::from("user_a") });
    {
        // borrow_mut获得RefCell内部数据的可变引用
        let mut r: std::cell::RefMut<'_, Employee> = employee.borrow_mut();
        r.change_name(String::from("user_b"));
    }
    assert_eq!(*employee.borrow(), Employee { name: String::from("user_b") });
}

#[test]
fn test_rc_borrow_mut_method() {
    #[derive(Debug)]
    struct Employee {
        name: String,
    }

    impl Employee {
        pub fn change_name(&mut self, name: String) {
            self.name = name;
        }
    }

    impl fmt::Display for Employee {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.pad(self.name.as_str())
        }
    }

    #[derive(Debug)]
    struct Company {
        name: String,
        manager: Rc<RefCell<Employee>>,
    }

    // 一个员工可能归属于多个公司
    let employee =
        Rc::new(RefCell::new(Employee { name: String::from("user_a") }));
    println!("{}", employee.borrow());

    let company1 =
        Company { name: String::from("company_a"), manager: employee.clone() };
    let company2 =
        Company { name: String::from("company_b"), manager: employee.clone() };

    employee.borrow_mut().change_name(String::from("user_b"));

    assert_eq!(company1.manager.borrow().name, String::from("user_b"));
    assert_eq!(company2.manager.borrow().name, String::from("user_b"));
}

#[test]
fn test_rc_borrow_1() {
    #[derive(Debug)]
    struct Student {
        name: String,                    // 学生名称
        age: u8,                         // 学生年纪
        class: Rc<RefCell<SchoolClass>>, // 学生所在的班级
    }

    impl fmt::Display for Student {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.pad(self.name.as_str())
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
    let student = Student {
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
    let class_a = class.borrow();
    let student_bob = class_a.fetch_student(student_name.to_string()).unwrap();
    assert_eq!(student_bob.name, student_name.to_string());
}
