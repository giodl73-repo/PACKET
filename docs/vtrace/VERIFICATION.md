# Verification Plan

## Scope

Repo: PACKET

VTRACE adoption scope: define verification methods and command levels for PACKET's
requirements. PACKET is greenfield, so implementation-dependent checks are `pending`
(no code/corpus to run yet). Process-level requirements that already have evidence
(VTRACE discipline, child-repo scoping, the review panel) are verified now. A method
being named here does not mean it has run — `EVID-*` records only actual results.

## Verification Matrix

| Req ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | inspect corpus regeneration commands once built | a documented regeneration path with labels + scale preserved | pending | future EVID-001 |
| REQ-002 | inspection / review | inspect evidence labels on corpus quantities | every material quantity carries an evidence label | pending | future EVID-002 |
| REQ-003 | citation audit | run `citation-auditor` lens over corpus + `data/sources.md` | every cited quantity resolves to a registry source or is labelled | pending | future EVID-003 |
| REQ-004 | schema check / inspection | validate corpus frontmatter keys against schema | stable node/link/network id present; labels are not keys | pending | future EVID-004 |
| REQ-005 | gate / data inspection | hold/reject check on unidentified, uncited, or untagged-scale rows | such rows held, not promoted | pending | future EVID-005 |
| REQ-006 | calibration record | inspect rubric version + calibration rationale | rubric changes are versioned and justified | pending | future EVID-006 |
| REQ-007 | analysis / inspection | inspect performance/availability claims for named load + redundancy basis | peak-vs-average and single-vs-diverse basis named on each claim | pending | future EVID-007 |
| REQ-008 | gap inspection / review | inspect an already-adequate market's gap artifact | null result recorded, no manufactured gap | pending | future EVID-008 |
| REQ-009 | review inspection | confirm parliament + editorial gate ran on a promoted claim | review records exist with dispositions | pass_with_risk | EVID-009 (panel exists, not yet exercised on a corpus claim) |
| REQ-010 | role review | confirm coverage/performance/resilience/affordability/competition/equity/cost/incumbent lenses represented | stakeholder lenses present in `.roles/` and applied | pass_with_risk | EVID-010 (`.roles/` panel built) |
| REQ-011 | editorial review | inspect public claims for scope boundary | outputs framed as research/tooling/conceptual design | pass_with_risk | EVID-011 (`README`/`PRODUCT_PLAN`/`MISSION` non-goals) |
| REQ-012 | git inspection | `git status --short`; confirm no TRACKER pointer touched | PACKET changes stay in the child repo | passed | EVID-012 |
| REQ-013 | wave ledger / review | inspect wave ledger + pulses for one-at-a-time discipline | each VTRACE stage settled to a fixed point in sequence | passed | EVID-013 |
| REQ-014 | schema check / inspection | validate `tier` + `sla` frontmatter and tier model | every element classified T1–T4 with declared SLA | pending | future EVID-014 |
| REQ-015 | gate / gap inspection | tier-SLA conformance (DIM-13) check on a market | tier-SLA shortfalls reported before adequacy claimed | pending | future EVID-015 |
| REQ-016 | schema check / gate | validate `scale`/`market` tags + within-scale gap filter | every element scale-tagged; cross-scale notes explicit | pending | future EVID-016 |
| REQ-DOC-001 | doc QA | `proof check .` | markdown QA clean across repo docs | passed | EVID-DOC-001 |

## Commands

```powershell
# Doc QA (active now)
proof check .
git diff --check

# Implementation levels (planned — once packet-network/CLI exist)
# cargo fmt --check
# cargo test
# packet corpus regenerate ; packet tier-sla --gate ; packet gap --scale regional
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast doc/sanity for the active VTRACE stage. | `proof check .`, `git diff --check` | passed |
| L1 | Full repo confidence before push. | L0 + future `cargo fmt --check`, `cargo test` | partial (docs pass; no code yet) |
| L2 | Readiness proof before a public claim. | corpus regeneration + `tier-sla --gate` + scale-filtered gap + role review | pending (greenfield) |

## Evidence Ledger

| Evidence ID | Type | Path / Command | Covers | Result |
|---|---|---|---|---|
| EVID-DOC-001 | report | `proof check .` (0 errors) | REQ-DOC-001 | passed |
| EVID-012 | command | `git status --short` (standalone child repo) | REQ-012 | passed |
| EVID-013 | review | `context/waves/2026-06-26-vtrace-foundation/` ledger + pulses | REQ-013 | passed |
| EVID-009..011 | review | `.roles/` panel present and applied in stage reviews | REQ-009/010/011 | pass_with_risk |
| EVID-001..008, 014, 015, 016 | (pending) | implementation-dependent | REQ-001..008/014/015/016 | pending |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| No implementation/corpus exists yet. | Most `VER-*` cannot run; results are `pending`. | defer to implementation work packages |
| Review gate not yet exercised on a real corpus claim. | REQ-009/010/011 are process-verified, not outcome-verified. | accept risk until first corpus entry |
| No `cargo` workspace yet. | L1 code checks unavailable. | defer to implementation wave |

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
