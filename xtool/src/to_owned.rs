use std::borrow::Borrow;
use std::collections::HashMap;

/// 学生编号类，生命周期小于班级的生命周期
struct StudentNo {
    no: String,
    class: SchoolClass,
}


/// 根据学生编号获得学生实例
// impl ToOwned for StudentNo {

//     type Owned = Student;

//     fn to_owned(&self) -> Self::Owned {
//         Student{name: StudentNo(self.0.clone()), age: 18}
//     }
// }


/// 学生类，学生的生命周期要小于班级的生命周期
struct Student {
    no: String, // 学生编号
    name:  String, // 学生名称
    age: u8, // 学生年纪
    class: SchoolClass, // 学生所在的班级
}

/// 获得学生的唯一编号
// impl Borrow<StudentNo> for Student {
//     fn borrow(&self) -> &'a StudentNo {
//         &StudentNo{
//             no: self.no.clone(),
//             class: &self.class,
//         }
//     }
// }

/// 班级类
struct SchoolClass {
    students: HashMap<String, Student>, // 班级内的所有学生
    name: String, // 班级名称
}

impl SchoolClass {
    fn new(name: String) -> Self {
        Self {
            name: name,
            students: HashMap::new(),
        }
    }

    // fn add_student(mut self, no: String, student: Student) {
    //     self.students.insert(no, student);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_borrow() {
        let s = "hello";
        let _: String = s.to_owned();
    }

    #[test]
    fn test_owned() {
        // 给班级添加学生
        // let class_name = "class name".to_string();
        // let mut class = SchoolClass::new(class_name);
        // let student_no = "AAA";

        // let student = Student {
        //     no: student_no.to_string(),
        //     name: "bob".to_string(),
        //     age: 18,
        //     class: &class,
        // };
        // class.add_student(student_no.to_string(), student)


    }
}