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
    pub fn add(&mut self, content: &str) {
        // 以天分组并
        let key = datetime::format_date();
        // 在vec中插入一条待办
        let created_at = datetime::format_datetime();
        let updated_at = created_at.clone(); // 不能将created_at直接复制给updated_at，借用会导致created_at失效
        let tasks = &mut self.tasks;
        let task_chunks = tasks.entry(key).or_insert(Vec::new());
        task_chunks.push(TodoTask {
            content: content.to_string(),
            is_finish: false,
            created_at: created_at,
            updated_at: updated_at,
        });
    }
}

