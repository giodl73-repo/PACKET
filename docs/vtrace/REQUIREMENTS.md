# Requirements

## Scope

Repo: PACKET

VTRACE adoption scope: derive initial repo-level requirements from
`docs/vtrace/MISSION.md` and `docs/vtrace/CONOPS.md`. These requirements describe what
PACKET must satisfy as analysis and implementation proceed; they do not by themselves
authorize implementation work — that comes from accepted work packages. Requirements
stay at contract level and assert no scores or designs.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | PACKET shall maintain a documented regeneration path for the active corpus, score, and gap artifacts from public source data. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | PACKET maintainer | inspection / command review | accepted |
| REQ-002 | PACKET shall label every material quantity with an evidence posture (implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited). | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent proxy or planned work from reading as proof. | must | PACKET maintainer | artifact inspection / review | accepted |
| REQ-003 | PACKET shall cite a declared source in `data/sources.md` for every quantity in a corpus entry, or mark it as a labelled proxy/heuristic. | NEED-001 / CON-003 / CON-004 / OPS-001 | Uncited numbers cannot be audited or regenerated; coverage maps overstate service. | must | data steward | citation audit / inspection | accepted |
| REQ-004 | PACKET shall identify each element by a stable node/link/network identifier, not by a transient label, provider, or map id. | NEED-004 / CON-002 / OPS-001 | Stable physical identity is required before scores, gaps, and proposals can be compared. | must | PACKET maintainer | schema check / inspection | accepted |
| REQ-005 | PACKET shall hold or reject any corpus or gap artifact that lacks a stable element identifier, a declared source label, or a declared scale. | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | Mutable labels, uncited rows, and untagged scale cannot safely join across analysis stages. | must | PACKET maintainer | gate / data inspection | accepted |
| REQ-006 | PACKET shall calibrate its scoring rubric from observed corpus variance and correlation, and record the rubric version and rationale for each change. | NEED-002 / NEED-005 / OPS-002 | Calibration must be evidence-driven and auditable, not asserted. | must | PACKET maintainer | calibration record / version diff | accepted |
| REQ-007 | PACKET shall ground performance and availability claims in an explicit basis — load (peak vs average) and redundancy (single vs diverse path) — and name the basis on the claim. | NEED-002 / CON-001 / OPS-003 / OPS-006 | A throughput/uptime claim is meaningless without stating peak-vs-average load and single-vs-diverse path. | must | reliability reviewer | inspection / review | accepted |
| REQ-008 | PACKET shall record a market that is already well-served, resilient, and affordable as a valid null result rather than manufacturing a gap. | NEED-006 / CON-001 / OPS-003 | Silent scope expansion to rescue a hypothesis is forbidden. | must | PACKET maintainer | gap-artifact inspection / review | accepted |
| REQ-009 | PACKET shall route every promotable network or project claim through the 7-voice parliament and the 3-role editorial gate before downstream use. | NEED-005 / CON-001 / OPS-004 | PACKET's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-010 | PACKET shall represent coverage, performance, resilience, affordability, competition, equity/digital-divide, benefit-cost, and incumbent right-of-way posture in reviews or claim labels before a design option is promoted. | NEED-003 / NEED-005 / OPS-004 | These stakeholder lenses must remain first-class, per the mission users. | should | review steward | role review / inspection | accepted |
| REQ-011 | PACKET shall keep its outputs framed as research, tooling, review, and conceptual design — not construction readiness, performance validity of record, regulatory determination, or agency/carrier endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects PACKET from overclaiming public authority. | must | PACKET maintainer | editorial review | accepted |
| REQ-012 | PACKET shall keep implementation and VTRACE changes scoped to the PACKET child repo until an intentional TRACKER submodule pointer update after intake. | CON-005 / OPS-005 | TRACKER is the snapshot repo; PACKET owns implementation history. | must | PACKET / portfolio maintainer | git status / submodule diff | accepted |
| REQ-013 | PACKET shall advance VTRACE deliverables one at a time to a `.roles` review fixed point, recording dispositions and deferrals. | NEED-005 / OPS-005 | The one-at-a-time discipline keeps each artifact reviewable and traceable. | must | PACKET maintainer | wave ledger / review notes | accepted |
| REQ-014 | PACKET shall classify every analyzed element into exactly one tier (T1 International/Tier-1 Backbone, T2 National/Regional Backbone, T3 Metro/Access Aggregation, T4 Last-Mile/Premise) and attach the tier's declared SLA (capacity, latency, availability, affordability). | NEED-007 / CON-002 / OPS-006 | A tiered SLA system requires every element to carry a tier and a promise it is judged against. | must | PACKET maintainer | schema check / inspection | accepted |
| REQ-015 | PACKET shall assess each element against its tier SLA and report any tier-SLA shortfall as a gap before a market is described as adequate. | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | Adequacy must be measured against an explicit tier promise; SLA gaps are first-class findings. | must | PACKET maintainer | gate / gap-artifact inspection | accepted |
| REQ-016 | PACKET shall tag every element with a scale (international/national/regional/local) and market/jurisdiction, interpret scores/tiers/gaps within scale, and require an explicit labelled note for any cross-scale comparison or aggregation. | NEED-008 / CON-007 / OPS-007 | The multi-scale methodology is only sound if scale is explicit and not silently mixed. | must | PACKET maintainer | schema check / gate / review | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at contract level; REQ-016 makes scale a hard requirement. | pass |
| Citation Auditor | Requirements introduce no new numeric claims; REQ-003 hardens citation discipline against overstated coverage maps. | pass |
| Numeracy Checker | No calculations, units, scores, speed, or cost claims. | pass |
| Network Planner | Coverage, tiering, and multi-scale intent preserved via REQ-014/016/010. | pass |
| Reliability & Operations Engineer | Initial draft left the performance basis implicit; resolved by adding REQ-007 (load + redundancy basis named on the claim). | resolved |
| Incumbent-ISP Realist | Right-of-way / overbuild posture required before promotion (REQ-010). | pass |
| Digital-Equity & Resilience advocates | Affordability/divide and resilience required before promotion (REQ-010). | pass |

Fixed-point note: one actionable finding (performance/redundancy basis implicit) was
raised and applied as REQ-007. No unresolved critical or major finding remains.

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003, REQ-004, REQ-005 |
| OPS-002 | REQ-006 |
| OPS-003 | REQ-007, REQ-008 |
| OPS-004 | REQ-002, REQ-009, REQ-010, REQ-011 |
| OPS-005 | REQ-012, REQ-013 |
| OPS-006 | REQ-014, REQ-015 |
| OPS-007 | REQ-005, REQ-016 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| DEF-001 | Exact dimension pool and per-dimension definitions. | `SPECIFICATION_BASELINE.md` and first corpus-calibration wave. |
| DEF-002 | Whether performance scoring uses measured (Ookla/M-Lab) data explicitly vs. advertised speeds as a proxy. | `SPECIFICATION_BASELINE.md` once the method is chosen. |
| DEF-003 | Specific data-source acquisition commands and refresh cadence. | `data/sources.md` and `VERIFICATION.md`. |
| DEF-004 | Implementation interfaces (CLI, schemas, crates). | `ARCHITECTURE.md` / `INTERFACES.md` after the minimum slice. |
| DEF-005 | Whether scale is a flat tag or a nested hierarchy (a local market within a region within a national/international backbone). | `SPECIFICATION_BASELINE.md` / `INTERFACES.md`. |
