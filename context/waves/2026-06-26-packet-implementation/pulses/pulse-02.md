# Pulse 02: WP-002 `packet-corpus` model, scale tags, schema, sources, labels

Status: pending (blocked by WP-001). Executes WP-002.

Represent one element as a labelled, sourced, **scale-tagged**, tiered corpus entry, and
hold/reject unidentified, uncited, or untagged-scale rows. Creates `crates/packet-corpus/`,
`corpus/SCHEMA.md` (IF-001 incl. scale enum), and `data/sources.md` (IF-002).

Parent: REQ-001/002/003/005/016 · SPEC-002/003/009/013 · IF-001/002 · PKG-002 ·
CR-003/004/007.

Exit: `cargo test -p packet-corpus` green (missing-id reject, uncited quantity held,
**missing-scale held**, label preservation); `proof check .` clean. On completion:
VER-001/002/003/016 → passed; TRACE REQ-001/002/003/005/016 → implemented; WP-002 → done;
unblock WP-003.
