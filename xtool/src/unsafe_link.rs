use std::ptr;


struct Node <T> {
    elem: T, // 节点值
    next: Link<T>,   // 下一个节点，在堆上分配
}

// link是一个可变指针
type Link<T> = *mut Node<T>;

pub struct UnSafeLink<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> UnSafeLink<T> {
    pub fn new() -> Self {
        // 使用空指针初始化
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    // 入栈
    pub fn push(&mut self, elem: T) {
        unsafe {
            // 创建一个新的尾部节点
            let new_node = Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            });
            // 转换为指针类型
            let new_node_ptr = Box::into_raw(new_node);

            if !self.tail.is_null() {
                (*self.tail).next = new_node_ptr;
            } else {
                self.head = new_node_ptr;
            }

            self.tail = new_node_ptr;
        }
    }

    // 出栈
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    
    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

}

pub struct IntoIter<T>(UnSafeLink<T>); 
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, 
} 
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>, 
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
} 

impl<T> UnSafeLink<T> {

    pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

    pub fn iter(&self) -> Iter<'_, T> {
            unsafe {
                Iter {
                    next: self.head.as_ref(),
                }
            }
        }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            unsafe {
                IterMut {
                    next: self.head.as_mut(),
                }
            }
    }
}


impl<T> Drop for UnSafeLink<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}






