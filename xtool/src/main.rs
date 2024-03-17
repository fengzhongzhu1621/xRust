use xtool::*;

fn largest(list: &[i32]) -> i32 {
    let mut largest_value = list[0];
    for &item in list {
        if item > largest_value {
            largest_value = item;
        }
    }

    return largest_value;
}

fn main() {
    println!("Hello, world!");

    let number_list = [2, 1, 3];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![2, 1, 3];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // 测试 Cow<T>
    cow_example();
    cow_example2();

    // 测试Box<T>
    box_example();
    box_example2();

    // 测试 Cell<T> RefCell<T>
    cell_example();
    refcell_example();

    // 测试懒加载
    once_cell_example();

    // 测试 Rc<T>
    rc_example();
}
