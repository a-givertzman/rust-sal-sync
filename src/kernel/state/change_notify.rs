use std::sync::Arc;

use arc_swap::ArcSwap;
use crate::collections::FxHashMap;
///
/// Provides callback on connection status changes
pub struct ChangeNotify<'a, S, T> {
    id: String,
    state: ArcSwap<S>,
    cases: Arc<FxHashMap<S, Box<dyn Fn(T) + Send + Sync + 'a>>>,
}
//
//
impl<'a, S, T> ChangeNotify<'a, S, T> 
where
    S: Clone + std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + std::fmt::Debug {
    ///
    /// Returns [ChangeNotify] new instance
    pub fn new(parent: impl Into<String>, initial: S, cases: Vec<(S, Box<dyn Fn(T) + Send + Sync + 'a>)>) -> Self {
        let cases = Arc::new(FxHashMap::from_iter(cases));
        Self {
            id: format!("{}/ChangeNotify<{}>", parent.into(), std::any::type_name::<S>()),
            state: ArcSwap::new(Arc::new(initial)),
            cases,
        }
    }
    ///
    /// Returns [ChangeNotifyBuilder] new instance
    pub fn builder(parent: impl Into<String>, initial: S) -> ChangeNotifyBuilder<S, T> {
        ChangeNotifyBuilder {
            id: format!("{}/ChangeNotify<{}>", parent.into(), std::any::type_name::<S>()),
            initial,
            cases: FxHashMap::default(),
        }
    }
    ///
    /// Update state and arguments
    pub fn add(&self, state: S, args: T) {
        let self_state_guard = self.state.load();
        if state != **self_state_guard {
            let prev_guard = self.state.compare_and_swap(&self_state_guard, Arc::new(state.clone()));
            // Если то, что вернул CAS, совпадает с тем, что мы загрузили в начале
            // - замена успешна (именно данный вызов обновил стейт)
            // - вызываем калбэк
            if Arc::ptr_eq(&self_state_guard, &prev_guard) {
                match self.cases.get(&state) {
                    Some(callback) => {
                        (callback)(args)
                    },
                    None => log::error!("{}.add | State `{:?}` is not found", self.id, state),
                }
            }
        }
    }
    ///
    /// Update state lazily. Arguments is evalueted only if state has changed
    pub fn update<F>(&self, s: S, f: F)
    where
        F: FnOnce() -> T, {
        let self_state_guard = self.state.load();
        if s != **self_state_guard {
            let prev_guard = self.state.compare_and_swap(&self_state_guard, Arc::new(s.clone()));
            // Если то, что вернул CAS, совпадает с тем, что мы загрузили в начале
            // - замена успешна (именно данный вызов обновил стейт)
            // - вызываем калбэк
            if Arc::ptr_eq(&self_state_guard, &prev_guard) {
                match self.cases.get(&s) {
                    Some(callback) => {
                        (callback)((f)())
                    },
                    None => log::error!("{}.add | State `{:?}` is not found", self.id, s),
                }
            }
        }
    }
}
///
/// Builder for the ChangeNotify
pub struct ChangeNotifyBuilder<S, T> {
    id: String,
    initial: S,
    cases: FxHashMap<S, Box<dyn Fn(T) + Send + Sync + 'static>>,
}
impl<'a, S, T> ChangeNotifyBuilder<S, T>
where
    S: Clone + std::cmp::Eq + std::hash::Hash + std::fmt::Debug {
    /// 
    /// Добавляем состояние и колбэк для него
    pub fn on(mut self, state: S, case: impl Fn(T) + Send + Sync + 'static) -> Self {
        self.cases.insert(state, Box::new(case));
        self
    }
    ///
    /// Returns ChangeNotify ready to use
    pub fn build(self) -> ChangeNotify<'a, S, T> {
        ChangeNotify {
            id: self.id,
            state: ArcSwap::new(Arc::new(self.initial)),
            cases: Arc::new(self.cases),
        }
    }
}
