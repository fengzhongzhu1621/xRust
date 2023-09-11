use std::rc::Rc;

struct Node<T> {
    elem: T,       // 节点值
    next: Link<T>, // 下一个节点，在堆上分配
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct ConstStack<T> {
    head: Link<T>,
}

impl<T> ConstStack<T> {
    /// 初始化栈
    pub fn new() -> Self {
        Self { head: None }
    }

    // 压栈，并生成一个新的栈
    pub fn prepend(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    // 出栈，返回下一个节点开头的新的栈
    pub fn tail(&self) -> Self {
        let node = self.head.as_ref();
        Self {
            head: node.and_then(|node| node.next.clone()),
        }
    }

    // 仅返回栈顶元素，不出栈
    pub fn head(&self) -> Option<&T> {
        let node = self.head.as_ref();
        node.map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> ConstStack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for ConstStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}
