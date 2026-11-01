# PACKET Principles

These entries summarize durable PACKET decision rules for connectivity evidence,
scale, service promises, gap analysis, review, and public-authority boundaries.

## PACKET-P-01: Coverage Is Not Connectivity

**Status:** ACTIVE

**Statement:** A served-address or coverage-map signal is not enough to claim
adequate connectivity; PACKET evaluates capacity, latency, availability,
affordability, competition, path diversity, scale, and service promise.

**Rationale:** Broadband programs can overstate service when passings or
availability hide adoption, price, resilience, congestion, and middle-mile
limits.

**Decision rule:** Any promoted finding must keep assessed dimensions,
unassessed dimensions, evidence labels, source limits, and service-tier posture
visible.

**Evidence:** `README.md`, `CLAUDE.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and
`docs/findings/2026-06-broadband-adoption-divide.md`.

## PACKET-P-02: Scale And Market Bind Every Claim

**Status:** ACTIVE

**Statement:** Connectivity scores, tiers, gaps, and conceptual designs are
interpreted only within the declared scale and market unless a cross-scale
comparison is explicitly labelled.

**Rationale:** International backbone, national/regional middle-mile, metro
aggregation, and local last-mile questions can use related dimensions but have
different denominators and intervention meanings.

**Decision rule:** Missing scale or market holds corpus promotion, and
cross-scale operations require explicit opt-in or narrative qualification.

**Evidence:** `CLAUDE.md`, `corpus/SCHEMA.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/CODE_RIGOR.md`, `crates/packet-corpus/src/lib.rs`, and
`crates/packet-gap/src/lib.rs`.

## PACKET-P-03: Evidence Labels Survive The Pipeline

**Status:** ACTIVE

**Statement:** Cited, estimated, heuristic, provisional, held, source-needed,
and confidence-limited evidence states must survive corpus parsing, scoring,
tier/SLA checks, gap analysis, CLI output, and findings.

**Rationale:** A labelled estimate is useful only while readers can see that it
is not a precise cited quantity.

**Decision rule:** Source ids, evidence labels, scale tags, and measurement
basis cannot be silently dropped or upgraded as data moves across crates.

**Evidence:** `data/sources.md`, `corpus/SCHEMA.md`,
`docs/vtrace/CODE_RIGOR.md`, `crates/packet-corpus/src/lib.rs`, and
`crates/packet-cli/src/main.rs`.

## PACKET-P-04: Null Results Are Valid, Tails Still Matter

**Status:** ACTIVE

**Statement:** A rigorous null result is valid, but a corpus-mean null result
does not erase under-served tails or within-scale inequity.

**Rationale:** Split distributions can clear a mean adequacy threshold while a
cluster of markets remains far below the bar.

**Decision rule:** Gap analysis must report systemic mean gaps, null results,
and tail/dispersion gaps separately instead of rescuing or hiding a hypothesis.

**Evidence:** `CLAUDE.md`, `docs/vtrace/REQUIREMENTS.md`,
`crates/packet-gap/src/lib.rs`, and
`docs/findings/2026-06-broadband-adoption-divide.md`.

## PACKET-P-05: Research Output Is Not Build Authority

**Status:** ACTIVE

**Statement:** PACKET can support research, review, cited diagnostics, and
conceptual design, but it does not issue RF/optical plans, regulatory filings,
carrier build plans, agency endorsements, or funding instructions.

**Rationale:** Connectivity evidence and infrastructure authority have
different owners, standards, and accountability requirements.

**Decision rule:** Public-facing outputs must retain research-lab framing,
source and scale limits, and no-authority language before reuse or publication.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `SHOWCASE.md`,
`docs/vtrace/REVIEW.md`, and `.roles/ROLE.md`.
