# Pulse 03: WP-003 `packet-score` dimension scoring

Status: pending (blocked by WP-002). Executes WP-003.

Score DIM-01..13 on a 0–10 scale with a versioned rubric record; values provisional and
labelled (no calibration yet). Creates `crates/packet-score/`.

Parent: REQ-006 · SPEC-004 · IF-003 · PKG-003 · CR-004.

Exit: `cargo test -p packet-score` green; score-bounds invariant tested; rubric version
present. On completion: VER-006 → passed; TRACE REQ-006 → implemented; WP-003 → done;
unblock WP-004.
