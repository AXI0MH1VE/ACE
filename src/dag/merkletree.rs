use blake3::Hasher;

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
        if self.nodes.is_empty() {
            return None;
        }
        let mut layer: Vec<String> = self.nodes.iter().map(|n| n.hash.clone()).collect();
        while layer.len() > 1 {
            let mut next = Vec::new();
            for pair in layer.chunks(2) {
                if pair.len() == 1 {
                    next.push(pair[0].clone());
                } else {
                    let mut hasher = Hasher::new();
                    hasher.update(pair[0].as_bytes());
                    hasher.update(pair[1].as_bytes());
                    next.push(hasher.finalize().to_hex().to_string());
                }
            }
            layer = next;
        }
        layer.first().cloned()
    }
}
