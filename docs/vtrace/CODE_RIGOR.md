# Code Rigor

## Scope

Repo: PACKET (all `packet-*` crates)

Risk level: medium (research tooling with critical graph/identity/label/scale logic; no
safety-of-life, but wrong coverage/performance/SLA claims are harmful if overstated).

Language/toolchain: Rust 2021, `cargo fmt`, `cargo clippy`, `cargo test`.

These constraints are the pre-code agreement every work package must satisfy. implementation automation (or
any implementer) treats them as exit criteria, not aspirations.

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Functions stay small enough for focused review; soft cap 60 logical lines. | All `packet-*` code | review + `cargo clippy` (`too_many_lines`) | Larger units need a rationale comment. |
| CR-002 | Complex control flow is bounded, tested, or justified. | Graph/scoring/tier logic | tests + review | Record why complexity is necessary. |
| CR-003 | Public APIs and CLIs handle invalid input with typed errors; no panics on expected bad input. | IF-005, IF-006, crate APIs | interface tests | Waive only for truly impossible states, with rationale. |
| CR-004 | Critical invariants have tests: stable identity, scale-tag presence, connectivity/diverse-path correctness, 0–10 score bounds, label preservation. | kernel, corpus, score, tier | unit/property tests | Explain if enforced elsewhere. |
| CR-005 | `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` are clean or waived. | whole workspace | tool output | Waivers need owner + revisit trigger. |
| CR-006 | No `unwrap`/`expect`/`panic!` in library code paths except tests and documented invariants. | `packet-network/corpus/score/tier/gap` libs | `cargo clippy`, review | Allowed in `packet-cli` top-level error reporting and tests. |
| CR-007 | Evidence labels, source ids, and scale tags are never silently dropped when data flows between layers. | corpus → score → tier → gap | tests + review | None; this is a mission invariant (REQ-002/003/016). |
| CR-008 | Outputs are deterministic given the same inputs (stable ordering, stable ids). | all generators | tests | Document any intentional nondeterminism. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Errors | Each lib crate defines a `thiserror` error enum; `packet-cli` uses `anyhow`. | Typed library errors (CR-003), ergonomic CLI. |
| Numbers | Throughput = Mbps/Gbps, latency = ms, availability = % / nines; never conflate; scores are `f64` in `[0,10]`. | Numeracy discipline (editorial gate). |
| Scale | `scale` is a first-class enum (international/national/regional/local), not a free string; cross-scale ops require an explicit marker. | Enforce REQ-016/CON-007 at the type level. |
| Performance basis | Performance/availability values carry their basis — load (peak/average) and redundancy (single/diverse) — as typed data, not prose. | Enforce REQ-007/SPEC-MB-01 at the type level. |
| Tests | Every public function in a lib crate has at least one test; invariants get dedicated tests. | CR-004 coverage. |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
| (none yet) | — | — | — | — | — |

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-005 | `cargo fmt --check` | pending | per work package |
| EVID-CR-002 | CR-005, CR-006 | `cargo clippy -- -D warnings` | pending | per work package |
| EVID-CR-003 | CR-004 | `cargo test` | pending | per work package |

## Role Review Notes

| Role Lens | Code-Rigor Impact | Disposition |
|---|---|---|
| Software-assurance lens | Constraints are pre-code, testable, and mapped to verification commands. | pass |
| Reliability & Operations Engineer | The performance-basis tailoring rule pins load + redundancy into the type system, not prose. | pass |
| Scope Keeper | The scale tailoring rule makes scale a typed enum and cross-scale ops explicit. | pass |
| Citation Auditor | CR-007 makes label/source/scale preservation a hard invariant. | pass |

Fixed-point note: no actionable finding required a change. Constraints are credible and
command-backed. No unresolved critical/major finding.
