# Mission

## Scope

Repo: PACKET

VTRACE adoption scope: establish the mission baseline for PACKET before creating
requirements, specification baselines, trace rows, or work packages. This file is
the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`,
`WP-*`, verification, and validation records. PACKET is greenfield: this mission
defines intent ahead of any implementation, and implementation must trace back to
the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | PACKET shall turn public connectivity data (e.g. FCC BDC, Ookla/M-Lab, PeeringDB, NTIA, submarine-cable maps) into a reproducible scored corpus of existing networks and elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | PACKET shall identify and explain connectivity gaps — unserved/underserved access, thin middle-mile, single-path fragility, affordability, weak competition, and congested peering — without overstating the evidence. | Every material claim is tied to a data artifact, command, source label, confidence label, or review record. | accepted |
| NEED-003 | PACKET shall convert analysis into defensible conceptual Internet 2.0 upgrade options, not engineering studies, regulatory filings, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the measurement and economic basis labelled before publication. | accepted |
| NEED-004 | PACKET shall keep network identity stable as analysis moves from raw nodes/links to scored networks, gap regions, and design proposals. | Element-bearing artifacts join through a stable node/link/network identifier rather than a transient label, provider, or map id. | accepted |
| NEED-005 | PACKET shall expose connectivity tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | PACKET shall report a rigorous null result as a valid finding. | When the scored corpus shows a market is already well-served, resilient, and affordable, the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | PACKET shall classify each network into a four-tier hierarchy (T1 International/Tier-1 Backbone, T2 National/Regional Backbone, T3 Metro/Access Aggregation, T4 Last-Mile/Premise) and define capacity, latency, availability, and affordability SLAs per tier, so that "is connectivity adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | PACKET shall apply the same methodology at multiple scales — international (submarine cables, Tier-1 transit, cross-border interconnection), national, regional (state/middle-mile), and local (metro/last-mile) — with every element tagged by scale and market, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| PACKET maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Connectivity analyst | Inspect scored networks, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, and scale. |
| Broadband planner / reviewer | Understand why a network, tier, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Operations / reliability reviewer | See how PACKET handles uptime, peak latency, and path redundancy conceptually. | Performance and availability claims expose their load and redundancy basis and evidence level, not just an aggregate score. |
| Incumbent / right-of-way stakeholder | See whether pole/conduit/spectrum access and market power are represented honestly. | Access and overbuild assumptions are explicit and priced, not assumed free. |
| Digital-equity / community reviewer | See affordability, adoption, and digital-divide exposure before a project is promoted. | Access and equity claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

PACKET will be a data corpus, review system, and research/design process for
Internet 2.0, with an implementation built later from accepted VTRACE work
packages. It is **multi-scale by design**: the same corpus, dimension pool, and tier
model apply to a town, a state program, a national backbone, or an international
interconnection map, and a run targets a stated scale. Work happens inside a dirty
portfolio checkout, so repo-local changes must stay scoped and must not depend on
TRACKER-relative paths for build correctness. PACKET is not yet a TRACKER submodule
until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE
anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) and the scale frame (NEED-008) extend the portfolio
pattern shared with ROUTE, PYLON, GAUGE, and BASIN: connectivity is a tiered SLA
system (backbone to last-mile) that, like water, must be analyzed at whatever scale
(local to international) the question demands.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | PACKET public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable node/link/network identity; providers, plan names, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable physical identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | PACKET implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | PACKET must not claim construction readiness, performance validity of record, regulatory determination, or official agency/carrier endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local and backbone/international evidence (NEED-008). | accepted |

## Non-Goals

- PACKET is not an engineering study, network design, or RF/optical plan.
- PACKET is not a regulatory filing, tariff, or franchise determination.
- PACKET is not an advocacy brief for a specific provider, technology, or policy.
- PACKET does not predict what the FCC, NTIA, states, or carriers will build or fund.
- PACKET does not treat illustrative maps or heuristic forecasts as proof-grade
  evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, tiered SLAs, and multi-scale applicability. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims; source links are repo-local context artifacts. | pass |
| Numeracy Checker | Mission contains no arithmetic, speed, latency, or cost claims. | pass |
| Network Planner | Mission names coverage, tiering, multi-scale, and public-interest intent. | pass |
| Reliability & Operations Engineer | Mission requires load/redundancy-basis framing for performance/availability (NEED-002/003). | pass |
| Incumbent-ISP Realist | Initial draft underplayed pole/conduit/right-of-way control and market power; resolved by adding the Incumbent/right-of-way user lens and CON-006 endorsement boundary. | resolved |
| Digital-Equity & Resilience advocates | Mission names affordability/divide and resilience as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (incumbent right-of-way / market power
under-represented) was raised and applied. No unresolved critical or major finding
remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, measurement
methodology, and the scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`
