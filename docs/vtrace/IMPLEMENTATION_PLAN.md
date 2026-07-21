# Implementation Plan

## Scope

Repo: PACKET

Implementation baseline: the accepted left-side VTRACE artifacts. PACKET is greenfield;
this plan sequences the first buildable slices bottom-up so each work package compiles,
tests, and is reviewable on its own. Every work package is sized to be run end-to-end by
an implementing agent (implementation automation) in one sitting. Scale is threaded through the corpus and
gap layers.

## Baseline Inputs

| Artifact | Status | Notes |
|---|---|---|
| `MISSION.md` | accepted | NEED-001..008 |
| `CONOPS.md` | accepted | OPS-001..007 |
| `REQUIREMENTS.md` | accepted | REQ-001..016 |
| `SPECIFICATION_BASELINE.md` | accepted | DIM-01..13, scale model, SPEC-001..013, T1–T4 |
| `ARCHITECTURE.md` | accepted | PKG-001..006, downward deps |
| `INTERFACES.md` | accepted | IF-001..006 |
| `CODE_RIGOR.md` | accepted | CR-001..008 |
| `TRACE.md` | accepted | REQ→SPEC trace |
| `VERIFICATION.md` | accepted | VER matrix |
| `REVIEW.md` | accepted | pass_with_risk |
| `VALIDATION.md` | deferred | scenario validation after first corpus |

## Implementation Strategy

Build the pipeline primitive-first. The network graph kernel (`packet-network`) has no
internal dependencies and carries the load-bearing invariants (identity, connectivity,
diverse-path), so it ships first with a Cargo workspace. The corpus layer adds the
scale/market tags. Each subsequent crate depends only on already-built crates, so the
workspace always compiles and `cargo test` always runs green. The CLI is last.

## Sequencing

| Order | Product Capability | Surfaces To Edit | Work Package | Why This Order |
|---:|---|---|---|---|
| 1 | Network graph kernel: identity, connectivity, diverse-path/redundancy, latency helpers | `Cargo.toml`, `crates/packet-network/` | WP-001 | Primitive; no deps; carries invariants. |
| 2 | Corpus + scale/market tags + source registry + schema + evidence labels | `crates/packet-corpus/`, `corpus/SCHEMA.md`, `data/sources.md` | WP-002 | Depends on kernel types; adds scale; feeds all scoring. |
| 3 | Dimension scoring (DIM-01..13) + rubric version record | `crates/packet-score/` | WP-003 | Depends on corpus; needed before tier/gap. |
| 4 | Tier classification (T1–T4) + SLA conformance (DIM-13) + tier-SLA gap | `crates/packet-tier/` | WP-004 | Depends on score + kernel. |
| 5 | Gap analysis (under-served regions, scale filter, null result) | `crates/packet-gap/` | WP-005 | Depends on score + tier. |
| 6 | `packet` CLI orchestration (incl. `--scale`) + reproducible artifacts | `crates/packet-cli/` | WP-006 | Orchestrates all layers; last. |

## Source-To-Work-Package Mapping

| Source IDs | Work Package | Disposition |
|---|---|---|
| REQ-004 / REQ-005 / REQ-007 / SPEC-001 / SPEC-005 / IF-005 / PKG-001 / CR-* | WP-001 | implement |
| REQ-001 / REQ-002 / REQ-003 / REQ-005 / REQ-016 / SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 / IF-001 / IF-002 / PKG-002 | WP-002 | implement |
| REQ-006 / SPEC-004 / IF-003 / PKG-003 | WP-003 | implement |
| REQ-014 / REQ-015 / SPEC-011 / SPEC-012 / DIM-13 / IF-004 / PKG-004 | WP-004 | implement |
| REQ-008 / REQ-016 / SPEC-006 / SPEC-013 / PKG-005 | WP-005 | implement |
| REQ-001 / IF-006 / PKG-006 | WP-006 | implement |
| REQ-009 / REQ-010 / REQ-011 | — | already_satisfied (docs/`.roles` process; exercised per corpus claim) |
| REQ-012 / REQ-013 | — | already_satisfied (VTRACE/wave discipline) |

## Boundary-To-Work-Package Mapping

| Boundary IDs | Work Package | Allowed Touches | Integration Needed |
|---|---|---|---|
| PKG-001 / IF-005 | WP-001 | `crates/packet-network/**`, workspace `Cargo.toml` | no |
| PKG-002 / IF-001 / IF-002 | WP-002 | `crates/packet-corpus/**`, `corpus/`, `data/` | yes (kernel types) |
| PKG-003 / IF-003 | WP-003 | `crates/packet-score/**` | yes (corpus) |
| PKG-004 / IF-004 | WP-004 | `crates/packet-tier/**` | yes (score, kernel) |
| PKG-005 | WP-005 | `crates/packet-gap/**` | yes (score, tier) |
| PKG-006 / IF-006 | WP-006 | `crates/packet-cli/**` | yes (all) |

## Branch / Change Control

Branch strategy: one branch per work package (e.g. `wp-001-network`). Change-control
trigger: any edit to an `IF-*` contract, a `DIM-*`/tier definition, or the `scale` enum
requires updating the owning VTRACE doc in the same change. Rollback: revert the
work-package commit; crates are independent.

## Commit / Push Policy

Commit scope: one work package per commit. Push condition: L1 green (`cargo fmt --check`,
`cargo clippy -D warnings`, `cargo test`, `proof check .`). Merge/readiness: WP exit
criteria met and pulse recorded.

## Wave / Pulse Policy

Active wave: a new `context/waves/<date>-packet-implementation/` wave (created when
WP-001 starts). Pulse mapping rule: one pulse per work package. Pulse close condition: WP
exit criteria + verification commands pass + ledger updated.

## Integration Strategy

The CLI (WP-006) wires the crates into a reproducible pipeline with a `--scale` filter.
Until then each crate is exercised by its own tests. No external services are required for
WP-001; data acquisition (FLETCH) is introduced only when the corpus is populated, after
WP-002 establishes the schema and scale tags.

## Product / Process / Verification Split

| Work Package | Product Capability | Implementation Area | Verification Command | VTRACE-Only Closeout |
|---|---|---|---|---|
| WP-001 | Network graph kernel | `crates/packet-network` | `cargo test -p packet-network` | evidence/trace/review/status rows |
| WP-002 | Corpus + scale + sources + schema | `crates/packet-corpus`, `corpus/`, `data/` | `cargo test -p packet-corpus`, `proof check .` | evidence/trace rows |
| WP-003 | Dimension scoring | `crates/packet-score` | `cargo test -p packet-score` | evidence/trace rows |
| WP-004 | Tier + SLA conformance | `crates/packet-tier` | `cargo test -p packet-tier` | evidence/trace rows |
| WP-005 | Gap analysis (scale-filtered) | `crates/packet-gap` | `cargo test -p packet-gap` | evidence/trace rows |
| WP-006 | CLI orchestration | `crates/packet-cli` | `cargo run -p packet-cli -- demo`, `cargo test` | evidence/trace rows |

Boundary rule: VTRACE/work-package/proof/readiness concepts are **not** product features.
PACKET exposes no `work-package`, `prove`, or `evidence` subcommand; the CLI's product
surface is corpus/score/tier/gap analysis only.

## Verification Strategy

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
proof check .
git diff --check
```

## Validation Levels

| Level | Scope | Required Commands / Evidence | Required Before |
|---|---|---|---|
| L0 | Fast local sanity | `cargo test -p <crate>`, `proof check .` | commit |
| L1 | Full repo confidence | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, `proof check .` | push |
| L2 | Pipeline readiness | `packet` end-to-end run at a chosen scale on a seed corpus + role review | first public claim |

## Risks

| Risk ID | Risk | Mitigation | Owner |
|---|---|---|---|
| RISK-001 | Scope creep inside a work package. | Each WP has fixed exit criteria + boundary control. | PACKET maintainer |
| RISK-002 | Premature scoring before calibration. | WP-003 ships provisional scores, labelled; calibration is a later wave. | PACKET maintainer |
| RISK-003 | Data acquisition / coverage-map overstatement bleeds into WP-002. | WP-002 ships schema + fixtures only; real acquisition (FLETCH) deferred; coverage labelled proxy. | data steward |
| RISK-004 | Scale nesting (DEF-005) undecided. | WP-002 ships a flat `scale` enum; nesting is a later change-controlled extension. | PACKET maintainer |

## Role Review Notes

| Role Lens | Plan Impact | Disposition |
|---|---|---|
| Systems engineering | Bottom-up sequence keeps the workspace always-green; each WP self-contained; scale in corpus+gap. | pass |
| Scope Keeper | Product/process split explicit; no VTRACE concept leaks into the CLI. | pass |
| V&V lens | Every WP names verification commands and L0/L1/L2. | pass |
| Software-assurance lens | Each WP inherits CR-001..008 as exit criteria. | pass |

## Implementation Readiness Decision

Decision: pass

Rationale: requirements, specs, architecture, interfaces, and code rigor are accepted;
the sequence is buildable primitive-first with concrete per-WP verification, and scale is
allocated to the corpus and gap layers. WP-001 may start. Validation depth
(`VALIDATION.md`) is deferred until a seed corpus exists.
