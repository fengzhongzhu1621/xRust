use clap::Parser;
use todo_list::*;

#[derive(Parser)]
#[command(name = "TodoList")]
struct Cli {
    #[arg(long)]
    action: String,
}

fn main() {
    let cli = Cli::parse();
    let action = cli.action;
    let task_list = TodoTaskList::new();

    if action == "print" {
        let key = format_date();
        task_list.print(&key);
    } else if action == "add" {
    } else if action == "toggle" {
    } else if action == "remove" {
    }
}
