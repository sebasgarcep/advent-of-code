const NODE_CAPACITY: usize = 256;
const CHILDREN_CAPACITY: usize = 8;

#[derive(Debug)]
pub struct Trie {
    nodes: Vec<TrieNode>,
}

#[derive(Debug)]
pub struct TrieNode {
    char: char,
    value: Option<i64>,
    children: Vec<usize>,
}

#[derive(Debug)]
pub struct TriePointer {
    position: usize,
}

impl Trie {
    pub fn new() -> Trie {
        let root = TrieNode::new(0 as char, Option::None);
        let mut nodes = Vec::with_capacity(NODE_CAPACITY);
        nodes.push(root);
        return Trie { nodes };
    }

    pub fn get_root(&self) -> TriePointer {
        return TriePointer { position: 0 };
    }

    pub fn get_child(&self, pointer: &TriePointer, char: char) -> Option<TriePointer> {
        let current_node = &self.nodes[pointer.position];
        for &pos in current_node.children.iter() {
            let child_node = &self.nodes[pos];
            if child_node.char == char {
                return Option::Some(TriePointer { position: pos });
            }
        }
        return Option::None;
    }

    pub fn get_value(&self, pointer: &TriePointer) -> Option<i64> {
        let node = &self.nodes[pointer.position];
        return node.value;
    }

    pub fn insert(&mut self, word: String, value: i64) {
        let mut current_pointer = self.get_root();
        for char in word.chars() {
            let child_pointer = self.get_child(&current_pointer, char);
            if child_pointer.is_some() {
                current_pointer = child_pointer.unwrap();
                continue;
            }
            let child_position = self.nodes.len();
            let child_node = TrieNode::new(char, Option::None);
            self.nodes.push(child_node);

            self.nodes[current_pointer.position]
                .children
                .push(child_position);
            current_pointer = TriePointer {
                position: child_position,
            };
        }
        self.nodes[current_pointer.position].value = Option::Some(value);
    }
}

impl TrieNode {
    fn new(char: char, value: Option<i64>) -> TrieNode {
        return TrieNode {
            char,
            value,
            children: Vec::with_capacity(CHILDREN_CAPACITY),
        };
    }
}
