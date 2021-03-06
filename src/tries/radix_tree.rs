type Child = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    children: (Child, Child)
}

impl Node {
    pub fn new() -> Self {
        Node {
            children: (None, None)
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.0.is_none() && self.children.1.is_none()
    }

    pub fn insert(&mut self, value: &[u8]) {
        if value.len() == 0 { return; }
        match value[0] {
            0 => {
                if self.children.0.is_none() {
                    self.children.0 = Some(Box::new(Node::new()));
                }
                let node = self.children.0.as_mut().unwrap();
                node.insert(&value[1..value.len()])
            },
            _ => {
                if self.children.1.is_none() {
                    self.children.1 = Some(Box::new(Node::new()));
                }
                let node = self.children.1.as_mut().unwrap();
                node.insert(&value[1..value.len()]);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_binary_radix() {
        let test: [u8; 4] = [0, 1, 0, 1];

        let mut tree = Node::new();
        tree.insert(&test);

        // level 1 (root)
        assert!(tree.children.0.is_some());
        assert!(tree.children.1.is_none());

        // level 2
        let l2 = tree.children.0.unwrap();
        assert!(l2.children.0.is_none());
        assert!(l2.children.1.is_some());

        // level 3
        let l3 = l2.children.1.unwrap();
        assert!(l3.children.0.is_some());
        assert!(l3.children.1.is_none());

        // level 4
        let l4 = l3.children.0.unwrap();
        assert!(l4.children.0.is_none());
        assert!(l4.children.1.is_some());

        // level 5
        let l5 = l4.children.1.unwrap();
        assert!(l5.children.0.is_none());
        assert!(l5.children.1.is_none());
    }

    #[test]
    fn test_new_binary_radix_two_branches() {
        let branch_zero: [u8; 3] = [0, 1, 0];
        let branch_one: [u8; 3] = [0, 0, 0];

        let mut tree = Node::new();
        tree.insert(&branch_zero);
        tree.insert(&branch_one);

        // level 0 (root) - branch 0 + 1
        assert!(tree.children.0.is_some());
        assert!(tree.children.1.is_none());

        // level 1 - branch 0 + 1
        let l1 = tree.children.0.unwrap();
        assert!(l1.children.0.is_some());
        assert!(l1.children.1.is_some());

        // level 2 - branch 0
        let l2b0 = l1.children.0.unwrap();
        assert!(l2b0.children.0.is_some());
        assert!(l2b0.children.1.is_none());

        // level 2 - branch 1
        let l2b1 = l1.children.1.unwrap();
        assert!(l2b1.children.0.is_some());
        assert!(l2b1.children.1.is_none());

        // level 3 - branch 0
        let l3b0 = l2b0.children.0.unwrap();
        assert!(l3b0.children.0.is_none());
        assert!(l3b0.children.1.is_none());

        // level 3 - branch 1
        let l3b1 = l2b1.children.0.unwrap();
        assert!(l3b1.children.0.is_none());
        assert!(l3b1.children.1.is_none());
    }
}