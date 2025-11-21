use crate::dag::dag::{DagNode, RequestDag};
use blake3::Hasher;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub request_id: Uuid,
    pub mode: String,
    pub merkle_root: String,
    pub dag: Vec<DagNode>,
    pub prev_hash: String,
    pub hash: String,
}

pub struct AuditLog {
    path: PathBuf,
    last_hash: Mutex<String>,
}

impl AuditLog {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        let last_hash = Self::read_last_hash(&path_buf).unwrap_or_default();
        Self {
            path: path_buf,
            last_hash: Mutex::new(last_hash),
        }
    }

    fn read_last_hash(path: &Path) -> Option<String> {
        if !path.exists() {
            return None;
        }
        let file = fs::File::open(path).ok()?;
        let reader = BufReader::new(file);
        reader
            .lines()
            .filter_map(Result::ok)
            .last()
            .and_then(|line| serde_json::from_str::<AuditEntry>(&line).ok())
            .map(|entry| entry.hash)
    }

    pub fn append(&self, request_id: Uuid, mode: &str, dag: &RequestDag) -> Result<AuditEntry> {
        fs::create_dir_all(self.path.parent().unwrap_or_else(|| Path::new(".")))?;

        let merkle_root = dag.merkle_root().unwrap_or_else(|| "empty-dag".to_string());

        let prev_hash = { self.last_hash.lock().unwrap().clone() };
        let entry = AuditEntry {
            timestamp: Utc::now().to_rfc3339(),
            request_id,
            mode: mode.to_string(),
            merkle_root,
            dag: dag.nodes.clone(),
            prev_hash,
            hash: String::new(),
        };

        let hash = Self::compute_hash(&entry);
        let mut finalized = entry.clone();
        finalized.hash = hash.clone();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(file, "{}", serde_json::to_string(&finalized)?)?;

        let mut guard = self.last_hash.lock().unwrap();
        *guard = hash;

        Ok(finalized)
    }

    fn compute_hash(entry: &AuditEntry) -> String {
        let mut hasher = Hasher::new();
        hasher.update(entry.prev_hash.as_bytes());
        hasher.update(serde_json::to_string(entry).unwrap_or_default().as_bytes());
        hasher.finalize().to_hex().to_string()
    }
}
use anyhow::Result;
