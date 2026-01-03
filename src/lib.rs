//! Simple KVStore

#![deny(missing_docs)]
use std::collections::HashMap;

/// A simple KVStore based on memory
/// 'KVStore' use hashmap to sotre key-value pair
///
/// # example
///
/// ```
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
///
/// ```
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// assert_eq!(store.get("nonexistent".to_string()), None);
    /// ```
    pub fn new() -> Self {
        let store = KvStore {
            data: HashMap::new(),
        };
        store
    }

    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// assert_eq!(store.set("key1".to_string(), "value1".to_string()), ());
    ///
    /// assert_eq!(store.set("key1".to_string(), "new_value".to_string()),());
    ///            
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    ///
    /// assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
    /// assert_eq!(store.get("nonexistent".to_string()), None);
    /// ```
    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    ///
    /// assert_eq!(store.remove("nonexistent".to_string()), ());
    /// ```
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
