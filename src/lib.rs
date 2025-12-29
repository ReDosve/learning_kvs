pub struct KvStore;

impl KvStore {
    pub fn new() -> Self {
        KvStore
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!()
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        panic!()
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
