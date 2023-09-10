struct Node<T> {
    elem: T,       // 节点值
    next: Link<T>, // 下一个节点，在堆上分配
}

// 编译时Rust需要知道一个类型所占用的空间大小，是一个枚举类型
type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    /// 初始化栈
    pub fn new() -> Self {
        Self { head: None }
    }

    /// 压栈
    pub fn push(&mut self, elem: T) {
        // 新节点的下一个节点是head节点
        let next = self.head.take();
        let new_node = Box::new(Node {
            elem: elem,
            next: next,
        });

        // 修改head节点
        self.head = Some(new_node);
    }

    /// 出栈
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            // head指向下一个节点
            self.head = node.next;
            // 返回头节点的值
            node.elem
        })
    }

    /// 返回元素的不可变引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // 返回元素的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

/// 实现栈的into_iter()迭代器，所有权会转移

pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 实现 iter() 迭代器
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // 返回引用类型
            &node.elem
        })
    }
}

// 实现 iter_mut() 迭代器
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Stack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for Stack<T> {
    /// 离开作用域时调用，清理box在栈上分配的资源
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
