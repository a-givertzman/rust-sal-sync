use std::sync::atomic::{AtomicUsize, Ordering};

///
/// A lock-free stack
/// 
/// This is an implementation of the Treiber stack, one of the simplest lock-free data structures.
/// 
/// It can be used with multiple producers and multiple consumers at the same time.
pub struct Stack<T> {
    val: coco::Stack<T>,
    len: AtomicUsize,
}
//
//
impl<T> Stack<T> {
    ///
    /// Returns [Stack] new instance, containing specified `value`
    pub fn new() -> Self {
        let val = coco::Stack::new();
        Self {
            val,
            len: AtomicUsize::new(0),
        }
    }
    ///
    /// Returns [Stack] new instance, containing specified `value`
    pub fn from(values: Vec<T>) -> Self {
        let len = values.len();
        let val = coco::Stack::new();
        for value in values {
            val.push(value);
        }
        Self {
            val,
            len: AtomicUsize::new(len),
        }
    }
    ///
    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.val.is_empty()
    }
    ///
    /// Returns the number of elements in the stack
    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }
    ///
    /// Returns the number of elements in the stack
    pub fn pop(&self) -> Option<T> {
        match self.val.pop() {
            Some(value) => {
                self.len.fetch_sub(1, Ordering::SeqCst);
                Some(value)
            }
            None => None,
        }
    }
    ///
    /// Returns `true` if no contained value
    pub fn push(&self, value: T) {
        self.val.push(value)
    }
}
unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}
