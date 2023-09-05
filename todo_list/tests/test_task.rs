mod test_todo_task_list {
    use todo_list::datetime::format_date;
    use todo_list::task::TodoTaskList;

    #[test]
    fn test_add() {
        let mut task_list = TodoTaskList::new();
        let content = "hello world";
        let key = format_date();
        task_list.add(content);
        task_list.print(&key);
    }

    #[test]
    fn test_toggle() {
        let mut task_list = TodoTaskList::new();
        let content = "hello world";
        let key = format_date();
        let index = "0".to_string();
        task_list.add(content);

        task_list.toggle(&key, &index);
        task_list.print(&key);

        task_list.toggle(&key, &index);
        task_list.print(&key);

        task_list.remove(&key, &index);
        task_list.print(&key);

        task_list.print("2022-01-01");
    }
}
