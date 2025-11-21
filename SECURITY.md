# Security Policy

## Reporting
- Report vulnerabilities to security@axiomhive.local with `[SECURITY]` in the subject.
- Include repro steps, impact, proof-of-concept, and environment details (OS, commit hash).

## Scope
- In scope: hybrid model core (`src/model`), verification/axiom handling (`src/verification`), payment logic (`src/payment`), DAG/p2p (`src/dag`), API handlers (`src/main.rs`), Tauri shell, and browser UI.
- Out of scope: upstream third-party dependencies (report upstream), user-supplied models/axioms, or misconfigured Lightning nodes.

## Operational defaults
- Network access is off by default; enable with `AXIOMHIVE_ALLOW_NETWORK=1` or per-request.
- Verified mode requires payment unless `AXIOMHIVE_REQUIRE_PAYMENT=0` or `free_local=true` and policy allows.
- Outputs in verified mode fail closed if proofs or checks cannot be produced.

## Updates
- Dependencies are tracked via Cargo.lock and SBOM in `release/sbom.json`. Run `cargo audit` and keep proofs/axioms signed where applicable.
