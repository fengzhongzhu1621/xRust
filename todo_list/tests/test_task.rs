mod test_todo_task_list {
    use todo_list::task::TodoTaskList;

    #[test]
    fn test_add() {
        let mut task_list = TodoTaskList::new();
        let content = "hello world";
        task_list.add(content)
    }
}
