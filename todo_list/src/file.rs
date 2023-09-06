use crate::task::TodoTaskList;
use std::fs::File;
use std::io::{Read, Write};

pub struct TodoStore {
    // 文件路径
    path: String,
}

impl TodoStore {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn save(&mut self, todo_list: &TodoTaskList) {
        // 将待办列表转换为json字符串
        let data = &serde_json::to_string(todo_list).unwrap();
        // 以写方式打开文件，覆盖一个已经存在的文件
        let mut file = File::create(&self.path).unwrap();
        file.write(&data.as_bytes()).unwrap();
    }

    pub fn load(&self) -> TodoTaskList {
        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_err) => {
                panic!("文件不存在！");
            }
        };
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        let data: TodoTaskList = serde_json::from_str(&s).unwrap();

        data
    }
}
