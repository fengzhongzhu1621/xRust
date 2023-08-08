use container_primitive::*;

fn main() {
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
