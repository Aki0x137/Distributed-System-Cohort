use std::{borrow::Borrow, ptr, sync::{atomic::{AtomicPtr, Ordering}, Arc, Mutex}};
use std::cell::RefCell;

struct Node {
    value: i64,
    next: AtomicPtr<Node>,
}

impl Node {
    fn new(value: i64) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(Self { value, next: AtomicPtr::new(ptr::null_mut()) }))
    }
}

struct Queue {
    head: Arc<AtomicPtr<Node>>,
    tail: Arc<AtomicPtr<Node>>,
}

impl Queue {
    fn new() -> Self {
        let node: Arc<RefCell<Node>> = Node::new(0);
        Self {
            head: Arc::new(AtomicPtr::new(node.as_ptr())),
            tail: Arc::new(AtomicPtr::new(node.as_ptr())),
        }
    }

    fn enqueue(&mut self, value: i64) {
        let new_node = Node::new(value);
        let mut tail;

        loop {
            let tail = self.tail.load(Ordering::SeqCst);
            let next = (*tail).next.load(Ordering::SeqCst);

            if tail == self.tail.load(Ordering::SeqCst) {
                if next.is_null() {
                    if (*tail).next.compare_exchange(next, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        self.tail.compare_exchange(tail, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst);
                        break;
                    }
                } else {
                    self.tail.compare_exchange(tail, next, Ordering::SeqCst, Ordering::SeqCst);
                }
            }
        }
        
        self.tail.compare_exchange(tail, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst);
    }

    fn dequeue(&mut self) -> Option<i64> {
        loop {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            let next = (*head).next.load(Ordering::SeqCst);

            if head == self.head.load(Ordering::SeqCst) {
                if head == tail {
                    if next.is_null() {
                        return None;
                    }
                    self.tail.compare_exchange(tail, next, Ordering::SeqCst, Ordering::SeqCst).ok();
                } else {
                    if self.head.compare_exchange(head, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        return Some((*next).value);
                    }
                        
                }
            } 
        }
    }
}

fn main() {

}
