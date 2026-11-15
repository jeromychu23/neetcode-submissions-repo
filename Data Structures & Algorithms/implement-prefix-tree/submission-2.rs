struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
}
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end_of_word: false,
        }
    }
}

struct PrefixTree {
    root: TrieNode,
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(TrieNode::new())
        } 
        cur.end_of_word = true;        
    }

    pub fn search(&self, word: String) -> bool {
        let mut cur = &self.root;

        for c in word.chars() {
            if let Some(next) = cur.children.get(&c) {
                cur = next;
            } else {
                return false;
            }
        }
        cur.end_of_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;

        for c in prefix.chars() {
            if let Some(next) = cur.children.get(&c) {
                cur = next;
            } else {
                return false;
            }
        }
        true
    }
}
