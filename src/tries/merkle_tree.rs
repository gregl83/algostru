use sha2::{
    Digest,
    Sha256
};

type Hash = [u8; 32];

pub fn hash_value(value: usize) -> Hash {
    let mut hash = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(value.to_be_bytes());
    hash.copy_from_slice(&hasher.finalize());
    hash
}

pub fn hash_pair(left: Hash, right: Hash) -> Hash {
    let mut combined = &mut [];

    for i in 0..64 {
        if i < 32 {
            combined[i] = left[i];
        } else {
            combined[i] = right[i - 32];
        }
    }

    let mut hash = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(combined);
    hash.copy_from_slice(&hasher.finalize());
    hash
}

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    hash: Hash
}

impl Node {
    pub fn new_parent(left: Box<Node>, right: Option<Box<Node>>) -> Self {
        let left_hash = left.hash;
        let right_hash = match &right {
            Some(r) => r.hash,
            None => left_hash
        };
        let hash = hash_pair(left_hash, right_hash);
        Node {
            left: Some(left),
            right,
            hash
        }
    }

    pub fn new_leaf(value: usize) -> Self {
        let hash = hash_value(value);
        Node {
            left: None,
            right: None,
            hash
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leaf_node() {
        let value = 100;
        let node = Node::new_leaf(value);
        let expected = [95, 203, 162, 99, 59, 239, 28, 41, 66, 14, 14, 237, 123, 3, 124, 237, 139, 0, 70, 107, 14, 143, 28, 92, 225, 202, 210, 233, 126, 17, 122, 173];
        assert_eq!(node.hash, expected);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_new_parent_node_single_child() {
        // fixme!
    }

    #[test]
    fn test_new_parent_node_two_children() {
        // fixme!
    }
}