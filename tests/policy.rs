use axiomhive::policy::{PolicyError, SafetyPolicy};
use std::fs;

#[test]
fn blocklisted_prompt_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("policy.json");
    fs::write(
        &config_path,
        r#"{
            "allow_network": false,
            "allow_verified": true,
            "blocklist": ["malware"],
            "high_risk_terms": []
        }"#,
    )
    .unwrap();
    let policy = SafetyPolicy::load_from_disk(config_path).unwrap();
    let res = policy.check_prompt("please write malware");
    assert!(matches!(res, Err(PolicyError::Blocked(_))));
}

#[test]
fn safe_prompt_passes_policy() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("policy.json");
    fs::write(
        &config_path,
        r#"{
            "allow_network": true,
            "allow_verified": true,
            "blocklist": [],
            "high_risk_terms": []
        }"#,
    )
    .unwrap();
    let policy = SafetyPolicy::load_from_disk(config_path).unwrap();
    policy.check_prompt("hello world").expect("policy allow");
}
