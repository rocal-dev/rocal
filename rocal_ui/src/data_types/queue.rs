use std::cell::RefCell;
use std::rc::Rc;

pub struct Queue<T>
where
    T: Clone,
{
    start: Option<Rc<RefCell<LinkedList<T>>>>,
    end: Option<Rc<RefCell<LinkedList<T>>>>,
    pub len: u64,
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            len: 0,
        }
    }

    pub fn enqueue(&mut self, node: T) {
        let node = Rc::new(RefCell::new(LinkedList::new(node)));

        if self.end.is_some() {
            self.end.clone().unwrap().borrow_mut().right = Some(node.clone());
            node.clone().borrow_mut().left = self.end.clone();
        } else {
            self.start = Some(node.clone());
        }

        self.end = Some(node.clone());
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let mut node: Option<Rc<RefCell<LinkedList<T>>>> = None;

        if self.len > 0 {
            if let Some(start) = self.start.clone() {
                node = Some(start);
                self.start = node.clone().unwrap().borrow().right.clone();
                node.clone().unwrap().borrow_mut().left = None;
                node.clone().unwrap().borrow_mut().right = None;
                self.len -= 1;
            }
        } else {
            self.start = None;
            self.end = None;
        }

        match node {
            Some(node) => Some(node.borrow().get_value().to_owned()),
            None => None,
        }
    }

    pub fn peek(&self) -> Option<T> {
        if let Some(end) = self.end.clone() {
            Some(end.borrow().get_value().to_owned())
        } else {
            None
        }
    }
}

struct LinkedList<T>
where
    T: Clone,
{
    right: Option<Rc<RefCell<LinkedList<T>>>>,
    left: Option<Rc<RefCell<LinkedList<T>>>>,
    value: T,
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self {
            right: None,
            left: None,
            value,
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}
