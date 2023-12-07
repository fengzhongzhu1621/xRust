use todo_list::*;

#[test]
fn test_load() {
    // 新增一个待办事项
    let mut task_list = TodoTaskList::new();
    let content = "hello world".to_string();
    task_list.add(&content);

    // 保存待办
    let store = &mut TodoStore::new("todo_list.txt");
    store.save(&task_list);

    // 读取保存的待办
    let task_list = store.load();
    task_list.print_all();

    // 计算待办数量
    assert_eq!(task_list.count(), 1);
}
