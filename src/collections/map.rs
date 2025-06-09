use std::{collections::HashMap, hash::BuildHasherDefault};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
///
/// HashMap from std::collections with simple & fast hasher
///  - This hashing algorithm should not be used for cryptographic, or in scenarios where DOS attacks are a concern.
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
///
/// IndexMap from std::collections with simple & fast hasher
///  - This hashing algorithm should not be used for cryptographic, or in scenarios where DOS attacks are a concern.
pub type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;
///
/// DashMap with simple & fast hasher
///  - This hashing algorithm should not be used for cryptographic, or in scenarios where DOS attacks are a concern.
pub type FxDashMap<K, V> = dashmap::DashMap<K, V, std::hash::BuildHasherDefault<hashers::fx_hash::FxHasher>>;
