# AxiomHive - Edge-Native, Verified AI

AxiomHive fuses parallel State Space Models (Mamba-2) with attention heads (5:1 ratio) to deliver fast creative generation and deterministically verified responses on any device (desktop, browser/WASM, mobile, Raspberry Pi). Verified mode emits a C=0 signature with Lean/EZKL proof hooks; Creative mode focuses on low-latency sampling and multi-modal hooks.

## Quickstart (local dev)

1) Install Rust (stable) and Node (only needed if you rebuild the browser UI).
2) Start the edge node + REST API: `cargo run` (serves on `http://localhost:8090`).
3) Open `public/index.html` (or `npm serve` / `python -m http.server`) to hit the API.
4) Build the Tauri desktop shell: `installer/build.ps1` (Windows) or `installer/build.sh` (macOS/Linux).

## Safety, control, ownership

- Offline by default: outbound network is blocked unless `AXIOMHIVE_ALLOW_NETWORK=1` or the UI checkbox is enabled for a request.
- Verified mode payment: required unless `AXIOMHIVE_REQUIRE_PAYMENT=0` or the UI "Free local" checkbox is checked (and node policy permits).
- Axiom set is required for verified calls; requests fail closed if verification fails or policy denies.
- Safety gating uses blocklists and escalation for high-risk terms before any generation runs (see `policy/` and API handlers).

## Project layout

- `src/model/` - HybridBlock (parallel SSM + attention), meta-token injector, sliding-window/global attention mix.
- `src/verification/` - Lean-compatible axiom checker, C=0 signature, EZKL/Halo2 proof hook.
- `src/dag/` - DAG scheduler, Merkle checkpoints, libp2p peer registry for distributed compute.
- `src/payment/` - Lightning billing helper (Neutrino/SPV ready).
- `src-tauri/` - Tauri desktop shell invoking the Rust core.
- `public/` - Browser UI hitting the REST endpoints.
- `openapi.yaml` - REST/OpenAPI contract for creative and verified flows.
- `docs/` - Roadmap, sample axiom sets, and operational notes.
- `policy/` - Safety policy gating (blocklist, consent/escalation, payment requirement).

## Dual modes

- Creative: probabilistic SSM + attention sampling, multi-modal hooks (text/image/audio/PDF). Faster, lower-stakes.
- Verified: deterministic generation + axiom enforcement + C=0 signature; EZKL/Halo2 proof hook for ZK attestations; Lightning micropayment gating per proof; fails closed on verification failure.

## API (edge node)

- `POST /api/v1/creative` -> `{ prompt, media, temperature, top_k }`
- `POST /api/v1/verified` -> `{ prompt, axiom_set, max_steps, payment_token, allow_network, free_local }`

See `openapi.yaml` for full schemas.

## Browser/edge (WASM)

`src/lib.rs` exposes `wasm_generate` for deterministic responses; build with `wasm-pack` or `cargo build --target wasm32-wasi`.

## Roadmap and examples

See `docs/roadmap.md` for the 90-day plan and `docs/examples.md` for Creative/Verified request examples. A sample regulated axiom set lives in `docs/axioms/finance.md`.
