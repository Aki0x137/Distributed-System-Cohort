package main

import (
	"errors"
	"fmt"
	"sync"
	"sync/atomic"
	"unsafe"
)

type Node struct {
	value int
	next *Node
}

func NewNode(val int) *Node {
	return &Node{
		value: val,
	}
}

func compareAndSwapNode(p **Node, old, new *Node) bool {
    return atomic.CompareAndSwapPointer((*unsafe.Pointer)(unsafe.Pointer(p)), unsafe.Pointer(old), unsafe.Pointer(new))
}

// func load(node **Node) *Node {
// 	return (*Node)(atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(node))))
// }

type Queue struct {
	Head *Node
	Tail *Node
}

func NewQueue() *Queue {
	node := NewNode(0)
	head, tail := NewNode(0), NewNode(0)
	head.next = node
	tail.next = node

	return &Queue{
		Head: head,
		Tail: tail,
	}
}

func (q *Queue) enqueue(val int) {
	node := NewNode(val)
	var tail *Node

	for {
		tail = q.Tail
		next := tail.next

		if q.Tail == tail {
			if next == nil {
				if ok := compareAndSwapNode(&tail.next, next, node); ok {
					break
				}
			} else {
				compareAndSwapNode(&q.Tail, tail, next)
			}
		}
	}

	compareAndSwapNode(&q.Tail, tail, node)
}

func (q *Queue) dequeue() (int, error) {
	var pvalue int
	for {
		head := q.Head
		tail := q.Tail
		next := head.next

		if head == q.Head {
			if head == tail {
				if next == nil {
					return 0, errors.New("Queue is empty")
				}
				compareAndSwapNode(&q.Tail, tail, next)
			} else {
				pvalue = next.value
				if ok := compareAndSwapNode(&q.Head, head, next); ok {
					break
				}
			}
		}
	}

	return pvalue, nil
}

func main() {
	q := NewQueue()

	var wg sync.WaitGroup

	wg.Add(20)

	for i := 0; i < 10; i++ {
		go func(i int) {
			defer wg.Done()
			q.enqueue(i)
		}(i)
	}

	for i := 0; i < 10; i++ {
		go func() {
			defer wg.Done()
			val, err := q.dequeue()
			if err != nil {
				fmt.Println(err)
			} else {
				fmt.Println(val)
			}
		}()
	}
	
	wg.Wait()
}