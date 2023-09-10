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
}
