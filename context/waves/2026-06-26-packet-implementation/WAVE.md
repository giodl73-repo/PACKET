# Wave: PACKET Implementation

## Goal

Build the PACKET pipeline from the accepted work packages (WP-001..006), one work package
per pulse, each compiling and testing green before the next starts.

## Thesis

The left side of the V is settled (`docs/vtrace/`). This wave is the implementation build: turn
accepted work packages into tested Rust crates, bottom-up, with scale threaded through the
corpus and gap layers, and every pulse running the WP verification commands and recording
evidence back into the VTRACE trace.

## Pulse table

| Pulse | Work Package | Status | Outcome |
|------:|--------------|--------|---------|
| 01 | WP-001 `packet-network` | done | Network kernel: identity, connectivity, diverse-path/redundancy, latency helpers. |
| 02 | WP-002 `packet-corpus` | done | Corpus model + scale/market tags + schema + sources + evidence labels. |
| 03 | WP-003 `packet-score` | done | Dimension scoring DIM-01..13 + rubric record. |
| 04 | WP-004 `packet-tier` | done | Tier T1–T4 + SLA conformance + tier-SLA gap. |
| 05 | WP-005 `packet-gap` | done | Gap analysis (scale-filtered), null result, and tail-gap detection. |
| 06 | WP-006 `packet-cli` | done | CLI orchestration (`--scale`) + reproducible corpus/gap outputs. |

## Success criteria

- Each work package meets its exit criteria and verification commands.
- Workspace stays green (`cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`) after every pulse.
- `proof check .` stays clean.
- VTRACE trace/verification rows updated as each WP closes.

## Current validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p packet-cli -- --help
cargo run -p packet-cli -- gap --scale regional --corpus corpus
```

The regional gap run reports 10 regional entries, 0 systemic mean-gap regions,
and 2 tail-gap regions. The tail-gap signal is the implemented fix for the
failure mode where a corpus mean clears the adequacy bar while under-served
markets remain clustered in the bottom tail.
