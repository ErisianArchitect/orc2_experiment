#![allow(unsafe_op_in_unsafe_fn)]
use std::sync::atomic::{AtomicPtr, Ordering};


#[repr(C)]
#[derive(Debug)]
pub(crate) struct LockfreeLinkedListNode<T> {
    pub _value: T,
    pub handle: AtomicPtr<AtomicPtr<Self>>,
    pub next: AtomicPtr<Self>,
}

impl<T> LockfreeLinkedListNode<T> {
    #[must_use]
    #[inline]
    pub fn new(value: T, handle: *mut AtomicPtr<Self>) -> Box<Self> {
        Box::new(Self {
            _value: value,
            handle: AtomicPtr::new(handle),
            next: AtomicPtr::new(std::ptr::null_mut()),
        })
    }
    
    #[must_use]
    #[inline]
    pub fn handle_ptr(&self) -> Option<&AtomicPtr<Self>> {
        unsafe { self.handle.load(Ordering::Acquire).as_ref() }
    }
    
    #[must_use]
    #[inline]
    pub fn next_ptr_ptr(&self) -> &AtomicPtr<Self> {
        &self.next
    }
}

#[derive(Debug)]
pub(crate) struct LockfreeLinkedList<T> {
    pub head: Box<AtomicPtr<LockfreeLinkedListNode<T>>>,
}

impl<T> LockfreeLinkedList<T> {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            head: Box::new(AtomicPtr::new(std::ptr::null_mut())),
        }
    }
    
    pub fn get_head(&self) -> &AtomicPtr<LockfreeLinkedListNode<T>> {
        &*self.head
    }
    
    pub unsafe fn push(
        &self,
        next: &AtomicPtr<LockfreeLinkedListNode<T>>,
        node: *const LockfreeLinkedListNode<T>,
    ) {
        let mut head = self.head.load(Ordering::Acquire);
        loop {
            next.store(head, Ordering::Relaxed);
            head = match self.head.compare_exchange_weak(head, node.cast_mut(), Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => break,
                Err(new_head) => new_head,
            };
        }
    }
}

impl<T> Drop for LockfreeLinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.load(Ordering::Relaxed);
        loop {
            if head.is_null() {
                return;
            }
            let head_box = unsafe { Box::from_raw(head) };
            head = head_box.next.load(Ordering::Relaxed);
        }
    }
}