# Example Sessions

## Creative Mode (text)
- Prompt: "Draft a launch headline for a zero-hallucination AI."
- Request: `POST /api/v1/creative` with `{ "prompt": "...", "media": ["text"], "temperature": 0.85 }`
- Expected: Returns fused SSM+attn sample with `mode="creative"` and `request_id`.

## Verified Mode (finance)
- Prompt: "Produce GAAP2025-compliant revenue recognition steps for contract ACME-442."
- Request: `POST /api/v1/verified` with `{ "prompt": "...", "axiom_set": "finance.gaap2025", "max_steps": 2048 }`
- Expected: Deterministic output, `c0_signature` containing `input_hash`, `axiom_hash`, `state_trace`, `proof_cert`, plus `merkle_root` for checkpointing.
