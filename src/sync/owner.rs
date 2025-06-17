use coco::Stack;

///
/// Contains single value
/// - Thread safe
/// - Beheaves like `Option`
pub struct Owner<T> {
    val: Stack<T>
}
//
//
impl<T> Owner<T> {
    ///
    /// Returns [Owner] new instance, containing specified `value`
    pub fn new(value: T) -> Self {
        let val = Stack::new();
        val.push(value);
        Self {
            val,
        }
    }
    ///
    /// Returns [Owner] new instance, containing `None`
    pub fn empty() -> Self {
        Self {
            val: Stack::new(),
        }
    }
    ///
    /// Returns contained `value`
    pub fn take(&self) -> Option<T> {
        self.val.pop()
    }
    ///
    /// Replaces contained `value` with specified one
    pub fn replace(&self, value: T) {
        self.val.pop();
        self.val.push(value);
    }
    ///
    /// Returns `true` if no contained value
    pub fn is_empty(&self) -> bool {
        self.val.is_empty()
    }
}
unsafe impl<T: Send> Send for Owner<T> {}
unsafe impl<T: Send> Sync for Owner<T> {}
