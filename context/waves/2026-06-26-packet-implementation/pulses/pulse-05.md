# Pulse 05: WP-005 `packet-gap` gap analysis (scale-filtered)

Status: pending (blocked by WP-004). Executes WP-005.

Plot scored elements in dimension space, find under-served regions **at a chosen scale**,
and record an already-adequate market as a labelled null result (REQ-008). Cross-scale
comparisons require an explicit marker (REQ-016). Integrates tier-SLA gaps from
`packet-tier`. Creates `crates/packet-gap/`.

Parent: REQ-008/016 · SPEC-006/013 · PKG-005 · CR-004.

Exit: `cargo test -p packet-gap` green (one found gap at a scale, one null-result case,
and other-scale elements excluded unless a cross-scale marker is set). On completion:
VER-008/016 → passed; TRACE REQ-008/016 → implemented; WP-005 → done; unblock WP-006.
