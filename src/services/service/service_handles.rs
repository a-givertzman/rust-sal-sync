use std::thread::JoinHandle;
use indexmap::{map::IntoIter, IndexMap};

///
/// Holds the collaction of the Services's id & JoinHandle pairs
pub struct ServiceHandles<T> {
    // id: String,
    handles: IndexMap<String, JoinHandle<T>>
}
//
// 
impl<T> ServiceHandles<T> {
    ///
    /// Creates new collaction of the JoinHandle  
    pub fn new(handles: Vec<(String, JoinHandle<T>)>) -> Self {
        Self {
            handles: handles.into_iter().collect()
        }
    }
    ///
    /// Returns the nimber of the holding handles
    pub fn len(&self) -> usize {
        self.handles.len()
    }
    ///
    /// inserts new Services's id & JoinHandle 
    /// - if already have such id, current handle will be updated
    pub fn insert(&mut self, id: impl Into<String>, handle: JoinHandle<T>) {
        self.handles.insert(id.into(), handle);
    }
    // ///
    // /// 
    // pub fn first(&mut self) -> (String, JoinHandle<()>) {
    //     let key = self.handles.keys().next().cloned();
    //     match key {
    //         Some(key) => {
    //             match self.handles.remove_entry(&key) {
    //                 Some(handle) => handle,
    //                 None => {
    //                     panic!("ServiceHandle.first | Handle '{}' - not found", key)
    //                 }
    //             }
    //         }
    //         None => {
    //             panic!("ServiceHandle.first | Handles not found")
    //         }
    //     }
    // }
}
//
// 
impl<T> IntoIterator for ServiceHandles<T> {
    type Item = (String, JoinHandle<T>);

    type IntoIter = IntoIter<String, JoinHandle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        // todo!()
        self.handles.into_iter()
    }
}