use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Stack<T>
where
    T: Clone,
{
    top: Option<Rc<RefCell<LinkedList<T>>>>,
    pub len: u64,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { top: None, len: 0 }
    }

    pub fn push(&mut self, node: T) {
        let node = Rc::new(RefCell::new(LinkedList {
            next: self.top.clone(),
            value: node,
        }));

        self.top = Some(node.clone());
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(top) = self.top.clone() {
            self.top = top.borrow().next.clone();
            self.len -= 1;
            Some(top.borrow().get_value().to_owned())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<T> {
        if let Some(top) = self.top.clone() {
            Some(top.borrow().get_value().to_owned())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LinkedList<T>
where
    T: Clone,
{
    next: Option<Rc<RefCell<LinkedList<T>>>>,
    value: T,
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn get_value(&self) -> &T {
        &self.value
    }
}
