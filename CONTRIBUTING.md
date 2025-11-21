# Contributing to AxiomHive

1) Fork and branch: `git checkout -b feature/your-feature`.
2) Tooling:
   - Rust nightly not required; prefer stable.
   - Windows: install MSVC build tools (link.exe) before `cargo test`.
3) Build and test:
   - Core: `cargo fmt && cargo clippy && cargo test`.
   - Tauri shell: `installer/build.ps1` (Windows) or `installer/build.sh` (macOS/Linux).
   - WASM: `cargo build --target wasm32-wasi` (for browser/edge).
4) Docs: update `README.md`, `docs/`, and `openapi.yaml` when API or behavior changes; keep safety/payment/network defaults documented.
5) Commits: use clear messages; include test evidence in PR description; do not commit secrets or payment tokens.
6) Safety: ensure verified mode fails closed on proof errors; default network stays off unless explicitly enabled.
