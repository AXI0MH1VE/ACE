use blake3::Hasher;

use super::merkletree::MerkleTree;

#[derive(Clone, Default)]
pub struct DagScheduler {
    pub peers: Vec<String>,
}

impl DagScheduler {
    pub fn schedule_task(&self, task: &str) -> String {
        format!("scheduled: {} across {} peers", task, self.peers.len())
    }

    pub fn record_checkpoint(&self, payload: &str) -> String {
        let mut hasher = Hasher::new();
        hasher.update(payload.as_bytes());
        let leaf = hasher.finalize().to_hex().to_string();
        let tree = MerkleTree::from_leaves(vec![leaf.clone()]);
        tree.root().unwrap_or(leaf)
    }
}
