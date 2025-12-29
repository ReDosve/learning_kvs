use std::collections::HashMap;

pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let store = KvStore {
            data: HashMap::new(),
        };
        store
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
