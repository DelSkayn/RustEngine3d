use std::sync::atomic::{
    AtomicPtr,
    AtomicIsize,
};

struct EventNode<T>{
    next: AtomicPtr<EventNode<T>>,
    done: AtomicIsize,
    value: Vec<T>,
}

impl<T> EventNode<T>{
    fn new(value: Vec<T>,refs: isize) -> EventNode{
        EventNode{
            next: AtomicPtr::new(0),
            done: AtomicIsize(refs),
            value: value,
        }
    }
}

pub struct EventRef<'a,T>{
    eq: &'a EventQueue<T>,
}

pub struct EventQueue<T>{
    refs: u8,
    first: AtomicPtr<,
}
