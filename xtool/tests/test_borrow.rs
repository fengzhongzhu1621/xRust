use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::borrow::Borrow;

#[test]
fn test_rc_borrow() {
    #[derive(Debug)]
    struct StudentNo {
        no: String,
        class: Rc<RefCell<SchoolClass>>, // 学生所在的班级
    }

    #[derive(Debug)]
    struct Student {
        no: Rc<StudentNo>,               // 学生编号
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
    let no = "A001";
    let student_no = Rc::new(StudentNo{no: no.to_string(), class: Rc::clone(&class)});
    let student = Student {
        no: student_no,
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
}
