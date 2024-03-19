pub mod const_stack;
pub mod link;
pub mod linked_list;
pub mod numbers;
pub mod panic;
pub mod proxy;
pub mod stack;
pub mod unsafe_link;

pub mod r#box;
pub mod cell;
pub mod cow;
pub mod rc;

pub use cell::*;
pub use cow::*;
pub use r#box::*;
pub use rc::*;

pub use panic::*;

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,        // 查询参数
    filename: String,     // 文件命名
    case_sensitive: bool, // 是否区分大小写
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let contents = fs::read_to_string(config.filename)?;

    // 根据查询字符串查询文件
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // 打印搜索到的文件行
    for line in results {
        println!("{}", line)
    }

    Ok(())
}

impl Config {
    // 构造函数，返回Result类型
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // let query = args[1].clone();
        // let filename = args[2].clone();
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // 判断环境变了是否设置，如果没有设置is_err() 返回 true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // 成功返回Ok枚举
        Ok(Config { query, filename, case_sensitive })
    }
}

// 搜索字符串，区分大小写
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Cacher<T>
where
    T: Fn(u32) -> u32, // 闭包
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 构造函数，参数是一个闭包
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // 如果已缓冲，则直接返回
            None => {
                // 计算并存储缓冲
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 12, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![Shoe { size: 10, style: String::from("sneaker") },]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
