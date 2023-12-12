use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<SinglyLinkedListNode<T>>,
    tail: Option<SinglyLinkedListNode<T>>,
}

#[derive(Debug)]
pub struct SinglyLinkedListNode<T> {
    reference: Rc<RefCell<Box<SinglyLinkedListNodeData<T>>>>,
}

#[derive(Debug)]
pub struct SinglyLinkedListNodeData<T> {
    data: T,
    next: Option<SinglyLinkedListNode<T>>,
}

impl<T: Debug> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        return SinglyLinkedList {
            head: Option::None,
            tail: Option::None,
        };
    }

    pub fn insert_head(&mut self, data: T) {
        let node = SinglyLinkedListNode::new(data);
        if self.head.is_none() {
            self.head = Option::Some(node.clone());
            self.tail = Option::Some(node);
            return;
        }
        let head = self.head.as_ref().unwrap().clone();
        node.reference.borrow_mut().next = Option::Some(head);
        self.head = Option::Some(node);
    }

    pub fn insert_tail(&mut self, data: T) {
        let node = SinglyLinkedListNode::new(data);
        if self.head.is_none() {
            self.head = Option::Some(node.clone());
            self.tail = Option::Some(node);
            return;
        }
        self.tail.as_ref().unwrap().reference.borrow_mut().next = Option::Some(node.clone());
        self.tail = Option::Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return Option::None;
        }
        let head = &self.head.as_ref().unwrap().reference;
        let tail = &self.tail.as_ref().unwrap().reference;
        if Rc::ptr_eq(head, tail) {
            let head = self.head.take().unwrap();
            let _ = self.tail.take();
            return Option::Some(Rc::try_unwrap(head.reference).unwrap().into_inner().data);
        }
        let next = self.head.as_ref().unwrap().reference.borrow_mut().next.take();
        let head = std::mem::replace(&mut self.head, next).unwrap();
        return Option::Some(Rc::try_unwrap(head.reference).unwrap().into_inner().data);
    }

    pub fn is_empty(&self) -> bool {
        return self.head.is_none();
    }

    pub fn get_head(&self) -> Option<SinglyLinkedListNode<T>> {
        return self.head.as_ref().map(|n| n.clone());
    }
}

impl<T> SinglyLinkedListNode<T> {
    fn new(data: T) -> SinglyLinkedListNode<T> {
        return SinglyLinkedListNode {
            reference: Rc::new(RefCell::new(Box::new(SinglyLinkedListNodeData {
                data,
                next: Option::None,
            }))),
        };
    }

    pub fn set_data(&self, data: T) {
        self.reference.borrow_mut().data = data;
    }

    pub fn next(&self) -> Option<SinglyLinkedListNode<T>> {
        return self.reference.borrow().next.clone();
    }
}

impl<T: Copy> SinglyLinkedListNode<T> {
    pub fn get_data(&self) -> T {
        return self.reference.borrow().data;
    }
}

impl <T> Clone for SinglyLinkedListNode<T> {
    fn clone(&self) -> Self {
        return Self { reference: self.reference.clone() };
    }
}