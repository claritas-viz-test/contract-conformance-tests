# Peer-authority canary

This public `claritas-viz-test` canary contains synthetic test-only contract data. It does not copy or certify private Claritas product source.

Two files are independently authored first-class authorities:

- `contracts/peer-authority-canary/typespec/main.tsp` — TypeSpec authority;
- `contracts/peer-authority-canary/json-schema/authored.schema.json` — JSON Schema Draft 2020-12 authority.

The immutable TJSV action compiles TypeSpec to a generated JSON Schema B under `.typespec-json-schema-validator/`, then compares that generated witness with the independently authored JSON Schema A. Generated Schema B, parity reports, Contract IR, verification receipts and artifacts are evidence only; none is an editable authority.

Positive and negative recorded instances are evaluated by both lanes, and current-input Contract IR verification is mandatory before the job can pass. The job also proves neither authored source changed during comparison.

This repository intentionally uses split `typespec/` and `json-schema/` directories to exercise the structural family used by Claritas renderer contracts and the corresponding `ores-cli` lint work.

Tracking: #5 / DEN-2308 / ORESoftware/typespec-json-schema-validator#20.
