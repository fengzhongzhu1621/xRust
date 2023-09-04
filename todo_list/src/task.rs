use crate::datetime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// todo任务类
#[derive(Serialize, Deserialize)]
pub struct TodoTask {
    content: String,    // 内容
    is_finish: bool,    // 任务是否完成
    created_at: String, // 任务创建时间
    updated_at: String, // 任务最新更新时间
}

#[derive(Serialize, Deserialize)]
pub struct TodoTaskList {
    // 任务按天汇总
    tasks: HashMap<String, Vec<TodoTask>>,
}

impl TodoTaskList {
    pub fn new() -> Self {
        TodoTaskList {
            tasks: HashMap::new(),
        }
    }

    pub fn add(&mut self, content: &str) {
        let tasks = &mut self.tasks;
        let key = datetime::format_date();
        let task_chunks = tasks.entry(key).or_insert(Vec::new());
        let created_at = datetime::format_datetime();
        let updated_at = created_at.clone();
        task_chunks.push(TodoTask {
            content: content.to_string(),
            is_finish: false,
            created_at: created_at,
            updated_at: updated_at,
        });
    }
}
