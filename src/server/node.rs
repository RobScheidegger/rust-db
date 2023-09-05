use std::sync::RwLock;

pub struct Node<T> {
    pub key: T,
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub lock: RwLock<bool>,
}

impl<T> Node<T> {
    pub fn new(key: T, value: T) -> Box<Node<T>> {
        return Box::new(Node {
            key,
            value,
            left: None,
            right: None,
            lock: RwLock::new(true),
        });
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn add() {
        assert_eq!(1 + 1, 2);
    }
}
