# PACKET Pitfalls

These entries capture recurring connectivity-infrastructure evidence failure
classes and map them to PACKET controls or open repo-local risks.

## PACKET-PF-01: Coverage Map Becomes Connectivity Adequacy

**Status:** MITIGATED

**Pattern:** Served locations, passings, availability, or advertised-speed
coverage are treated as sufficient proof of adequate connectivity.

**Domain:** Corpus entries, DIM-01..13 scoring, broadband adoption finding,
public summaries, and local-adaptation reuse.

**Detection difficulty:** Coverage data is visible and official-looking, while
adoption, price, reliability, capacity, competition, and path diversity require
more labels and more review.

**Structural solution:** Keep coverage, adoption, availability, affordability,
resilience, competition, and tier/SLA dimensions separate, and make unassessed
dimensions explicit.

**Evidence:** `README.md`, `data/sources.md`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`docs/vtrace/REQUIREMENTS.md`, and `crates/packet-score/src/lib.rs`.

## PACKET-PF-02: Mean Null Result Hides Underserved Tail

**Status:** MITIGATED

**Pattern:** A corpus mean clears the adequacy threshold, so PACKET reports no
gap even though a bottom-tail cluster remains far below the bar.

**Domain:** Gap analysis, regional broadband finding, CLI output, null-result
review, and public narrative.

**Detection difficulty:** A null result is methodologically valid, and the mean
can be correct while still hiding distributional failure.

**Structural solution:** Report systemic mean gaps and tail/dispersion gaps
separately, with under-served entries named only within the declared scale.

**Evidence:** `crates/packet-gap/src/lib.rs`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`docs/vtrace/VERIFICATION.md`, and `CLAUDE.md`.

## PACKET-PF-03: Scale Mixing Manufactures Or Erases A Gap

**Status:** MITIGATED

**Pattern:** Local, regional, national, and international elements are compared
or aggregated without an explicit cross-scale basis.

**Domain:** Corpus schema, gap analysis, CLI `--scale`, adoption worksheets,
and public findings.

**Detection difficulty:** The same dimension names can appear at multiple
scales, making mixed denominators look like one comparable score table.

**Structural solution:** Require typed scale tags, hold missing-scale corpus
entries, filter gap analysis by requested scale, and require explicit
cross-scale comparison language.

**Evidence:** `corpus/SCHEMA.md`, `crates/packet-corpus/src/lib.rs`,
`crates/packet-gap/src/lib.rs`, `docs/vtrace/CODE_RIGOR.md`, and
`docs/adoption/local-adaptation-worksheet.md`.

## PACKET-PF-04: Historical Governance Text Lags Implementation

**Status:** MITIGATED

**Pattern:** Foundation VTRACE docs and implementation-wave rows continue to
describe PACKET as greenfield, pending, or code-free after the Rust workspace,
corpus, CLI, and first broadband finding exist.

**Domain:** VTRACE trace/review/verification, implementation wave, portfolio
status scoring, customer readiness, and research packet.

**Detection difficulty:** Historical VTRACE docs are internally coherent, but
they can be mistaken for current status after implementation lands.

**Structural solution:** Add current-state implementation updates to VTRACE
trace/review/verification and mark WP-001..006 done in the implementation wave
without rewriting the historical foundation record.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/REVIEW.md`,
`context/waves/2026-06-26-packet-implementation/WAVE.md`, and
`PRODUCT_PLAN.md`.

## PACKET-PF-05: First Public Finding Becomes Build Or Regulatory Authority

**Status:** MITIGATED

**Pattern:** The cited broadband adoption divide, CLI gap output, or
local-adaptation material is treated as a build plan, RF/optical design,
regulatory determination, carrier commitment, funding instruction, FCC/NTIA
endorsement, or complete national connectivity finding.

**Domain:** `SHOWCASE.md`, broadband adoption finding, README reuse path,
adoption worksheets, customer distribution, and downstream portfolio reuse.

**Detection difficulty:** The finding is reproducible and useful, and the tool
now emits concrete under-served state lists, which can invite authority claims
beyond its labels.

**Structural solution:** Keep research-lab and no-authority language visible,
preserve DIM-02 estimated posture and unassessed dimensions, and require
explicit full-panel release review before treating the finding as a public
decision artifact. PACKET now has a public finding release boundary and policy
check that block broader authority until command, scale, market set, dimensions,
source labels, allowed/blocked claims, full parliament review, editorial review,
and downstream owner acceptance are named.

**Evidence:** `README.md`, `SHOWCASE.md`,
`docs/findings/2026-06-broadband-adoption-divide.md`,
`docs/findings/public-finding-release-boundary.md`,
`docs/vtrace/VERIFICATION.md`, `docs/vtrace/REVIEW.md`, `.roles/ROLE.md`, and
`tests/check-public-finding-boundary.ps1`.
