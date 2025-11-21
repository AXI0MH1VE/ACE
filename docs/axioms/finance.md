# Finance Axiom Set (GAAP2025)

- All journal entries must balance (total debits == total credits).
- Revenue recognition must reference `contract_id`, `recognition_schedule`, and supporting evidence.
- Expense capitalization requires `asset_class`, `amortization_period`, and depreciation method.
- Cashflow statements must reconcile net income with operating cash via explicit adjustments.
- Fraud guard: entries above 5% materiality require dual signatures and UTC timestamp.
- Proof policy: C=0 signature must include `axiom_hash` of this document, `proof_cert` from Lean/EZKL, and Merkle root for checkpointing.
- Network policy: if `allow_network` is false, only local axiom files are permitted; remote axiom fetches must be rejected.
