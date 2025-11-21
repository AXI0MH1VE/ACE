# AxiomHive — Edge-Native, Verified AI

AxiomHive fuses parallel State Space Models (Mamba-2) with attention heads (5:1 ratio) to deliver creative generation and deterministically verified responses on any device: desktop, browser (WASM), mobile, and Raspberry Pi. Verified mode produces a C=0 signature plus EZKL/Halo2 proof hooks; Creative mode focuses on fast multi-modal sampling.

## Quickstart (local dev)

1. Install Rust (stable) and Node (if you want to rebuild the browser UI).
2. Start the edge node + REST API: `cargo run` (serves on `localhost:8090`).
3. Open `public/index.html` (or `npm serve` / `python -m http.server`) to hit the API.
4. Build the Tauri desktop shell: run `installer/build.ps1` (Windows) or `installer/build.sh` (macOS/Linux).

## Project Layout

- `src/model/` — HybridBlock (parallel SSM + attention), meta-token injector, sliding-window/global attention mix.
- `src/verification/` — Lean-compatible axiom checker, C=0 signature, EZKL/Halo2 proof hook.
- `src/dag/` — DAG scheduler, Merkle checkpoints, libp2p peer registry for distributed compute.
- `src/payment/` — Lightning billing helper (Neutrino/SPV ready).
- `src-tauri/` — Tauri desktop shell invoking the Rust core.
- `public/` — Browser UI hitting the REST endpoints.
- `openapi.yaml` — REST/OpenAPI contract for creative and verified flows.
- `docs/` — Roadmap, sample axiom sets, and operational notes.

## Dual Modes

- **Creative**: probabilistic SSM+attention sampling, multi-modal hooks (text/image/audio/PDF). Faster, lower-stakes.
- **Verified**: deterministic generation + axiom enforcement + C=0 signature; EZKL/Halo2 proof hook for ZK attestations; Lightning micropayment gating per proof.

## API (edge node)

- `POST /api/v1/creative` → `{ prompt, media, temperature, top_k }`
- `POST /api/v1/verified` → `{ prompt, axiom_set, max_steps }`

See `openapi.yaml` for the full schema.

## Browser/Edge (WASM)

`src/lib.rs` exposes `wasm_generate` for deterministic responses; compile with `wasm-pack` or `cargo build --target wasm32-wasi`.

## Roadmap

See `docs/roadmap.md` for a 90-day launch plan and `docs/axioms/finance.md` for a sample regulated axiom set.
