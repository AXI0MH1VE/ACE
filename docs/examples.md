# Example Sessions

## Creative mode (text)
- Prompt: "Draft a launch headline for a zero-hallucination AI."
- Request: `POST /api/v1/creative` with `{ "prompt": "...", "media": ["text"], "temperature": 0.85, "lightning_invoice": "<lnbc...>" }`
- Expected: Fused SSM+attention sample with `mode="creative"` and `request_id`; denied if invoice missing/invalid when paywall is on.

## Verified mode (finance, offline)
- Prompt: "Produce GAAP2025-compliant revenue recognition steps for contract ACME-442."
- Request: `POST /api/v1/verified` with `{ "prompt": "...", "axiom_set": "finance.gaap2025", "max_steps": 2048, "free_local": true, "allow_network": false }`
- Expected: Deterministic output, `c0_signature` with `input_hash`, `axiom_hash`, `state_trace`, `proof_cert`, plus `merkle_root`. Output is suppressed if verification fails.

## Verified mode (networked + paid)
- Prompt: "Generate a Lean-proofed compliance summary for invoice INV-991."
- Request: `POST /api/v1/verified` with `{ "prompt": "...", "axiom_set": "finance.gaap2025", "max_steps": 4096, "lightning_invoice": "<lnbc...>", "allow_network": true, "free_local": false }`
- Expected: Deterministic output with proof URI, C0 signature, and Merkle root; request denied if invoice is missing/invalid or policy blocks outbound access.
