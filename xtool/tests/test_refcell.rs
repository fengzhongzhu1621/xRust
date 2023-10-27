use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_new() {
    use std::cell::RefCell;
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

    let employee = RefCell::new(Employee {
        name: String::from("user_a"),
    });
    {
        // borrow_mut获得RefCell内部数据的可变引用
        let mut r: std::cell::RefMut<'_, Employee> = employee.borrow_mut(); 
        r.change_name(String::from("user_b"));
    }
    assert_eq!(*employee.borrow(), Employee {
        name: String::from("user_b"),
    }); 
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

    #[derive(Debug)]
    struct Company {
        name: String,
        manager: Rc<RefCell<Employee>>,
    }

    // 一个员工可能归属于多个公司
    let employee = Rc::new(RefCell::new(Employee {
        name: String::from("user_a"),
    }));

    let company1 = Company {
        name: String::from("company_a"),
        manager: employee.clone(),
    };
    let company2 = Company {
        name: String::from("company_b"),
        manager: employee.clone(),
    };

    employee.borrow_mut().change_name(String::from("user_b"));

    assert_eq!(company1.manager.borrow().name, String::from("user_b"));
    assert_eq!(company2.manager.borrow().name, String::from("user_b"));
}
