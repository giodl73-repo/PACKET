# Specification Baseline

## Scope

Repo: PACKET

Baseline type: target (provisional)

Baseline date: 2026-06-26

VTRACE adoption scope: define the controlled behavior PACKET intends to build — the
dimension pool, scoring scale, measurement basis, corpus schema, evidence labels,
tier model, and the **multi-scale model** — before architecture, interfaces, or
implementation planning. Because PACKET is greenfield, every item is `target`, not
observed `current`. The dimension pool is **provisional**: dimensions and their basis
are controlled here, but per-dimension anchors and rubric weights calibrate from the
scored corpus (REQ-006) and are not asserted in this file. Future work packages must
cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | PACKET thesis, hypothesis, multi-scale, pipeline. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves. | target | Product framing. |
| `CLAUDE.md` | House rules, multi-scale rule, pipeline, quality bar. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..016`, `DEF-001..005`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Scale Model (`SCALE-*`) (resolves NEED-008 / REQ-016)

PACKET runs the same methodology at any scale. Every corpus element declares a scale;
scores, tiers, and gaps are interpreted within scale.

| Scale | Meaning | Example governance |
|---|---|---|
| `international` | Submarine cables, Tier-1 transit, cross-border interconnection. | Treaties, IXP consortia, global carriers. |
| `national` | A single nation's networks and policy. | National regulators (FCC), backbone carriers. |
| `regional` | Multi-jurisdiction within a nation. | State broadband programs, regional/middle-mile. |
| `local` | A metro/market/last-mile system. | Municipal/utility/ISP footprints. |

| ID | Rule |
|---|---|
| SCALE-01 | Every corpus element carries a `scale` and a `market`/jurisdiction tag (REQ-016). |
| SCALE-02 | Scores, tiers, and gaps are interpreted within the element's scale; cross-scale comparison/aggregation requires an explicit labelled note (CON-007). |
| SCALE-03 | Scale may nest (a local market within a region within a national/international backbone); nesting representation is provisional (DEF-005). |

## Dimension Pool (`DIM-*`)

The candidate pool PACKET scores existing networks against. Each dimension is scored
0–10. Anchors and weights are **calibrated from the corpus** (REQ-006), not fixed
here. "Primary basis" names where the input comes from; "Default label" is the
evidence posture a fresh value carries until upgraded with a cited source (REQ-002,
REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Access Coverage | Served vs unserved/underserved locations at the stated scale. | FCC BDC, NTIA | source-needed |
| DIM-02 | Capacity / Throughput | Delivered vs advertised speeds available. | Ookla/M-Lab + BDC | heuristic |
| DIM-03 | Latency / Performance | Round-trip latency and performance under load. | M-Lab / RIPE Atlas | source-needed |
| DIM-04 | Network Redundancy / Topology | Diverse-path richness and centrality (graph). | Topology/route data (computable) | implemented |
| DIM-05 | Physical Resilience | Single points of failure, power, disaster exposure. | Infrastructure + hazard layers | heuristic |
| DIM-06 | Affordability | Price relative to local income at usable tiers. | Price surveys, Census income | source-needed |
| DIM-07 | Provider Competition | Number/diversity of competing providers. | FCC BDC provider counts (computable) | implemented |
| DIM-08 | Middle-Mile / Backhaul Adequacy | Transport capacity behind the last mile. | Middle-mile inventories, PeeringDB | heuristic |
| DIM-09 | Technology Future-Proofing | Medium headroom (fiber vs copper vs FWA vs satellite). | BDC technology codes | proxy |
| DIM-10 | Interconnection / Peering Richness | Peering/transit diversity and exchange presence. | PeeringDB (computable) | implemented |
| DIM-11 | Equity / Digital-Divide Exposure | Adoption + affordability gaps by community. | Census ACS, BDC, EJ | implemented |
| DIM-12 | Capital-Efficiency / Benefit-Cost | Benefit per unit capital. | Program B/C studies | heuristic |
| DIM-13 | Tier-SLA Conformance | Degree the element meets its tier's capacity/latency/availability/affordability SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/03/06 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance or
redundant dimensions are retired and informative ones promoted; the rubric version
records each change. The pool above is the v0 candidate set, not a final rubric.

## Measurement Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-MB-01 | Performance and availability dimensions (DIM-02, DIM-03, DIM-05) name the **load basis** (peak-hour congested vs average/off-peak) and the **redundancy basis** (single-path vs diverse-path) on every derived claim (REQ-007). |
| SPEC-MB-02 | Advertised speeds and coverage-map service may be used as a labelled proxy when measured data is unavailable; the proxy status must be explicit. Default scope names the basis (DEF-002 remains open for measured-data adoption). |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

PACKET classifies each network into a four-tier hierarchy — from international
backbone to premise — with a capacity/latency/availability/affordability SLA per
tier. This is the Internet 2.0 analog of the portfolio tiering. The tier hierarchy
nests within the scale model (a T1 backbone is typically international-scale). Roles
are typical, not strict.

| Tier | Name | Typical role | SLA promise (target) |
|---|---|---|---|
| T1 | International / Tier-1 Backbone | Submarine cables, long-haul, Tier-1 transit, IXPs. | Massive capacity; very low loss; diverse global paths; carrier-grade availability. |
| T2 | National / Regional Backbone | National/regional transport and middle-mile. | High capacity; low latency to backbone; route diversity; defined availability. |
| T3 | Metro / Access Aggregation | Metro fiber, central offices, head-ends, towers. | Aggregation capacity; bounded latency; redundancy to the backbone. |
| T4 | Last-Mile / Premise | Service to home/business/anchor. | Adequate, affordable, reliable service to the location. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Capacity / throughput | Delivered capacity the tier promises. | DIM-02, DIM-08 |
| Latency | Latency the tier must hold under load (basis named). | DIM-03 |
| Availability | Uptime the tier must hold, redundancy basis named. | DIM-04, DIM-05 |
| Affordability / access | Who the tier reaches and at what affordability. | DIM-01, DIM-06, DIM-11 |

SLA values per tier are **target and provisional** — exact thresholds calibrate with
the rubric (REQ-006) and are not asserted here. A tier-SLA shortfall is a first-class
gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable node/link/network identifier; provider, plan name, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | PACKET maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 / REQ-016 | product | target | A corpus entry is one markdown file with frontmatter (id, type, scale, market, termini, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | PACKET maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | PACKET maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | PACKET maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Performance/availability dimensions name the load and redundancy basis on each claim (SPEC-MB-01). | analysis / inspection | OPS-003 | reliability reviewer | high | accepted |
| SPEC-006 | REQ-008 | product | target | An already well-served, resilient, affordable market is recorded as a labelled null result; scope is not expanded to manufacture a gap. | gap-artifact inspection / review | OPS-003 | PACKET maintainer | medium | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with coverage, performance, resilience, affordability, competition, equity, benefit-cost, and incumbent-right-of-way lenses represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no construction readiness, performance validity of record, regulatory determination, or endorsement. | editorial review | OPS-004 | PACKET maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics (incl. coverage-map service) are labelled rather than cited. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; PACKET changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / git status | OPS-005 | PACKET maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed element is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | PACKET maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per element against its tier SLA; any shortfall is recorded as a tier-SLA gap and a market is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | PACKET maintainer | high | accepted |
| SPEC-013 | REQ-016 | product | target | Every element carries a `scale` and `market`/jurisdiction tag (SCALE-01); analysis runs within a scale and any cross-scale comparison carries an explicit labelled note (SCALE-02). | schema check / gate / review | OPS-007 | PACKET maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-013 | corpus file (markdown + frontmatter, incl. scale/market) | Frontmatter keys are additive; `id` immutable; `scale` from a fixed enum. | Any key rename/removal, id-semantics, or scale-enum change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or an element's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-004 / SPEC-005 | network kernel (future `packet-network`) | Graph model, topology/redundancy metrics (DIM-04), latency/path helpers. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 | corpus + data layer | File schema, scale/market tags, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/gap artifacts regenerate from documented commands with labels and scale preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 / SPEC-013 | Deterministic identity + scale | Element ids and scale tags are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Coverage (DIM-01), latency (DIM-03), affordability (DIM-06) depend on data of varying openness/accuracy across scales. | May force proxy/source-needed labels on early corpus rows. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | Coverage maps overstate service; measured data (Ookla/M-Lab) varies in coverage. | Likely proxy at v0 for data-poor markets. | accept risk (labelled proxy) | reliability reviewer |
| SPEC-UNK-003 | Benefit-cost (DIM-12) requires program assumptions. | Heuristic until grounded. | defer to corpus calibration | telecom economist |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | Affects conformance scoring. | defer to calibration | PACKET maintainer |
| SPEC-UNK-005 | Whether scale nests as a hierarchy or stays a flat tag. | Affects schema + cross-scale notes. | defer (DEF-005) | PACKET maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001, SPEC-013 | covered | Hold/reject unidentified/untagged rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric. |
| REQ-007 | SPEC-005, SPEC-MB-01 | covered | Load + redundancy basis named. |
| REQ-008 | SPEC-006 | covered | Null result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |
| REQ-016 | SPEC-013, SCALE-01..03, IF-001 | covered | Multi-scale tagging + within-scale interpretation. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..013 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior, a candidate pool, a tier model, and the scale model; it asserts no scored network or design. | pass |
| Citation Auditor | No quantities asserted; "Primary basis" names where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Only the 0–10 scale is named; the system `scale` enum is distinct from the score scale; no computed values. | pass |
| Reliability & Operations Engineer | Measurement basis is controlled (SPEC-MB-01 / SPEC-005); measured-data adoption deferred. | pass |
| Incumbent-ISP Realist | Initial draft let DIM-01 read as if coverage were access-cost-free; resolved by keeping right-of-way in SPEC-007 and affordability in the T4 SLA term. | resolved |
| Telecom Economist | Benefit-cost (DIM-12) default label set to `heuristic`; SPEC-UNK-003 records the gap. | pass |
| Digital-Equity & Resilience advocates | Affordability/divide (DIM-06/11) and resilience (DIM-05) are in the pool. | pass |

Fixed-point note: one actionable finding (coverage read as access-cost-free) was
raised and applied via the T4 affordability term + SPEC-007. No unresolved critical or
major finding remains. Pool, SLA, and scale-nesting details are explicitly provisional;
calibration and DEF-005 deferred.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..005 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: the baseline is coherent enough to drive trace, verification, and the
review gate. Residual risk is concentrated in cross-scale data openness and
coverage-map overstatement (SPEC-UNK-001/002), provisional weights/SLA thresholds, and
scale-nesting representation (DEF-005), all deferred to the corpus calibration wave
rather than blocking the minimum slice.
