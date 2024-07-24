

struct Node {
    key: String,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: String, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, key: String, value: String) {
        if key < self.key {
            if let Some(left) = &mut self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        } else if key > self.key {
            if let Some(right) = &mut self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {
            self.value = value;
        }
    }

    fn search(&self, key: &str) -> Option<&str> {
        if key < &self.key {
            self.left.as_ref().and_then(|left| left.search(key))
        } else if key > &self.key {
            self.right.as_ref().and_then(|right| right.search(key))
        } else {
            Some(&self.value)
        }
    }
}

fn main() {
    let mut root = Node::new("apple".to_string(), "fruit".to_string());

    root.insert("banana".to_string(), "yellow fruit".to_string());
    root.insert("cherry".to_string(), "red fruit".to_string());
    root.insert("c".to_string(), "c fruit".to_string());
    root.insert("ch".to_string(), "ch fruit".to_string());
    root.insert("z".to_string(), "z fruit".to_string());
    root.insert("ca".to_string(), "ca fruit".to_string());

    if let Some(value) = root.search(&"ca".to_string()) {
        println!("Value for 'ca': {}", value);
    } else {
        println!("Key not found.");
    }
}
