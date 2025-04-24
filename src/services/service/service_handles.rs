use std::{sync::atomic::{AtomicUsize, Ordering}, thread::JoinHandle};
use coco::Stack;
use indexmap::{map::IntoIter, IndexMap};
use sal_core::error::Error;

///
/// Holds the collaction of the Services's id & JoinHandle pairs
pub struct ServiceHandles<T> {
    // id: String,
    len: AtomicUsize,
    handles: Stack<(String, JoinHandle<T>)>
}
//
// 
impl<T> ServiceHandles<T> {
    ///
    /// Creates new collaction of the JoinHandle  
    pub fn new(handles: Vec<(String, JoinHandle<T>)>) -> Self {
        let len = AtomicUsize::new(handles.len());
        let hs = Stack::new();
        for h in handles {
            hs.push(h);
        }
        Self {
            len,
            handles: hs,
        }
    }
    ///
    /// Returns the nimber of the holding handles
    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }
    ///
    /// inserts new Services's id & JoinHandle 
    /// - if already have such id, current handle will be updated
    pub fn insert(&self, id: impl Into<String>, handle: JoinHandle<T>) {
        self.handles.push((id.into(), handle));
        self.len.fetch_add(1, Ordering::SeqCst);
    }
    ///
    /// Checks if the [Service] has finished running.
    /// 
    /// To finish the [Service] call exit
    pub fn is_finished(&self) -> bool {
        self.len.load(Ordering::SeqCst) == 0
    }
    ///
    /// Returns  to wait for [Service] will be finished
    pub fn wait(&self) -> Result<(), Error> {
        let mut errors = vec![];
        let mut handles = vec![];
        while !self.handles.is_empty() {
            if let Some((k, v)) = self.handles.pop() {
                handles.push((k, v));
            }
        }
        for (id, h) in handles {
            if let Err(err) = h.join() {
                errors.push(format!("Join thread '{id}' error: {:?}", err));
            }
        }
        self.len.store(0, Ordering::SeqCst);
        if !errors.is_empty() {
            return Err(Error::new("ServiceHandles", "wait").pass(errors.join("\n")));
        }
        Ok(())
    }
}
//
// 
impl<T> IntoIterator for ServiceHandles<T> {
    type Item = (String, JoinHandle<T>);

    type IntoIter = IntoIter<String, JoinHandle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut handles = IndexMap::new();
        while !self.handles.is_empty() {
            if let Some((k, v)) = self.handles.pop() {
                handles.insert(k, v);
            }
        }
        self.len.store(0, Ordering::SeqCst);
        handles.reverse();
        handles.into_iter()
    }
}
