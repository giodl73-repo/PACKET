# Trace Matrix

## Scope

Repo: PACKET

VTRACE adoption scope: connect PACKET's accepted requirements to mission needs, CONOPS
scenarios, controlled specification items, future design surfaces, work packages,
verification, validation, and evidence. PACKET is greenfield: design elements, work
packages, and evidence are **planned/deferred**, and this matrix exposes those gaps
honestly rather than implying built surfaces exist.

## Requirement Trace

| Req ID | Parent Need / Constraint / Scenario | Specification Item | Design Element (planned) | Work Package | Verification (planned) | Validation | Evidence | Status |
|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / CON-003 / OPS-001 | SPEC-002 / SPEC-NF-001 | corpus + data layer | deferred → IMPL_PLAN | VER-001 / command review | OPS-001 | future EVID-001 | accepted |
| REQ-002 | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | SPEC-003 | corpus + data layer | deferred → IMPL_PLAN | VER-002 / artifact inspection | OPS-001 / OPS-004 | future EVID-002 | accepted |
| REQ-003 | NEED-001 / CON-003 / CON-004 / OPS-001 | SPEC-009 | `data/sources.md` registry | deferred → IMPL_PLAN | VER-003 / citation audit | OPS-001 | future EVID-003 | accepted |
| REQ-004 | NEED-004 / CON-002 / OPS-001 | SPEC-001 | network kernel (`packet-network`) | deferred → IMPL_PLAN | VER-004 / schema check | OPS-001 | future EVID-004 | accepted |
| REQ-005 | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | SPEC-001 / SPEC-013 / SPEC-NF-003 | kernel + corpus layer | deferred → IMPL_PLAN | VER-005 / gate / data inspection | OPS-001 | future EVID-005 | accepted |
| REQ-006 | NEED-002 / NEED-005 / OPS-002 | SPEC-004 / IF-003 | scoring layer | deferred → IMPL_PLAN | VER-006 / calibration record | OPS-002 | future EVID-006 | accepted |
| REQ-007 | NEED-002 / CON-001 / OPS-003 / OPS-006 | SPEC-005 / SPEC-MB-01 | scoring + tier (measurement basis) | deferred → IMPL_PLAN | VER-007 / analysis / inspection | OPS-003 | future EVID-007 | accepted |
| REQ-008 | NEED-006 / CON-001 / OPS-003 | SPEC-006 | gap layer | deferred → IMPL_PLAN | VER-008 / gap inspection / review | OPS-003 | future EVID-008 | accepted |
| REQ-009 | NEED-005 / CON-001 / OPS-004 | SPEC-007 | review layer (`.roles`) | deferred → IMPL_PLAN | VER-009 / review inspection | OPS-004 | future EVID-009 | accepted |
| REQ-010 | NEED-003 / NEED-005 / OPS-004 | SPEC-007 | review layer (`.roles`) | deferred → IMPL_PLAN | VER-010 / role review | OPS-004 | future EVID-010 | accepted |
| REQ-011 | NEED-003 / CON-006 / OPS-004 | SPEC-008 | public docs + editorial | deferred → IMPL_PLAN | VER-011 / editorial review | OPS-004 | future EVID-011 | accepted |
| REQ-012 | CON-005 / OPS-005 | SPEC-010 | child repo / git workflow | deferred → IMPL_PLAN | VER-012 / git status | OPS-005 | future EVID-012 | accepted |
| REQ-013 | NEED-005 / OPS-005 | SPEC-010 | wave ledger / `.roles` | deferred → IMPL_PLAN | VER-013 / wave ledger / review | OPS-005 | this matrix + pulses | accepted |
| REQ-014 | NEED-007 / CON-002 / OPS-006 | SPEC-011 / IF-004 | tier/SLA layer | deferred → IMPL_PLAN | VER-014 / schema check | OPS-006 | future EVID-014 | accepted |
| REQ-015 | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | SPEC-012 / DIM-13 | tier/SLA + gap layer | deferred → IMPL_PLAN | VER-015 / gate / gap inspection | OPS-003 / OPS-006 | future EVID-015 | accepted |
| REQ-016 | NEED-008 / CON-007 / OPS-007 | SPEC-013 / SCALE-01..03 / IF-001 | corpus scale tags + gap scale filter | deferred → IMPL_PLAN | VER-016 / schema check / gate | OPS-007 | future EVID-016 | accepted |

## Cross-Stage Coverage

| Source Stage | IDs Covered | Downstream Stage | Coverage Status | Notes |
|---|---|---|---|---|
| Mission needs | NEED-001..008 | REQ-001..016 | covered | Every need drives one or more requirements; NEED-007 → REQ-014/015; NEED-008 → REQ-005/016. |
| Mission constraints | CON-001..007 | REQ-001..016 | covered | Constraints attached where they affect evidence, identity, scope, repo ops, or scale. |
| CONOPS scenarios | OPS-001..007 | REQ-001..016 | covered | Every scenario drives at least one accepted requirement. |
| Requirements | REQ-001..016 | SPEC-001..013 / SPEC-NF-001..003 / IF-001..004 | covered | Specification coverage accepted in `SPECIFICATION_BASELINE.md`. |
| Specifications | SPEC-001..013 | `VER-*` / `EVID-*` | planned | Verification IDs defined in `VERIFICATION.md`; evidence is greenfield-pending. |
| Requirements / specs | REQ-* / SPEC-* | `WP-*` | deferred | No work packages yet; implementation planning follows the minimum slice. |

## Unknowns And Deferrals Trace

| ID | Item | Downstream Owner | Status |
|---|---|---|---|
| DEF-001 | Dimension pool definitions + weights. | `SPECIFICATION_BASELINE` (pool defined, weights calibrate) / corpus wave | partially closed (pool DIM-01..13 defined; weights deferred) |
| DEF-002 | Measured (Ookla/M-Lab) vs advertised-speed performance modelling. | `SPECIFICATION_BASELINE` (SPEC-MB-02) | deferred |
| DEF-003 | Data-source acquisition + cadence. | `data/sources.md` / `VERIFICATION` | deferred |
| DEF-004 | Implementation interfaces (CLI, schema, crates). | `ARCHITECTURE` / `INTERFACES` | deferred |
| DEF-005 | Scale as flat tag vs nested hierarchy. | `SPECIFICATION_BASELINE` (SCALE-03) / `INTERFACES` | deferred |
| SPEC-UNK-001 | Cross-scale data openness for DIM-01/03/06. | data steward | discovery |
| SPEC-UNK-002 | Coverage-map overstatement vs measured data for DIM-02. | reliability reviewer | accept risk (proxy) |
| SPEC-UNK-003 | Benefit-cost basis for DIM-12. | telecom economist | defer to calibration |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | PACKET maintainer | defer to calibration |
| SPEC-UNK-005 | Scale nesting representation. | PACKET maintainer | defer (DEF-005) |

## Honest Gaps (no faked evidence)

- No `VER-*` evidence exists yet; verification methods are named but unrun. The next
  stage (`VERIFICATION.md`) defines `VER-*`; actual `EVID-*` accrue only once
  implementation work packages run.
- No `WP-*`, `ARCHITECTURE`, `INTERFACES`, or `CODE_RIGOR` artifacts exist yet; the
  trace marks these `deferred`, not satisfied.
- Dimension weights, SLA thresholds, and scale nesting are provisional; the trace does
  not claim calibrated values.

## Role Review Notes

| Role Lens | Trace Impact | Disposition |
|---|---|---|
| Scope Keeper | Trace links IDs only; asserts no score or design; REQ-016 scale traced. | pass |
| Citation Auditor | No quantities; evidence pointers are future or repo-local. | pass |
| Numeracy Checker | No arithmetic. | pass |
| Requirements-traceability lens | Every accepted REQ maps to a need/scenario and a spec; coverage gaps are labelled, not hidden. | pass |
| Reliability & Operations Engineer | REQ-007/REQ-015 trace the measurement basis and tier-SLA gating through SPEC-005/SPEC-012. | pass |
| Network Planner | NEED-007/008 trace cleanly to REQ-014/015/016 and SPEC-011/012/013. | pass |

Fixed-point note: no actionable finding required a change; the matrix's value is
exposing the greenfield gaps (no `WP-*`/`EVID-*` yet), recorded as `deferred`/`planned`
rather than fabricated. No unresolved critical/major finding.
