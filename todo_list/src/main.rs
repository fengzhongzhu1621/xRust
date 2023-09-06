use clap::{Args, Parser, Subcommand};
use todo_list::*;

#[derive(Parser)]
#[command(name = "TodoList")]
struct Cli {
    #[arg(short, long)]
    print: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add(TodoContent),
    Toggle(ToggleTodo),
    Remove(RemoveTodo),
}

#[derive(Debug, Args)]
struct TodoContent {
    content: String,
}

#[derive(Debug, Args)]
struct ToggleTodo {
    key: String,
    index: String,
}

#[derive(Debug, Args)]
struct RemoveTodo {
    key: String,
    index: String,
}

fn main() {
    let cli = Cli::parse();
    let show = cli.print;

    let filename = "todo_list.txt".to_string();
    let store = TodoStore::new(&filename);
    let task_list = &mut store.load();

    if show {
        task_list.print_all();
        return;
    }

    match &cli.command {
        Some(Commands::Add(command_args)) => {
            // 添加待办
            let content = &command_args.content;
            task_list.add(content);
            // 保存待办
            let store = &mut TodoStore::new(&filename);
            store.save(&task_list);
        }
        Some(Commands::Toggle(command_args)) => {
            let key = &command_args.key;
            let index = &command_args.index;
            // 修改状态
            task_list.toggle(key, index);
            // 保存待办
            let store = &mut TodoStore::new(&filename);
            store.save(&task_list);
        }
        Some(Commands::Remove(command_args)) => {
            let key = &command_args.key;
            let index = &command_args.index;
            // 删除待办
            task_list.remove(key, index);
            // 保存待办
            let store = &mut TodoStore::new(&filename);
            store.save(&task_list);
        }
        None => (),
    }
}
