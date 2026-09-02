# PACKET Invariants

These entries summarize properties that must remain true for PACKET corpus
entries, score artifacts, tier/SLA checks, gap runs, findings, review gates, and
future reuse.

## PACKET-I-01: Scale Tags Are Non-Optional For Promotion

**Status:** VERIFIED

**Claim:** Corpus entries without a recognized scale are held and cannot be
treated as promoted gap evidence.

**Why it matters:** Unscaled data can make a local, regional, national, or
international claim look comparable when it is not.

**Enforcement:** `Scale` is a typed enum, corpus validation emits
`MissingScale`, gap analysis filters by requested scale, and tests cover
missing-scale and cross-scale behavior.

**Evidence:** `corpus/SCHEMA.md`, `crates/packet-corpus/src/lib.rs`,
`crates/packet-gap/src/lib.rs`, `docs/vtrace/CODE_RIGOR.md`, and
`docs/vtrace/VERIFICATION.md`.

## PACKET-I-02: Uncited Or Estimated Quantities Stay Labelled

**Status:** VERIFIED

**Claim:** Quantities without source ids are held, and estimated quantities
remain labelled as estimates instead of becoming cited measurements.

**Why it matters:** Adoption, availability, affordability, and resilience
figures can look authoritative even when one dimension is a proxy or
order-of-magnitude estimate.

**Enforcement:** Corpus parsing preserves quantity labels/source ids,
validation holds uncited quantities, `data/sources.md` declares source posture,
and the broadband finding names DIM-02 as estimated.

**Evidence:** `data/sources.md`, `corpus/SCHEMA.md`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`crates/packet-corpus/src/lib.rs`, and `docs/vtrace/VERIFICATION.md`.

## PACKET-I-03: Tail Gaps Are Separate From Systemic Mean Gaps

**Status:** VERIFIED

**Claim:** PACKET reports corpus-mean gap regions and under-served tail regions
as separate outputs.

**Why it matters:** A mean-level null result can be true while the bottom-tail
markets still represent a meaningful connectivity divide.

**Enforcement:** `packet-gap` computes `GapRegion` and `TailGapRegion`
separately, tests cover split-corpus tail detection, and the CLI reports both
counts.

**Evidence:** `crates/packet-gap/src/lib.rs`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`docs/vtrace/VERIFICATION.md`, and `README.md`.

## PACKET-I-04: Tier/SLA Shortfalls Stay Distinct From General Gaps

**Status:** VERIFIED

**Claim:** Tier classification, provisional SLA conformance, DIM-13 shortfalls,
and scale-filtered gap regions are distinct artifacts.

**Why it matters:** A general adequacy score can hide service-promise failures,
and a tier/SLA shortfall should not imply a full regulatory or engineering
finding.

**Enforcement:** `packet-tier` owns T1-T4 classification and SLA gaps,
`packet-gap` consumes tier gaps without collapsing them into mean regions, and
tests cover conforming and shortfall cases.

**Evidence:** `crates/packet-tier/src/lib.rs`,
`crates/packet-gap/src/lib.rs`, `docs/vtrace/WORK_PACKAGES.md`, and
`docs/vtrace/VERIFICATION.md`.

## PACKET-I-05: Public Reuse Requires Review Boundary Language

**Status:** VERIFIED

**Claim:** PACKET can be used as a reference model or local adaptation starting
point, but any public or downstream claim needs scope, citation, numeracy, and
parliament/editorial review evidence.

**Why it matters:** A useful diagnostic tool can be mistaken for a carrier
engineering plan, regulatory determination, or agency-endorsed build program.

**Enforcement:** README/showcase boundary language, adoption docs, role panel,
review gate, and no-authority language keep public reuse bounded.

**Evidence:** `README.md`, `docs/adoption/README.md`, `.roles/ROLE.md`,
`docs/vtrace/REVIEW.md`, and `PRODUCT_PLAN.md`.

## PACKET-I-06: Public Findings Require Release Evidence

**Status:** VERIFIED

**Claim:** A PACKET finding cannot become public build, regulatory, funding,
procurement, advocacy, carrier, FCC/NTIA, standards-body, or complete national
connectivity authority unless the release record names command, scale, market
set, assessed and unassessed dimensions, source labels, exact allowed and
blocked claims, full parliament review, editorial review, and downstream owner
acceptance.

**Why it matters:** A reproducible finding can be useful and still be overread
as an engineering plan or public authority once it leaves the repo-local
research context.

**Enforcement:** `PACKET-PF-05` is guarded by the public finding release
boundary, README/adoption/finding/VTRACE wording, role gate, and policy check.

**Evidence:** `docs/findings/public-finding-release-boundary.md`, `README.md`,
`docs/adoption/README.md`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`docs/vtrace/VERIFICATION.md`, `.roles/ROLE.md`, and
`tests/check-public-finding-boundary.ps1`.
