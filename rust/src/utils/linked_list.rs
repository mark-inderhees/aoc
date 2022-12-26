use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// A linked list. Is doubly linked, giving prev and next. Keeps a current
/// pointer and can navigate with move next and prev. Can get value of current.
/// Can pop current out of the list. Can insert at current.
#[derive(Debug, Default, Clone)]
pub struct LinkedList<T> {
    /// First node added to the list.
    head: Weak<RefCell<Node<T>>>,

    /// Current node with all the moves done.
    current: Weak<RefCell<Node<T>>>,

    /// A vector of all of the nodes. This keeps the Rc in scope so the memory
    /// is not released.
    pub values: Vec<Rc<RefCell<Node<T>>>>,
}

/// Simple node struct. Stores a value and pointers to prev and next.
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
    /// Create a new node with a given value. Has no prev and next connections.
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
        // Default init, empty current and head pointers
        let mut linked_list = LinkedList {
            ..Default::default()
        };

        // Init nodes into vector so they are long lived
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

    /// Set the current value to a specific node.
    pub fn set_current(&mut self, node: &Weak<RefCell<Node<T>>>) {
        self.current = node.clone();
    }

    /// Get the current node.
    #[allow(dead_code)]
    pub fn get_current(&self) -> Weak<RefCell<Node<T>>> {
        self.current.clone()
    }

    /// Get the length of the original list. This does not change with pop and insert.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Get the value stored in the current node.
    pub fn get_current_value(&self) -> T {
        self.current
            .upgrade()
            .unwrap()
            .deref()
            .borrow()
            .value
            .clone()
    }

    /// Set the value of the current node.
    #[allow(dead_code)]
    pub fn set_current_value(&self, value: &T) {
        self.current.upgrade().unwrap().deref().borrow_mut().value = value.clone();
    }

    /// Move the current to the next node.
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

    /// Move the current to the prev node.
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

    /// Remove the current node form the list and return it. Set current to the next node.
    pub fn pop(&mut self) -> Weak<RefCell<Node<T>>> {
        // Remove the current node
        let popped = self.current.clone();

        // Connect the prev and next together
        let prev = popped.upgrade().unwrap().deref().borrow().prev.clone();
        let next = popped.upgrade().unwrap().deref().borrow().next.clone();
        prev.upgrade().unwrap().deref().borrow_mut().next = next.clone();
        next.upgrade().unwrap().deref().borrow_mut().prev = prev;

        // Update current
        self.current = next;

        popped
    }

    /// Insert this node into the list. The current will point to this new node as the next node.
    /// Then update the current pointer to point to this new node.
    pub fn insert(&mut self, node: &Weak<RefCell<Node<T>>>) {
        // Connect prev and next to this new node
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

        // Connect this new node to prev and next
        node.upgrade().unwrap().deref().borrow_mut().next = next;
        node.upgrade().unwrap().deref().borrow_mut().prev = prev;

        // Update current
        self.current = node.clone();
    }

    /// Print all the values in the linked list
    pub fn print(&self) {
        // Walk the list and build a vector that we can print
        let mut values = vec![];
        let mut node = self.head.clone();
        for _ in 0..self.values.len() {
            values.push(node.upgrade().unwrap().deref().borrow().value.clone());
            node = node.upgrade().unwrap().deref().borrow().next.clone();
        }
        log::debug!("{:?}", values);
    }
}
