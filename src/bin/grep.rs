use std::env;
use std::process;
use xrust::Config;

fn main() {
    // 获得命令行参数，ascii字符
    let args: Vec<String> = env::args().collect();

    // 参数复制给变量
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = xrust::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
