use std::any::Any;
use std::rc::Rc;

#[test]
fn test_print_rc() {
    let mut num = Rc::new("hello".to_string());
    let num2 = num.clone();
    println!("num {} num2 {}", num, num2);
}

#[test]
fn test_strong_count() {
    // 创建 Rc 对象的引用计数值为 1
    let rc_examples = "Rc examples".to_string();
    let rc_a: Rc<String> = Rc::new(rc_examples);
    assert_eq!(Rc::strong_count(&rc_a), 1);

    // 添加引用计数
    let rc_b: Rc<String> = Rc::clone(&rc_a);
    assert_eq!(rc_a, rc_b);
    assert!(rc_a.eq(&rc_b));
    assert_eq!(Rc::strong_count(&rc_a), 2);
    assert_eq!(Rc::strong_count(&rc_b), 2);
}

#[test]
fn tet_clone() {
    let rc_examples = "Rc examples".to_string();
    let rc_a: Rc<String> = Rc::new(rc_examples);

    let rc_b: Rc<String> = rc_a.clone();
    assert_eq!(Rc::strong_count(&rc_a), 2);
    assert_eq!(Rc::strong_count(&rc_a), Rc::strong_count(&rc_b));
}

#[test]
fn test_drop() {
    let rc_examples = "Rc examples".to_string();
    let rc_a: Rc<String> = Rc::new(rc_examples);

    let rc_b: Rc<String> = rc_a.clone();

    drop(rc_a);
    assert_eq!(Rc::strong_count(&rc_b), 1);
}

/// 获得 Rc 内部的值的属性
#[test]
fn get_rc_innter_item_attribute() {
    // Rc<T> 自动取消对 T 的引用 (通过 Deref trait)，因此您可以在 Rc<T> 类型的值上调用 T 的方法。
    // 为了避免与 T 方法的名称冲突，Rc<T> 本身的方法是关联函数，使用 fully qualified syntax 进行调用

    struct Owner {
        name: String,
        // ... 其他领域
    }

    impl Owner {
        fn print_address(&self) {
            println!("name is {:p}", &self);
        }
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
        // ... 其他领域
    }

    // 创建一个引用计数的 `Owner`。
    let gadget_owner: Rc<Owner> =
        Rc::new(Owner { name: "Gadget Man".to_string() });

    // 创建属于 `gadget_owner` 的 `Gadget`。
    // 克隆 `Rc<Owner>` 为我们提供了指向同一个 `Owner` 分配的新指针，从而增加了该进程中的引用计数。
    //
    let gadget1 = Gadget { id: 1, owner: Rc::clone(&gadget_owner) };

    // 获取 Rc 内部对象的属性
    assert_eq!(gadget1.owner.name, "Gadget Man".to_string());

    // 调用 Rc 内部对象的方法
    gadget1.owner.print_address()
}

#[test]
fn test_pin() {
    // pub fn pin(value: T) -> Pin<Rc<T>>
    // 构建一个新的 Pin<Rc<T>>。如果 T 没有实现 Unpin，那么 value 将会固定在内存中不可移动。
}

/// pub fn try_unwrap(this: Self) -> Result<T, Self>
/// 如果 Rc 有且只有1个强引用，则返回包含的值，否则返回 Err<T>。
/// 不管 Rc 有多少弱引用，只要符合上述条件，该函数都将成功。
#[test]
fn test_try_unwrap() {
    // 只有一个强引用
    let x = Rc::new(3);
    assert_eq!(Rc::try_unwrap(x), Ok(3));

    let x = Rc::new(4);
    assert_eq!(Rc::try_unwrap(x).unwrap(), 4);

    // 有多个强引用,返回 Err
    let x = Rc::new(5);
    let _y = Rc::clone(&x);
    assert_eq!(*Rc::try_unwrap(x).unwrap_err(), 5); // Rc::try_unwrap(x) 返回 Err(4)
}

/// pub fn into_raw(this: Self) -> *const T
/// 消费 Rc, 返回被包装的指针。
/// 为了避免内存泄漏，被包装的指针如果要被重新转换为 Rc, 应该使用 Rc::from_raw
#[test]
fn test_into_raw() {
    let x = Rc::new(4);
    let x_ptr = Rc::into_raw(x); // x_ptr 为裸指针
    assert_eq!(unsafe { *x_ptr }, 4);
}

/// pub unsafe fn from_raw(ptr: *const T) -> Self
/// 从裸指针中构建一个 Rc。
/// 裸指针必须是从 Rc::into_raw 中返回的裸指针。
/// 这个函数是不安全的，因为不正确使用可能会导致内存问题。例如，在裸指针上二次释放资源。
#[test]
fn test_from_raw() {
    let x = Rc::new(10);
    let x_ptr = Rc::into_raw(x); // x_ptr 为裸指针

    unsafe {
        // 必须使用 from_raw 转换成 Rc 避免内存泄漏
        let x = Rc::from_raw(x_ptr);
        assert_eq!(*x, 10);

        // 再次调用 `Rc::from_row(x_ptr)` 会导致内存不安全
    }

    // `x` 的内存将会在离开作用域后释放，所以 `x_ptr` 不是悬吊指针
}

/// 判断两个指针是否指向同一个值
#[test]
fn test_ptr_eq() {
    let five = Rc::new(5);
    let same_five = Rc::clone(&five);
    let other_five = Rc::new(5);

    assert!(Rc::ptr_eq(&five, &same_five));
    assert!(!Rc::ptr_eq(&five, &other_five));
}

/// 创建一个 Rc 的可变引用。如果 Rc 还有其他引用或弱引用，make_mut 将会克隆内部值以保证所有权的唯一性。这也被称为写时克隆。
/// pub fn make_mut(this: &mut Self) -> &mut T
#[test]
fn test_make_mut() {
    let mut data = Rc::new(5);

    // 只有一个强引用不会克隆
    assert_eq!(Rc::strong_count(&data), 1);
    *Rc::make_mut(&mut data) += 1;
    assert_eq!(Rc::strong_count(&data), 1);

    // 有多个引用会克隆数据
    let mut other_data = Rc::clone(&data); //此时还未复制
    assert_eq!(Rc::strong_count(&data), 2);
    assert_eq!(Rc::strong_count(&other_data), 2);
    *Rc::make_mut(&mut data) += 1; // 复制内部数据，复制后引用计数重置
    assert_eq!(Rc::strong_count(&data), 1);
    assert_eq!(Rc::strong_count(&other_data), 1);

    // 复制后再次调用原指针将不会触发克隆
    *Rc::make_mut(&mut data) += 1;
    *Rc::make_mut(&mut other_data) *= 2;
    assert_eq!(Rc::strong_count(&data), 1);
    assert_eq!(Rc::strong_count(&other_data), 1);

    // 现在 `data` 和 `other_data` 指向不同值
    assert_eq!(*data, 8);
    assert_eq!(*other_data, 12);
}

/// 如果没有其他 Rc 或者 Weak 指向內部值，則返回內部值的可变引用，否則返回 None，因为改变共享值是不安全的。
#[test]
fn test_get_mut() {
    let mut x = Rc::new(3);
    *Rc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);

    let _y = Rc::clone(&x);
    assert!(Rc::get_mut(&mut x).is_none());
}

/// 尝试将 Rc<dyn Any> 降级为具体值
/// pub fn downcast<T: Any>(self) -> Result<Rc<T>, Rc<dyn Any>>
#[test]
fn test_downcast() {
    fn print_if_string(value: Rc<dyn Any>) {
        if let Ok(string) = value.downcast::<String>() {
            println!("String ({}): {}", string.len(), string);
        }
    }

    let my_string = "Hello World".to_string();
    print_if_string(Rc::new(my_string));
    print_if_string(Rc::new(0i8)); // 不会打印
}

#[test]
fn test_downgrade() {
    // pub fn downgrade(this: &Self) -> Weak<T>

    let rc_examples = "Rc examples";
    let rc_a: Rc<String> = Rc::new(rc_examples.to_string());

    let rc_b: Rc<String> = rc_a.clone();

    // 创建一个被包裹值的弱引用指针
    // 但是如果已经丢弃了分配中存储的值，则它将返回 None。
    // 换句话说，Weak 指针不会使分配内部的值保持活动状态。但是，它们确实使分配 (内部值的后备存储) 保持活动状态。
    let _weak = Rc::downgrade(&rc_a);

    assert_eq!(Rc::strong_count(&rc_a), 2);
    assert_eq!(Rc::strong_count(&rc_b), 2);
    assert_eq!(Rc::weak_count(&rc_a), 1);
    assert_eq!(Rc::weak_count(&rc_b), 1);
}

#[test]
fn test_weak_upgrade() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);

    // Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 值为 None
    let _: Option<Rc<_>> = weak_five.upgrade();
}
