use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkedList<T> {
    pub right: Option<Rc<RefCell<LinkedList<T>>>>,
    pub left: Option<Rc<RefCell<LinkedList<T>>>>,
    value: T,
}

impl<T> LinkedList<T> {
    pub fn get_value(&self) -> &T {
        &self.value
    }
}
