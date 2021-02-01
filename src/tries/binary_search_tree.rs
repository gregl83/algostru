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
    fn test_count_leaves() { ;
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