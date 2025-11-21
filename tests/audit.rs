use axiomhive::audit::AuditLog;
use axiomhive::dag::dag::RequestDag;
use serde_json::json;
use uuid::Uuid;

#[test]
fn merkle_root_is_deterministic() {
    let mut dag = RequestDag::default();
    dag.add_node("policy", json!({"ok": true}));
    dag.add_node("model", json!({"run": 1}));
    let first = dag.merkle_root().expect("root");
    let mut dag2 = RequestDag::default();
    dag2.add_node("policy", json!({"ok": true}));
    dag2.add_node("model", json!({"run": 1}));
    let second = dag2.merkle_root().expect("root");
    assert_eq!(first, second);
}

#[test]
fn audit_log_chains_hashes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("audit.jsonl");
    let log = AuditLog::new(&path);

    let mut dag = RequestDag::default();
    dag.add_node("policy", json!({"ok": true}));
    let first = log
        .append(Uuid::new_v4(), "creative", &dag)
        .expect("append");

    let second = log
        .append(Uuid::new_v4(), "verified", &dag)
        .expect("append2");

    assert_eq!(second.prev_hash, first.hash);
}
