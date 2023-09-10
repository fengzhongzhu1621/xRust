use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    elem: String, // 节点值
    next: Link,   // 下一个节点，在堆上分配
}

// Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取
// RefCell<T> 允许在运行时执行不可变或可变借用检查。它们只能在单线程中使用。
type Link = Option<Rc<RefCell<Node>>>;

pub struct LinkList {
    head: Link,
    tail: Link,
    length: usize,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            elem: value,
            next: None,
        }))
    }
}

impl LinkList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// 从尾部插入一个节点
    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail_node) => {
                // 修改节点的连线
                old_tail_node.borrow_mut().next = Some(new_node.clone())
            }
            None => {
                // 头尾指向同一个节点
                // 因为tail和head可以指向同一个节点，所以采用Rc<T>
                self.head = Some(new_node.clone())
            }
        }
        self.length += 1;
        self.tail = Some(new_node)
    }

    /// 从头部取出一个节点
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|old_head_node| {
            if let Some(next_node) = old_head_node.borrow_mut().next.take() {
                // 头部指针指向下一个节点
                self.head = Some(next_node);
            } else {
                // 下一个节点为None，说明只一个元素
                self.tail.take();
            }
            self.length -= 1;

            let cell = Rc::try_unwrap(old_head_node)
                .ok()
                .expect("Something is error");

            // 返回RecCell内部数据
            cell.into_inner().elem
        })
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }
}
