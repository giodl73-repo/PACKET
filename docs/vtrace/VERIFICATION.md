# Verification Plan

## Scope

Repo: PACKET

VTRACE adoption scope: define verification methods and command levels for PACKET's
requirements.

Current implementation update: the initial greenfield plan has since produced the
Rust workspace, seed corpus, cited broadband adoption finding, CLI, and
scale-filtered gap run. The original planning rows remain useful history, but
current evidence below records the active implementation state.

## Verification Matrix

| Req ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | `cargo run -p packet-cli -- gap --scale regional --corpus corpus` | a documented regeneration path with labels + scale preserved | passed | EVID-001 |
| REQ-002 | inspection / review | inspect corpus quantities and gap output labels | every material quantity carries an evidence label | passed | EVID-002 |
| REQ-003 | citation audit | inspect corpus + `data/sources.md` | every cited quantity resolves to a registry source or is labelled | pass_with_risk | EVID-003 (DIM-02 remains estimated, not cited per-state) |
| REQ-004 | schema check / inspection | `cargo test -p packet-corpus`; inspect corpus frontmatter keys | stable node/link/network id present; labels are not keys | passed | EVID-004 |
| REQ-005 | gate / data inspection | `cargo test -p packet-corpus missing_scale_is_held uncited_quantity_is_held` | such rows held, not promoted | passed | EVID-005 |
| REQ-006 | calibration record | inspect `packet-score` rubric v0 | rubric changes are versioned and justified | pass_with_risk | EVID-006 (v0 equal weights pending calibration) |
| REQ-007 | analysis / inspection | inspect `packet-tier` load/redundancy tests and README claims | peak-vs-average and single-vs-diverse basis named on each claim | pass_with_risk | EVID-007 |
| REQ-008 | gap inspection / review | `cargo test -p packet-gap`; regional gap run | null result recorded, no manufactured gap; tail gaps also reported | passed | EVID-008 |
| REQ-009 | review inspection | confirm parliament + editorial gate ran on a promoted claim | review records exist with dispositions | pass_with_risk | EVID-009 (panel exists, not yet exercised on a corpus claim) |
| REQ-010 | role review | confirm coverage/performance/resilience/affordability/competition/equity/cost/incumbent lenses represented | stakeholder lenses present in `.roles/` and applied | pass_with_risk | EVID-010 (`.roles/` panel built) |
| REQ-011 | editorial review | inspect public claims for scope boundary | outputs framed as research/tooling/conceptual design | pass_with_risk | EVID-011 (`README`/`PRODUCT_PLAN`/`MISSION` non-goals) |
| REQ-012 | git inspection | `git status --short`; confirm no TRACKER pointer touched | PACKET changes stay in the child repo | passed | EVID-012 |
| REQ-013 | wave ledger / review | inspect wave ledger + pulses for one-at-a-time discipline | each VTRACE stage settled to a fixed point in sequence | passed | EVID-013 |
| REQ-014 | schema check / inspection | `cargo test -p packet-tier`; inspect corpus tier fields | every element classified T1–T4 with declared SLA | passed | EVID-014 |
| REQ-015 | gate / gap inspection | `cargo test -p packet-tier`; `cargo test -p packet-gap` | tier-SLA shortfalls reported before adequacy claimed | passed | EVID-015 |
| REQ-016 | schema check / gate | `cargo test -p packet-gap`; regional gap run | every element scale-tagged; cross-scale notes explicit | passed | EVID-016 |
| REQ-DOC-001 | doc QA | `proof check .` | markdown QA clean across repo docs | passed | EVID-DOC-001 |

## Commands

```powershell
# Doc QA (active now)
proof check .
git diff --check

# Implementation levels (active)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p packet-cli -- --help
cargo run -p packet-cli -- gap --scale regional --corpus corpus
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast doc/sanity for the active VTRACE stage. | `proof check .`, `git diff --check` | passed |
| L1 | Full repo confidence before push. | L0 + `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --locked` | passed |
| L2 | Readiness proof before a public claim. | corpus regeneration + tier/SLA checks + scale-filtered gap + role review + public finding release boundary | pass_with_risk (pipeline exists; `PACKET-PF-05` now requires explicit full-panel release review before broader public-authority reuse) |

## Evidence Ledger

| Evidence ID | Type | Path / Command | Covers | Result |
|---|---|---|---|---|
| EVID-DOC-001 | report | `proof check .` (0 errors) | REQ-DOC-001 | passed |
| EVID-012 | command | `git status --short` (standalone child repo) | REQ-012 | passed |
| EVID-013 | review | `context/waves/2026-06-26-vtrace-foundation/` ledger + pulses | REQ-013 | passed |
| EVID-009..011 | review | `.roles/` panel present and applied in stage reviews | REQ-009/010/011 | pass_with_risk |
| EVID-001 | command | `cargo run -p packet-cli -- gap --scale regional --corpus corpus` | REQ-001/008/016 | passed |
| EVID-002 | inspection/tests | corpus labels and `packet-corpus` tests | REQ-002 | passed |
| EVID-003 | inspection | `data/sources.md`, `corpus/us-*.md`, finding limits | REQ-003 | pass_with_risk |
| EVID-004..008, 014..016 | commands/tests | workspace tests plus focused package tests | REQ-004..008/014/015/016 | passed |
| EVID-CR-001..003 | commands | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --locked` | code rigor | passed |
| EVID-PF-05 | policy check | `pwsh -NoProfile -File tests\check-public-finding-boundary.ps1` plus `docs/findings/public-finding-release-boundary.md` | REQ-009/010/011 / `PACKET-PF-05` | passed |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Historical VTRACE rows lag current implementation state. | Planning docs can understate the current Rust/corpus/CLI/finding evidence. | mitigated by this implementation update; keep future pulses current |
| Review gate not yet exercised on a real public corpus claim. | REQ-009/010/011 are process-verified and the finding is bounded, but full release review should be recorded for public reuse. | mitigated by public finding release boundary; broader authority remains blocked until first explicit full-panel release review |
| DIM-02 availability is estimated, not per-state cited. | Availability finding is useful order-of-magnitude context but not a precise state claim. | keep label and source-limit wording visible |

## Role Review Notes

| Role Lens | Verification Impact | Disposition |
|---|---|---|
| V&V lens | Methods are credible and mapped 1:1 to requirements; unrun checks are `pending`, not faked. | pass |
| Citation Auditor | Evidence pointers are real (commands run) or explicitly future. | pass |
| Numeracy Checker | The one quantity (0 errors) is a real command result. | pass |
| Scope Keeper | Verification stays at method/result level; REQ-016 scale check named. | pass |

Fixed-point note: no actionable finding required a change. The plan honestly separates
verified-now (process/doc) from pending (implementation). No unresolved critical/major
finding.
