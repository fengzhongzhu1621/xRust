#[macro_export]
macro_rules! hello {
    // $name 是一个表达式
    ($name: expr) => {
        // 生成的代码如下
        println!("Hello, {}", $name);
    };

    // 没有任何参数
    () => {
        println!("Hello, world!");
    };
}
