use crate::server::node::Node;

struct Database {
    // root is a box node of a tuple of str, str
    root: Option<Box<Node<String>>>,
}

impl Database {
    fn search<'l>(node: &'l mut Node<String>, key: &String) -> &'l mut Node<String> {
        // Returns what would be the parent node if the key is not found
        // Otherwise, returns the node with the given key
        if node.key == *key {
            return node;
        } else if node.key < *key && node.left.is_some() {
            return Database::search(node.left.as_mut().unwrap(), key);
        } else if node.key > *key && node.right.is_some() {
            return Database::search(node.right.as_mut().unwrap(), key);
        } else {
            return node;
        }
    }

    pub fn new() -> Database {
        return Database { root: None };
    }

    pub fn add(&mut self, key: &String, value: &String) -> Result<(), &str> {
        match self.root.as_mut() {
            None => {
                self.root = Some(Node::new(key.clone(), value.clone()));
                return Ok(());
            }
            Some(x) => {
                let node = Database::search(&mut *x, key);

                if node.key == *key {
                    return Err("Key already exists in database");
                } else if node.key < *key {
                    assert!(node.left.is_none(), "Left node is not empty");

                    node.left = Some(Node::new(key.clone(), value.clone()));
                } else {
                    assert!(node.right.is_none(), "Right node is not empty");

                    node.right = Some(Node::new(key.clone(), value.clone()));
                }

                return Ok(());
            }
        }
    }

    pub fn update(&mut self, key: &String, value: &String) -> Result<(), &str> {
        match self.root.as_mut() {
            None => return Err("Key not found."),
            Some(x) => {
                let node = Database::search(&mut *x, key);

                if node.key == *key {
                    // Update the value
                    node.value = value.clone();
                    return Ok(());
                } else {
                    return Err("Key not found.");
                }
            }
        }
    }

    pub fn get(&mut self, key: &String) -> Result<String, &str> {
        match self.root.as_mut() {
            None => return Err("Key not found."),
            Some(x) => {
                let node = Database::search(&mut *x, key);

                if node.key == *key {
                    return Ok(node.value.clone());
                } else {
                    return Err("Key not found.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_error_on_duplicate() {
        let mut db = Database::new();
        let result = db.add(&String::from("Test"), &String::from("Other test"));
        assert!(result.is_ok());

        let result = db.add(&String::from("Test"), &String::from("Some Other test"));
        assert!(result.is_err());
    }

    #[test]
    fn get_added_value() {
        let mut db = Database::new();
        let result = db.add(&String::from("Test"), &String::from("Other Test"));
        assert!(result.is_ok());

        let result = db.get(&String::from("Test"));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(String::from("Other Test")));
    }

    #[test]
    fn get_added_value_from_big_tree() {
        let mut db = Database::new();
        for i in 1..=10 {
            assert!(db.add(&i.to_string(), &i.to_string()).is_ok());
        }

        let result = db.get(&String::from("10"));
        assert!(result.is_ok_and(|val| val == "10"))
    }

    #[test]
    fn add_and_update_value() {
        let mut db = Database::new();
        let key = String::from("Test");
        let result = db.add(&key, &String::from("Other Test"));
        assert!(result.is_ok());

        let result = db.update(&key, &String::from("Some Other Test"));
        assert!(result.is_ok());

        let result = db.get(&key);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(String::from("Some Other Test")));
    }
}
