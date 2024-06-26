use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
struct StudentNo(String);

impl ToOwned for StudentNo {
    type Owned = Student;

    fn to_owned(&self) -> Student {
        // 根据学号查询学生
        let student = STUDENTS.get(&self.0).unwrap();
        // 返回学生对象
        Student {
            no: StudentNo(self.0.clone()),
            name: student.name.clone(),
            age: student.age,
        }
    }
}

/// 使用 From 将 &StudentNo -> Cow<'a, StudentNo>
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

// 学号唯一表示一名学生
impl Borrow<StudentNo> for Student {
    fn borrow(&self) -> &StudentNo {
        &self.no
    }
}

/// 使用 From 将 Student -> Cow<'a, StudentNo>
impl<'a> From<Student> for Cow<'a, StudentNo> {
    #[inline]
    fn from(s: Student) -> Cow<'a, StudentNo> {
        let cow: Cow<'_, StudentNo> = Cow::Owned(s);
        cow
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
    // 获得学生的学号
    let student_no = StudentNo(STUDENT_NO.clone());

    // 转换为Cow<'a StuduentNo>类型，使用into_owned() 复制一个 Student 类型
    let cow_1 = Cow::Borrowed(&student_no);
    let student_1 = cow_1.into_owned();
    assert_eq!(student_1.name, "bob");

    // Student 类型作为参数传入,返回结果就是输入参数
    // B 类型是 StudentNo
    // <B as ToOwned>::Owned 类型是 Student
    let cow_2: Cow<'_, StudentNo> = Cow::Owned(student_1);
    let student_2 = cow_2.into_owned(); // 没有复制， student_1 和 student_2是同一个对象
    assert_eq!(student_2.name, "bob");
}

#[test]
fn test_to_owned() {
    let student_no = StudentNo(STUDENT_NO.clone());

    // 复制一个 Student 类型
    let cow_1 = Cow::Borrowed(&student_no);
    let student_1 = cow_1.to_owned();
    let student_2 = student_1.into_owned();
    assert_eq!(student_2.name, "bob");

    // Student 类型作为参数传入,返回结果就是输入参数
    // B 类型是 StudentNo
    // <B as ToOwned>::Owned 类型是 Student
    let cow_2: Cow<'_, StudentNo> = Cow::Owned(student_2);
    let student_3 = cow_2.to_owned();
    let student_4 = student_3.into_owned(); // student_3 和 student_4 是同一个对象
    assert_eq!(student_4.name, "bob");
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

    let cow: Cow<'_, StudentNo> = Cow::from(student);
    let student = cow.into_owned();
    assert_eq!(student.name, "bob");
}

/// 测试解引用
/// ```
/// impl<B: ?Sized + ToOwned> const Deref for Cow<'_, B>
/// where
/// B::Owned: ~const Borrow<B>,
/// {
/// type Target = B;
///
/// fn deref(&self) -> &B {
///     match *self {
///         Borrowed(borrowed) => borrowed,
///          Owned(ref owned) => owned.borrow(),
///    }
/// }
/// }
/// ```
///
#[test]
fn test_deref() {
    let student_no = StudentNo(STUDENT_NO.clone());
    let cow_1 = Cow::Borrowed(&student_no);
    assert_eq!(cow_1.0, "A0001");

    let student_1 = cow_1.into_owned();
    let cow_2: Cow<'_, StudentNo> = Cow::Owned(student_1);
    assert_eq!(cow_2.0, "A0001");
}

#[test]
fn test_as_ref() {
    let student_no = StudentNo(STUDENT_NO.clone());
    let cow_1 = Cow::Borrowed(&student_no);
    assert_eq!(cow_1.as_ref().0, "A0001"); // 转换为 &StudentNo 类型

    let student_1 = cow_1.into_owned();
    let cow_2: Cow<'_, StudentNo> = Cow::Owned(student_1);
    assert_eq!(cow_2.as_ref().0, "A0001");
}

#[test]
fn test_borrowed() {
    let s = "Hello world!";

    // 输入参数是 s 的不可变引用, s 必须实现 ToOwned
    let _x = Cow::Borrowed(s);
}

#[test]
fn test_borrowed_into_owned() {
    let s = "Hello world!";
    let cow = Cow::Borrowed(s);
    // into_owned() 执行 borrowed.to_owned()，将&str 复制为 String，通常会执行克隆操作
    // borrowed： <str as ToOwned>::Owned，即 Borrow<str> 类型
    let t = cow.into_owned();
    assert_eq!(t, s);

    let t = "Hello world!".to_string();
    let cow = Cow::Borrowed(&t);
    let _x = cow.into_owned();
}

#[test]
fn test_string_from() {
    fn abs_all(input: &mut Cow<[i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // Clones into a vector if not already owned.
                input.to_mut()[i] = -v;
            }
        }
    }

    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
}
