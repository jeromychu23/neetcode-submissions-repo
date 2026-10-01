struct MyHashMap {
    map: HashMap<i32, i32>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(val) => *val,
            None => -1,
        }
    }

    pub fn remove(&mut self, key: i32) {
        self.map.remove(&key);
    }
}
