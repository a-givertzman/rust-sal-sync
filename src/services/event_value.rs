/// ### Thread-safe abstraction for consuming aggregated event states.
/// 
/// Maintains an in-memory snapshot of the latest values mapped by event keys.
/// Designed for low-latency, concurrent reads by analytical or calculation algorithms.
pub trait EventValueAccess<K: ?Sized, V>
where
    K: ToOwned,
    <K as ToOwned>::Owned: std::hash::Hash + Eq {
    /// ### Registers an event key for subsequent stream subscription.
    /// 
    /// Must be called during the initialization phase before the services starts.
    fn subscribe(&self, key: &K);
    /// ### Performs a lock-free lookup for the most recent value of the specified `key`.
    /// 
    /// Returns `None` if the event key is unregistered or no data has been received yet.
    fn get(&self, key: &K) -> Option<V>;
}