use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct LRUCache {
    capacity: usize,
    // key -> nodes 裡的 index
    cache: HashMap<i32, usize>,
    // 實際儲存節點
    nodes: Vec<Node>,
    // Most Recently Used
    head: Option<usize>,
    // Leat Recently Used
    tail: Option<usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;

        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let index = match self.cache.get(&key) {
            Some(&index) => index,
            None => return -1,
        };
        let value = self.nodes[index].value;
        // get 也算使用，因此移到最後面
        self.move_to_front(index);

        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        // 情況1 : key 已經存在
        if let Some(&index) = self.cache.get(&key) {
            self.nodes[index].value = value;
            self.move_to_front(index);
            return;
        }
        // 情況2 : cache 已滿，重複利用 tail 節點
        if self.cache.len() == self.capacity {
            let index = self.tail.unwrap();
            let old_key = self.nodes[index].key;

            self.cache.remove(&old_key);
            self.detach(index);

            self.nodes[index].key = key;
            self.nodes[index].value = value;

            self.attach_front(index);
            self.cache.insert(key, index);
        } else {
            // 情況3 : cache 還有空間，建立新節點
            let index = self.nodes.len();

            self.nodes.push(Node {
                key,
                value,
                prev: None,
                next: None,
            });

            self.attach_front(index);
            self.cache.insert(key, index);
        }
    }

    // 將節點從目前位置移除
    fn detach(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        match prev {
            Some(prev_index) => {
                self.nodes[prev_index].next = next;
            }
            None => {
                // 沒有 prev，代表這個節點原本是head
                self.head = next;
            }
        }

        match next {
            Some(next_index) => {
                self.nodes[next_index].prev = prev;
            }
            None => {
                // 沒有 next，代表這個節點原本是tail
                self.tail = prev;
            }
        }
    }

    // 將節點插入 linked list 最前面
    fn attach_front(&mut self, index: usize) {
        self.nodes[index].prev = None;
        self.nodes[index].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(index);
        } else {
            // 原本是空的 linked list
            self.tail = Some(index);
        }

        self.head = Some(index);
    }

    // 將以存在的節點標記為最近使用
    fn move_to_front(&mut self, index: usize) {
        if self.head == Some(index) {
            return;
        }

        self.detach(index);
        self.attach_front(index);
    }
}
