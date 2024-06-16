use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i64,
    ptr: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, ptr: None }))
    }
}

struct Queue {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl Queue {
    fn new() -> Self {
        let node = Node::new(0);
        Self {
            head: Rc::clone(&node),
            tail: node,
        }
    }

    fn enqueue(&mut self, value: i64) {
        let new_node = Node::new(value);
        
        // Update the tail pointer to the new node, and adjust other pointers as needed.
        self.tail.borrow_mut().ptr = Some(Rc::clone(&new_node));
        self.tail = new_node;
    }
}

fn main() {

}
