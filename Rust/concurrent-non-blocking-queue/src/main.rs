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

#[derive(Clone)]
struct Queue {
    head: Arc<AtomicPtr<Node>>,
    tail: Arc<AtomicPtr<Node>>,
}

impl Queue {
    fn new() -> Self {
        Self {
            head: Arc::new(AtomicPtr::new(Node::new(0).as_ptr())),
            tail: Arc::new(AtomicPtr::new(Node::new(0).as_ptr())),
        }
    }

    fn enqueue(&mut self, value: i64) {
        let new_node = Node::new(value);
        let mut tail;

        loop {
            tail = self.tail.load(Ordering::SeqCst);
            let next = unsafe{ (*tail).next.load(Ordering::SeqCst) };

            if tail == self.tail.load(Ordering::SeqCst) {
                if next.is_null() {
                    if unsafe {
                        (*tail).next.compare_exchange(next, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst).is_ok()
                    }  {
                        let _ = self.tail.compare_exchange(tail, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst);
                        break;
                    }
                } else {
                    let _ = self.tail.compare_exchange(tail, next, Ordering::SeqCst, Ordering::SeqCst);
                }
            }
        }
        
        let _ = self.tail.compare_exchange(tail, new_node.as_ptr(), Ordering::SeqCst, Ordering::SeqCst);
    }

    fn dequeue(&mut self) -> Option<i64> {
        loop {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            let next = unsafe {(*head).next.load(Ordering::SeqCst)};

            if head == self.head.load(Ordering::SeqCst) {
                if head == tail {
                    if next.is_null() {
                        return None;
                    }
                    self.tail.compare_exchange(tail, next, Ordering::SeqCst, Ordering::SeqCst).ok();
                } else {
                    if self.head.compare_exchange(head, unsafe { (*next ).next.load(Ordering::SeqCst) }, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        return unsafe{ Some((*next).value) };
                    }
                        
                }
            } 
        }
    }
}


fn main() {
    let queue = Queue::new();

    let mut handles = vec![];

    for i in 0..10 {
        let queue = Arc::new(Mutex::new(queue.clone()));

        let handle = std::thread::spawn(move || {
            let mut queue = queue.lock().unwrap();
            queue.enqueue(i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles = vec![];

    for _ in 0..10 {
        let queue = Arc::new(Mutex::new(queue.clone()));

        let handle = std::thread::spawn(move || {
            let mut queue = queue.lock().unwrap();
            println!("{:?}", queue.dequeue());
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
