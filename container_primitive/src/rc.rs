// Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.

use std::rc::Rc;

pub fn rc_example() {
    let rc = Rc::new(1);

    let rc2 = rc.clone();
    let rc3 = Rc::clone(&rc);
    println!("rc_example rc2: {}, rc3:{}", rc2, rc3);

    // 调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。你也可以通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，
    // 类似于 strong_count。其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。
    let my_weak = Rc::downgrade(&rc);
    drop(rc);
    drop(rc2);
    drop(rc3);

    // 因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。
    // 因为 upgrade 返回一个 Option<T>，我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。
    println!("rc_example my_weak: {}", my_weak.upgrade().is_none());
}
