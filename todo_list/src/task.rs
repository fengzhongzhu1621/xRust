use crate::datetime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// todo任务类
#[derive(Serialize, Deserialize)]
pub struct TodoTask {
    content: String,    // 内容，非引用类型
    is_finish: bool,    // 任务是否完成
    created_at: String, // 任务创建时间，非引用类型
    updated_at: String, // 任务最新更新时间，非引用类型
    index: String,      // 任务索引
}

#[derive(Serialize, Deserialize)]
pub struct TodoTaskList {
    // 任务按天汇总
    tasks: HashMap<String, Vec<TodoTask>>,
}

impl TodoTaskList {
    /// 初始化任务待办列表
    pub fn new() -> Self {
        TodoTaskList {
            tasks: HashMap::new(),
        }
    }

    /// 添加一条待办
    pub fn add(&mut self, content: &String) {
        // 以天分组并
        let key = datetime::format_date();
        // 在vec中插入一条待办
        let created_at = datetime::format_datetime();
        // 不能将created_at直接复制给updated_at，借用会导致created_at失效
        let updated_at = created_at.clone();
        // 转换为可变引用
        let tasks = &mut self.tasks;
        let task_chunks = tasks.entry(key).or_insert(Vec::new());
        task_chunks.push(TodoTask {
            content: content.to_string(),
            is_finish: false,
            created_at: created_at,
            updated_at: updated_at,
            index: task_chunks.len().to_string(),
        });
    }

    // 切换任务状态
    pub fn toggle(&mut self, key: &String, index: &String) {
        // 根据时间查找某一天的所有任务
        let task_chunks = self.tasks.get_mut(key).unwrap();
        // 根据任务自定义索引查找一天中的任务所在的数组索引
        let task_index = task_chunks.iter().position(|x| x.index == *index).unwrap();
        // 根据数组索引获得任务的可变引用
        let task_item = task_chunks.get_mut(task_index).unwrap();
        // 修改数组的元素
        *task_item = TodoTask {
            content: task_item.content.to_string(),
            is_finish: !task_item.is_finish,
            created_at: task_item.created_at.to_string(),
            updated_at: datetime::format_datetime(),
            index: task_item.index.to_string(),
        };
    }

    pub fn remove(&mut self, key: &String, index: &String) {
        // 根据时间查找某一天的所有任务
        let task_chunks = self.tasks.get_mut(key).unwrap();
        // 根据任务自定义索引查找一天中的任务所在的数组索引
        let target = task_chunks.iter().position(|x| x.index == *index);
        match target {
            Some(i) => {
                task_chunks.remove(i);
            }
            _ => println!("没有可删除的任务"),
        }
    }

    pub fn print(&self, key: &String) {
        println!("---------------------");
        println!("{}\n", key);
        // 根据时间查找某一天的所有任务
        match self.tasks.get(key) {
            Some(task_chunks) => {
                if task_chunks.len() == 0 {
                    println!("没有待办事项");
                    return;
                }
                for task in task_chunks {
                    let state = if task.is_finish {
                        "已完成"
                    } else {
                        "未完成"
                    };
                    println!("{}: {} {}", task.index, state, task.content)
                }
                println!();
            }
            _ => println!("没有待办事项"),
        }
    }

    pub fn print_all(&self) {
        for key in self.tasks.keys() {
            self.print(key)
        }
    }

    pub fn count(&self) -> usize {
        let mut sum = 0;
        for tasks in self.tasks.values() {
            sum += tasks.len();
        }

        sum
    }
}
