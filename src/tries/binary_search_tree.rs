#[derive(Debug)]
pub struct Node {
    value: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: isize) -> Self {
        Node {
            value,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, value: isize) {
        if value < self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Self::new(value)))
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Self::new(value)))
            }
        }
    }

    pub fn get(&mut self, value: isize) -> Option<&Node> {
        if self.value == value { return Some(self); }

        if value < self.value {
            match self.left {
                Some(ref mut node) => node.get(value),
                None => None
            }
        } else {
            match self.right {
                Some(ref mut node) => node.get(value),
                None => None
            }
        }
    }

    pub fn len(&self) -> usize {
        // todo
        1
    }

    pub fn count_leaves(&self) -> usize {
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        let left_leaves = match &self.left {
            None => 0,
            Some(left) => left.count_leaves()
        };

        let right_leaves = match &self.right {
            None => 0,
            Some(right) => right.count_leaves()
        };

        left_leaves + right_leaves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_node() {
        let mut tree = Node::new(10);

        tree.insert(0);
        tree.insert(-5);
        tree.insert(5);

        tree.insert(20);
        tree.insert(15);
        tree.insert(25);

        let result = tree.get(10).unwrap();
        assert_eq!(result.value, 10);
        let left = &result.left.as_ref().unwrap();
        assert_eq!(left.value, 0);
        let right = &result.right.as_ref().unwrap();
        assert_eq!(right.value, 20);
    }

    #[test]
    fn test_get_leaf_parent_node() {
        let mut tree = Node::new(10);

        tree.insert(0);
        tree.insert(-5);
        tree.insert(5);

        tree.insert(20);
        tree.insert(15);
        tree.insert(25);

        let result = tree.get(20).unwrap();
        assert_eq!(result.value, 20);
        let left = &result.left.as_ref().unwrap();
        assert_eq!(left.value, 15);
        let right = &result.right.as_ref().unwrap();
        assert_eq!(right.value, 25);
    }

    #[test]
    fn test_get_leaf_node() {
        let mut tree = Node::new(10);

        tree.insert(0);
        tree.insert(-5);
        tree.insert(5);

        tree.insert(20);
        tree.insert(15);
        tree.insert(25);

        let result = tree.get(-5).unwrap();
        assert_eq!(result.value, -5);
        let left = &result.left.as_ref();
        assert!(left.is_none());
        let right = &result.right.as_ref();
        assert!(right.is_none());
    }

    #[test]
    fn test_count_leaves() {
        let expectation: usize = 4;

        let mut tree = Node::new(10);

        tree.insert(0);
        tree.insert(-5);
        tree.insert(5);

        tree.insert(20);
        tree.insert(15);
        tree.insert(25);

        let result = tree.count_leaves();
        assert_eq!(result, expectation);
    }
}