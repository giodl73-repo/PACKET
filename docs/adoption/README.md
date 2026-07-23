---
name: PACKET Open Adoption Guide
slug: packet-open-adoption-guide
type: adoption
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - README.md
  - docs/findings/2026-06-broadband-adoption-divide.md
---

# PACKET Open Adoption Guide

## Purpose

PACKET is public and open to use. Use it as a reference model for evidence-gated
connectivity analysis, as a cited finding, or as a pattern for a bounded
regional, market, middle-mile, affordability, resilience, or service-quality
adaptation.

Public use does not create an engineering plan, RF or optical design,
regulatory filing, network build plan, advocacy brief, FCC endorsement, NTIA
endorsement, carrier endorsement, or standards-body endorsement.

## Fast Paths

| If You Are | Start With | What You Can Do |
|---|---|---|
| Public reader | [`README.md`](../../README.md) | Understand the connectivity service-promise model. |
| Researcher | [`docs/findings/2026-06-broadband-adoption-divide.md`](../findings/2026-06-broadband-adoption-divide.md) | Inspect the cited adoption-divide finding. |
| Planner or reviewer | [`local-adaptation-worksheet.md`](local-adaptation-worksheet.md) | Scope a connectivity gap without treating coverage as binary proof. |
| Builder or contributor | [`docs/vtrace/`](../vtrace) | Work from requirements, traceability, and evidence labels. |

## First Local Adaptation

1. Pick a bounded region, market, middle-mile path, last-mile area, or resilience
   scenario.
2. Name the promise: coverage, capacity, latency, availability, affordability,
   competition, or path diversity.
3. List source surfaces: FCC/NTIA, state broadband office, carrier, outage,
   price, speed, route diversity, or local evidence.
4. Mark every claim as source-backed, heuristic, held, source-needed, or
   confidence-limited.
5. Produce a short readout: service promise, current gap, evidence holds, and
   next source asks.

## Contribution Targets

- source inventories for regional or local connectivity evidence;
- corrections to coverage, adoption, affordability, or competition claims;
- resilience, outage, middle-mile, or path-diversity review;
- safer public wording that avoids coverage-as-connectivity drift.

Use GitHub issue templates for local adaptations and source/claim corrections.
Pull requests should use `.github/PULL_REQUEST_TEMPLATE.md`.

## Gate

Decision: **open_for_reference_review_and_adaptation**

Rationale: PACKET can be reused as an inspectable connectivity analysis pattern.
Reuse alone does not create regulatory, engineering, build-plan, funding,
procurement, endorsement, or validation claims.
