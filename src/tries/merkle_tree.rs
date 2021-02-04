use sha2::{
    Digest,
    Sha256
};
use rand::AsByteSliceMut;

type Hash = [u8; 32];

pub fn hash_value(value: usize) -> Hash {
    let mut hash = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(value.to_le_bytes());
    hash.copy_from_slice(&hasher.finalize());
    hash
}

pub fn hash_pair(left: Hash, right: Hash) -> Hash {
    let mut combined = [0u8; 64];

    for i in 0..64 {
        if i < 32 {
            combined[i] = left[i];
        } else {
            combined[i] = right[i - 32];
        }
    }

    let mut hash = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(combined.as_byte_slice_mut());
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
        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        assert_eq!(node.hash, expected);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_new_parent_node_single_child() {
        let value = 100;
        let left = Box::new(Node::new_leaf(value));
        let parent = Node::new_parent(left, None);

        let expected = [131, 237, 132, 182, 36, 12, 210, 190, 81, 158, 169, 193, 199, 65, 69, 209, 49, 43, 179, 169, 165, 79, 41, 215, 243, 205, 158, 182, 131, 102, 81, 149];
        assert_eq!(parent.hash, expected);
        assert!(parent.left.is_some());
        assert!(parent.right.is_none());

        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        let left = parent.left.unwrap();
        assert_eq!(left.hash, expected);
        assert!(left.left.is_none());
        assert!(left.right.is_none());
    }

    #[test]
    fn test_new_parent_node_two_children() {
        let value = 100;
        let left = Box::new(Node::new_leaf(value));

        let value = 200;
        let right = Box::new(Node::new_leaf(value));

        let parent = Node::new_parent(left, Some(right));

        let expected = [83, 140, 112, 18, 32, 216, 224, 174, 61, 23, 34, 146, 208, 222, 20, 212, 230, 196, 187, 152, 146, 118, 130, 59, 153, 235, 141, 189, 255, 254, 166, 235];
        assert_eq!(parent.hash, expected);
        assert!(parent.left.is_some());
        assert!(parent.right.is_some());

        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        let left = parent.left.unwrap();
        assert_eq!(left.hash, expected);
        assert!(left.left.is_none());
        assert!(left.right.is_none());

        let expected = [202, 137, 35, 214, 196, 68, 125, 111, 166, 208, 84, 12, 175, 255, 1, 246, 71, 242, 187, 254, 143, 25, 147, 150, 134, 206, 111, 167, 176, 218, 238, 40];
        let right = parent.right.unwrap();
        assert_eq!(right.hash, expected);
        assert!(right.left.is_none());
        assert!(right.right.is_none());
    }
}