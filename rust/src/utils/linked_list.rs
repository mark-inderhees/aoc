use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug, Default, Clone)]
pub struct LinkedList<T> {
    head: Weak<RefCell<Node<T>>>,
    current: Weak<RefCell<Node<T>>>,
    pub values: Vec<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Default, Clone)]
pub struct Node<T> {
    pub value: T,
    next: Weak<RefCell<Node<T>>>,
    prev: Weak<RefCell<Node<T>>>,
}

impl<T> Node<T>
where
    T: Default + Clone,
{
    fn new(value: &T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value.clone(),
            ..Default::default()
        }))
    }
}

impl<T> LinkedList<T>
where
    T: Default + Clone + Debug,
{
    /// Create a new linked list based on a list of values. This also stores
    /// each node in a list so the data is kept alive.
    pub fn new(values: &Vec<T>) -> LinkedList<T> {
        let mut linked_list = LinkedList {
            ..Default::default()
        };

        // Init nodes into vector
        for value in values {
            linked_list.values.push(Node::new(value));
        }

        // Create links
        for i in 0..values.len() {
            let mut current = linked_list.values[i].deref().borrow_mut();
            let next = &linked_list.values[(i + 1) % values.len()];
            let prev = &linked_list.values[i.checked_sub(1).unwrap_or(values.len() - 1)];

            current.next = Rc::downgrade(next);
            current.prev = Rc::downgrade(prev);
        }

        // Set current as head
        linked_list.current = Rc::downgrade(&linked_list.values[0]);
        linked_list.head = linked_list.current.clone();

        linked_list
    }

    pub fn set_current(&mut self, node: &Weak<RefCell<Node<T>>>) {
        self.current = node.clone();
    }

    pub fn get_current(&self) -> Weak<RefCell<Node<T>>> {
        self.current.clone()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get_current_value(&self) -> T {
        self.current
            .upgrade()
            .unwrap()
            .deref()
            .borrow()
            .value
            .clone()
    }

    pub fn move_next(&mut self) {
        self.current = self
            .current
            .upgrade()
            .unwrap()
            .deref()
            .borrow()
            .next
            .clone();
    }

    pub fn move_prev(&mut self) {
        self.current = self
            .current
            .upgrade()
            .unwrap()
            .deref()
            .borrow()
            .prev
            .clone();
    }

    pub fn pop(&mut self) -> Weak<RefCell<Node<T>>> {
        let popped = self.current.clone();

        let current_rc = self.current.upgrade().unwrap();
        let current = current_rc.deref().borrow_mut();
        let prev = current.prev.clone();
        let next = current.next.clone();
        prev.upgrade().unwrap().deref().borrow_mut().next = next.clone();
        next.upgrade().unwrap().deref().borrow_mut().prev = prev;

        self.current = next;

        popped
    }

    pub fn insert(&mut self, node: &Weak<RefCell<Node<T>>>) {
        let prev = self.current.clone();
        let next = self
            .current
            .upgrade()
            .unwrap()
            .deref()
            .borrow()
            .next
            .clone();
        next.upgrade().unwrap().deref().borrow_mut().prev = node.clone();
        prev.upgrade().unwrap().deref().borrow_mut().next = node.clone();

        node.upgrade().unwrap().deref().borrow_mut().next = next;
        node.upgrade().unwrap().deref().borrow_mut().prev = prev;
        self.current = node.clone();
    }

    pub fn print(&self) {
        let mut values = vec![];
        let mut node = self.head.clone();
        for _ in 0..self.values.len() {
            values.push(node.upgrade().unwrap().deref().borrow().value.clone());
            node = node.upgrade().unwrap().deref().borrow().next.clone();
        }
        log::debug!("{:?}", values);
    }
}
