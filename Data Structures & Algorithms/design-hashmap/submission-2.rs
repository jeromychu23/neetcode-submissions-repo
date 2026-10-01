struct MyHashMap {
    map: Vec<i32>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self {
            map: vec![-1; 1_000_001],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.map[key as usize] = value;
    }

    pub fn get(&self, key: i32) -> i32 {
        self.map[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.map[key as usize] = -1;
    }
}
