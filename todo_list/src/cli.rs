use clap::Parser;

#[derive(Parser)]
#[command(name="TodoList")]
struct cli {
    #[arg(long)]
    action: String,

    #[arg(long)]
    
}
fn from_args() {}
