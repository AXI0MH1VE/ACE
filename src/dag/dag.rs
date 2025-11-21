use blake3::Hasher;
use serde::Serialize;

use super::merkletree::MerkleTree;

#[derive(Debug, Clone, Serialize)]
pub struct DagNode {
    pub id: String,
    pub node_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RequestDag {
    pub nodes: Vec<DagNode>,
}

impl RequestDag {
    pub fn add_node(&mut self, node_type: &str, payload: serde_json::Value) -> DagNode {
        let id = format!("{}-{}", node_type, self.nodes.len());
        let node = DagNode {
            id,
            node_type: node_type.to_string(),
            payload,
        };
        self.nodes.push(node.clone());
        node
    }

    pub fn merkle_root(&self) -> Option<String> {
        if self.nodes.is_empty() {
            return None;
        }
        let leaves = self.nodes.iter().map(|n| node_hash(n)).collect::<Vec<_>>();
        let tree = MerkleTree::from_leaves(leaves);
        tree.root()
    }
}

#[derive(Clone, Default)]
pub struct DagScheduler {
    pub peers: Vec<String>,
}

impl DagScheduler {
    pub fn schedule_task(&self, task: &str) -> String {
        format!("scheduled: {} across {} peers", task, self.peers.len())
    }

    pub fn record_checkpoint(&self, dag: &RequestDag) -> Option<String> {
        dag.merkle_root()
    }
}

fn node_hash(node: &DagNode) -> String {
    let serialized =
        serde_json::to_vec(node).unwrap_or_else(|_| format!("{:?}", node.node_type).into_bytes());
    let mut hasher = Hasher::new();
    hasher.update(&serialized);
    hasher.finalize().to_hex().to_string()
}
