use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

struct StudentNo(String);

impl ToOwned for StudentNo {
    type Owned = Student;

    fn to_owned(&self) -> Student {
        let student = STUDENTS.get(&self.0).unwrap();
        Student {
            no: StudentNo(self.0.clone()),
            name: student.name.clone(),
            age: student.age,
        }
    }
}

impl<'a> From<&'a StudentNo> for Cow<'a, StudentNo> {
    #[inline]
    fn from(s: &'a StudentNo) -> Cow<'a, StudentNo> {
        Cow::Borrowed(s)
    }
}

struct Student {
    no: StudentNo,
    name: String,
    age: u8,
}

impl Borrow<StudentNo> for Student {
    fn borrow(&self) -> &StudentNo {
        &self.no
    }
}

lazy_static! {
    static ref STUDENT_NO: String = "A0001".to_string();
    static ref STUDENTS: HashMap<String, Student> = {
        let mut m = HashMap::new();
        m.insert(
            STUDENT_NO.clone(),
            Student {
                no: StudentNo(STUDENT_NO.clone()),
                name: "bob".to_string(),
                age: 18,
            },
        );
        m
    };
}

#[test]
fn test_into_owned() {
    let student_no = StudentNo(STUDENT_NO.clone());

    // 复制一个 Student 类型
    let cow_1 = Cow::Borrowed(&student_no);
    let student_1 = cow_1.into_owned();
    assert_eq!(student_1.name, "bob");

    // Student 类型作为参数传入,返回结果就是输入参数
    // B 类型是 StudentNo
    // <B as ToOwned>::Owned 类型是 Student
    let cow_2: Cow<'_, StudentNo> = Cow::Owned(student_1);
    let student_2 = cow_2.into_owned();
    assert_eq!(student_2.name, "bob");
}

#[test]
fn test_borrowed_to_mut() {
    let student_no = StudentNo(STUDENT_NO.clone());

    // 修改所有者的值,需要复制 Student 对象
    let mut cow_1 = Cow::Borrowed(&student_no);
    assert_eq!(cow_1.0, "A0001");
    let student_1 = cow_1.to_mut();
    student_1.name = "sam".to_string();
    let student_2 = cow_1.into_owned();
    assert_eq!(student_2.name, "sam");
}

#[test]
fn test_owned_to_mut() {
    // 复制一个 Student 类型
    let student_no = StudentNo(STUDENT_NO.clone());
    let cow_1 = Cow::Borrowed(&student_no);
    let student_1 = cow_1.into_owned();
    assert_eq!(student_1.name, "bob");

    // 修改所有者的值,不需要复制 Student 对象
    // Student 类型作为参数传入,返回结果就是输入参数的可变引用
    let mut cow_2: Cow<'_, StudentNo> = Cow::Owned(student_1);
    let student_3 = cow_2.to_mut();
    student_3.name = "frank".to_string();
    // 不需要复制
    let student_4 = cow_2.into_owned();
    assert_eq!(student_4.name, "frank");
}

#[test]
fn test_from() {
    let student_no = StudentNo(STUDENT_NO.clone());
    let cow: Cow<'_, StudentNo> = Cow::from(&student_no);
    let student = cow.into_owned();
    assert_eq!(student.name, "bob");
}

#[test]
fn test_deref() {}
