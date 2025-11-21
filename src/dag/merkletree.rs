pub struct MerkleNode {
    pub hash: String,
}

pub struct MerkleTree {
    pub nodes: Vec<MerkleNode>,
}

impl MerkleTree {
    pub fn from_leaves(leaves: Vec<String>) -> Self {
        let nodes = leaves.into_iter().map(|h| MerkleNode { hash: h }).collect();
        Self { nodes }
    }

    pub fn root(&self) -> Option<String> {
        self.nodes.first().map(|n| n.hash.clone())
    }
}
