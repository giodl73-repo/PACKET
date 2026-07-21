# PACKET Product Plan

## Thesis

Score connectivity networks at a declared scale, identify measurable
infrastructure and service gaps, and design Internet 2.0 interventions only
where evidence supports them.

## Implemented product shape

- Six-crate Rust workspace covering network, corpus, score, tier, gap, and CLI.
- International, national, regional, and local scale contracts.
- DIM-01..13 scoring and tier-SLA shortfall artifacts.
- Tail-versus-systemic gap classification.
- Deterministic tests and machine-readable CLI outputs.

## Current evidence

The first cited broadband-divide analysis reports a majority-below-bar result
for both tested dimensions. Broader source coverage, competition evidence, and
path-diversity analysis remain next steps.

## Next public work

1. Publish reproducible source manifests for cited connectivity runs.
2. Expand middle-mile, peering, affordability, and redundancy evidence.
3. Add sensitivity analysis across scales and market definitions.
4. Review the first gap-targeted intervention through the full panel.

## Non-goals

- No RF, optical, routing, regulatory, tariff, or franchise design.
- No forecast of what agencies or carriers will fund or build.
- No uncited coverage, latency, availability, affordability, or cost claim.
- No aggregation across scales without an explicit comparison basis.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p packet-cli -- --help
```
