// Box<T>, casually referred to as a ‘box’, provides the simplest form of heap allocation in Rust.
// Boxes provide ownership for this allocation, and drop their contents when they go out of scope.
// Boxes also ensure that they never allocate more than isize::MAX bytes.
// 
// Box<T> 在堆上分配空间，用于存储 不适合栈或需要超出当前作用域的数据
//
// - 使用场景
// * 堆分配：对于大型数据结构或需要在不复制数据的情况下传递数据特别有用。避免栈溢出。
// * 所有权或安全性：保证堆分配的内存在不再需要时被正确清理。
// * 传递数据不需要克隆：在不需要克隆的情况下将数据传递给函数或跨线程，避免不必要的复制来提高性能。
// * 动态大小类型（dst）：可以存储在编译时大小未知的类型，例如使用 Box<dyn Trait>可以使用 trait object，支持动态分派和多态。
// * 递归数据结构：递归类型的大小（链表或树）不能在编译时确认，可以通过将递归元素包装在堆分配的 Box 中解决此问题。
//
// - 注意：
// 过度使用 Box<T>可能导致内存碎片化和堆空间的低效使用，确保仅在堆分配合理时才使用 Box<T>。对于大多数涉及小类型或非递归数据结构的时候，默认的栈分配更有效。
// 为拥有 Box<T> 的类型手动实现 Drop trait时，请确保正确处理清理以避免内存泄漏。

pub fn box_example() {
    // 创建一个堆指针
    let b = Box::new(5);
    println!("b = {}", b);
}

pub fn box_example2() {
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
