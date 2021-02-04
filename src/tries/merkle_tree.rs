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

#[derive(Debug, Clone)]
pub struct Node {
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

fn height(nodes: usize, level: usize) -> usize {
    if nodes == 0 { return 0; }
    if nodes == 1 { return level; }

    let mut n = nodes;
    if n % 2 == 1 { n += 1; }

    height(n / 2, level + 1)
}

fn to_parents(nodes: &mut Vec<Node>) -> Vec<Node> {
    if nodes.len() <= 1 { return vec![nodes.swap_remove(0)]; }

    let mut parents = vec![];
    let range: Vec<usize> = (0..nodes.len()).collect();
    for indices in range.chunks(2) {
        let left = Box::new(nodes[indices[0]].clone());
        let mut right: Option<Box<Node>> = None;
        if indices.len() > 1 {
            right = Some(Box::new(nodes[indices[1]].clone()));
        }
        parents.push(Node::new_parent(left, right));
    }

    return to_parents(&mut parents);
}

pub fn to_tree(values: Vec<isize>) -> Node {
    let mut leafs = vec![];
    for value in values {
        leafs.push(Node::new_leaf(value as usize));
    }
    to_parents(&mut leafs).swap_remove(0)
}

struct Proof<'a> {
    root_hash: Hash,
    nodes: usize,
    bits: &'a [u8],
    hashes: Vec<Hash>,
}

// pub fn to_proof(tree: &Node, value: isize) -> Proof {
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leaf_node() {
        let node = Node::new_leaf(100);
        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        assert_eq!(node.hash, expected);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_new_parent_node_single_child() {
        let left = Box::new(Node::new_leaf(100));
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
        let left = Box::new(Node::new_leaf(100));
        let right = Box::new(Node::new_leaf(200));
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

    #[test]
    fn test_to_tree_even_values() {
        let values = vec![
            100,
            200
        ];

        let tree = to_tree(values);

        // level 1 (merkle root)
        let expected = [83, 140, 112, 18, 32, 216, 224, 174, 61, 23, 34, 146, 208, 222, 20, 212, 230, 196, 187, 152, 146, 118, 130, 59, 153, 235, 141, 189, 255, 254, 166, 235];
        assert_eq!(tree.hash, expected);

        // level 2 (leaf nodes)
        let tree_l2l = tree.left.clone().unwrap();
        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        assert_eq!(tree_l2l.hash, expected);
        assert!(tree_l2l.left.is_none());
        assert!(tree_l2l.right.is_none());

        let tree_l2r = tree.right.clone().unwrap();
        let expected = [202, 137, 35, 214, 196, 68, 125, 111, 166, 208, 84, 12, 175, 255, 1, 246, 71, 242, 187, 254, 143, 25, 147, 150, 134, 206, 111, 167, 176, 218, 238, 40];
        assert_eq!(tree_l2r.hash, expected);
        assert!(tree_l2r.left.is_none());
        assert!(tree_l2r.right.is_none());
    }

    #[test]
    fn test_to_tree_odd_with_depth() {
        let values = vec![
            100,
            200,
            300,
            400,
            500,
        ];

        let tree = to_tree(values);

        // level 1 (merkle root)
        let expected = [171, 70, 133, 81, 171, 212, 140, 114, 206, 33, 95, 19, 98, 139, 183, 17, 20, 79, 232, 79, 177, 185, 30, 9, 80, 225, 11, 107, 141, 54, 39, 113];
        assert_eq!(tree.hash, expected);

        // level 2
        let tree_l2l = tree.left.clone().unwrap();
        let expected = [166, 117, 160, 130, 34, 182, 101, 190, 13, 105, 248, 103, 2, 79, 236, 77, 168, 236, 4, 84, 75, 234, 69, 16, 167, 188, 252, 70, 87, 0, 230, 138];
        assert_eq!(tree_l2l.hash, expected);

        let tree_l2r = tree.right.clone().unwrap();
        let expected = [229, 137, 125, 209, 115, 163, 74, 120, 96, 208, 18, 201, 111, 162, 226, 145, 31, 240, 211, 153, 76, 203, 9, 21, 174, 228, 46, 238, 183, 200, 168, 193];
        assert_eq!(tree_l2r.hash, expected);

        // level 3
        let tree_l3ll = tree_l2l.left.clone().unwrap();
        let expected = [83, 140, 112, 18, 32, 216, 224, 174, 61, 23, 34, 146, 208, 222, 20, 212, 230, 196, 187, 152, 146, 118, 130, 59, 153, 235, 141, 189, 255, 254, 166, 235];
        assert_eq!(tree_l3ll.hash, expected);

        let tree_l3lr = tree_l2l.right.clone().unwrap();
        let expected = [242, 120, 220, 100, 66, 200, 37, 235, 18, 52, 182, 208, 53, 222, 76, 142, 223, 152, 25, 44, 159, 227, 167, 253, 225, 244, 247, 80, 58, 80, 180, 8];
        assert_eq!(tree_l3lr.hash, expected);

        let tree_l3rl = tree_l2r.left.clone().unwrap();
        let expected = [118, 72, 64, 243, 140, 158, 127, 88, 41, 212, 233, 245, 147, 94, 147, 47, 255, 210, 182, 7, 121, 105, 111, 74, 86, 19, 48, 243, 156, 119, 200, 131];
        assert_eq!(tree_l3rl.hash, expected);

        let tree_l3rr = tree_l2r.right.clone();
        assert!(tree_l3rr.is_none());

        // level 4 (leaf nodes)
        let tree_l4lll = tree_l3ll.left.clone().unwrap();
        let expected = [38, 171, 57, 21, 11, 99, 48, 21, 37, 118, 228, 199, 250, 126, 12, 170, 128, 75, 94, 157, 176, 71, 106, 62, 72, 230, 181, 63, 28, 218, 130, 121];
        assert_eq!(tree_l4lll.hash, expected);
        assert!(tree_l4lll.left.is_none());
        assert!(tree_l4lll.right.is_none());

        let tree_l4llr = tree_l3ll.right.clone().unwrap();
        let expected = [202, 137, 35, 214, 196, 68, 125, 111, 166, 208, 84, 12, 175, 255, 1, 246, 71, 242, 187, 254, 143, 25, 147, 150, 134, 206, 111, 167, 176, 218, 238, 40];
        assert_eq!(tree_l4llr.hash, expected);
        assert!(tree_l4llr.left.is_none());
        assert!(tree_l4llr.right.is_none());

        let tree_l4lrl = tree_l3lr.left.clone().unwrap();
        let expected = [95, 187, 197, 10, 94, 11, 196, 225, 187, 94, 164, 187, 172, 218, 90, 230, 112, 158, 255, 142, 40, 111, 176, 2, 254, 231, 58, 73, 19, 130, 15, 233];
        assert_eq!(tree_l4lrl.hash, expected);
        assert!(tree_l4lrl.left.is_none());
        assert!(tree_l4lrl.right.is_none());

        let tree_l4lrl = tree_l3lr.left.clone().unwrap();
        let expected = [95, 187, 197, 10, 94, 11, 196, 225, 187, 94, 164, 187, 172, 218, 90, 230, 112, 158, 255, 142, 40, 111, 176, 2, 254, 231, 58, 73, 19, 130, 15, 233];
        assert_eq!(tree_l4lrl.hash, expected);
        assert!(tree_l4lrl.left.is_none());
        assert!(tree_l4lrl.right.is_none());

        let tree_l4rll = tree_l3rl.left.clone().unwrap();
        let expected = [207, 92, 178, 244, 211, 125, 182, 239, 68, 86, 171, 251, 205, 193, 18, 132, 78, 147, 194, 95, 236, 22, 116, 112, 211, 242, 229, 26, 87, 197, 209, 102];
        assert_eq!(tree_l4rll.hash, expected);
        assert!(tree_l4rll.left.is_none());
        assert!(tree_l4rll.right.is_none());

        let tree_l4rlr = tree_l3rl.right.clone();
        assert!(tree_l4rlr.is_none());
    }

    #[test]
    fn test_tree_height_from_values() {
        let values: Vec<isize> = vec![100, 200, 300, 400, 500];

        let expects = 4;
        let actual = height(values.len(), 0);

        assert_eq!(actual, expects);
    }
}