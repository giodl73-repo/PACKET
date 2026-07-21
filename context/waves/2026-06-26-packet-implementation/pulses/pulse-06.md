# Pulse 06: WP-006 `packet-cli` orchestration

Status: pending (blocked by WP-005). Executes WP-006.

`packet` CLI exposing `corpus`, `score`, `tier-sla`, and `gap` subcommands (with a
`--scale` filter) that drive the pipeline and emit reproducible artifacts with labels and
scale preserved. Creates `crates/packet-cli/`. No VTRACE subcommands — product/process
split per IMPLEMENTATION_PLAN.

Parent: REQ-001 · IF-006 · PKG-006 · CR-003/006/008.

Exit: `cargo run -p packet-cli -- --help` lists product subcommands and `--scale`;
end-to-end run on a seed corpus at a chosen scale regenerates artifacts deterministically;
`cargo test --workspace` green. On completion: VER-001 → passed; TRACE REQ-001 →
implemented; WP-006 → done; pipeline end-to-end; L2 readiness review.
