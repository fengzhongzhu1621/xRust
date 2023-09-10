use std::env;
use std::process;
use std::thread;
use std::time::Duration;
use xrust::Cacher;
use xrust::Config;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("intensity is {}", expensive_closure.value(intensity));
        println!("intensity is {}", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("random_number is {}", random_number);
        } else {
            println!("intensity is {}", expensive_closure.value(intensity));
        }
    }
}

fn main() {
    // 获得命令行参数，ascii字符
    // let args: Vec<String> = env::args().collect();

    // 参数复制给变量
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 执行业务逻辑
    if let Err(e) = xrust::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn one_result() {
        let query = "one";
        let contents = "\
    one
    two
    three";
        let actual = search(query, contents);
        let expect = vec!["one"];
        assert_eq!(actual, expect);
    }
}
