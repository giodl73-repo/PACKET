# Pulse 01: WP-001 `packet-network` network kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The network graph kernel — the pipeline primitive every other crate depends on.
Implements the load-bearing identity, connectivity, and diverse-path (redundancy basis)
invariants required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/packet-network`).
- `crates/packet-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/packet-network/src/lib.rs`: `Node`, `Link`, `Network`, `NetworkError`;
  `add_node`/`add_link` (identity + validation); `node_count`, `link_count`, `degree`,
  `is_connected`, `has_diverse_path`, `incident_capacity_mbps`.

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p packet-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident capacity;
  `has_diverse_path` true on a ring/mesh and false on a single-path chain (redundancy
  basis); duplicate-node, non-positive capacity, unknown-node typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p packet-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented;
WORK_PACKAGES WP-001 → done; unblock WP-002.

## Status

Completed — the six-crate workspace and validation baseline are implemented.
