# WP-WEB-001 — local corpus and gap workbench

Status: **accepted for implementation**, 2026-11-01. This is a first-party
consumer work package, accepted through the repository's simulated role-review
practice under the owner's continuing WASM task. It is not independent human
approval, acceptance of user findings, downstream owner acceptance, or authority
to publish a connectivity finding. No implementation was written before this
review reached its fixed point.

## Scope and trace

Parent: REQ-001/002/003/005/008/009/011/012/013/015/016, IF-001/003/004;
consumer boundary PKG-WEB-001; CR-001..008. Existing WP-001..006 and research
finding acceptance retain their own evidence. No new shared provider contract.

Reader task: supply a JSON bundle of named Markdown corpus documents, choose one
explicit scale, inspect native parsing/admission, source labels and quantities,
complete/partial dimension coverage, native mean/tail signals and capacity-only
tier screens. Export original input, current request, native workbench report,
source, WASM, provenance and notices. No corpus is fetched or persisted.

The standalone unpublished web/adapter workspace consumes existing packet-corpus,
packet-score, packet-tier and packet-gap APIs. Core source, workspace manifest,
root lockfile and CLI remain unchanged from integrated baseline a4421146. Preserve
TRACKER's c08e97b public-finding authority gate. No new scoring rubric or network
model, topology simulation, cross-scale aggregation, intervention recommendation,
real-data finding publication or calibrated/validated promotion.

## Admission and interpretation

Envelope: scale plus raw corpus_json string. The corpus is an object with a
documents array of name/markdown pairs; unknown JSON fields are rejected. Bounds:
1 MiB envelope, 512 KiB corpus JSON, 32 documents, 32 KiB/4,096 lines per document,
256 KiB combined Markdown, 128 UTF-8 bytes per unique name. Native IDs must be
unique. Require leading closed frontmatter; report unknown frontmatter keys,
hold malformed/duplicate identity/score fields, reject non-finite/out-of-range
scores and quantities, and retain original input independently. Native validate
holds missing scale or uncited quantities; empty market/kind and invalid tier
also hold in the companion. Any held/rejected record prevents analysis, with a
per-record reason. An empty selected scale yields insufficient data, not adequacy.

Call find_gaps with cross_scale=false and Rubric::v0. The native scorer defaults
absent dimensions to zero. Preserve its complete native result, but distinguish
fully supplied dimension intersections from partial/unassessed dimensions in
the user-facing summary and export. Never promote missing/defaulted scores as
measured deficits. Show per-dimension coverage counts and raw ratings. A native
null result means no modeled flags, not a well-served market or complete SLA.
Rubric weights are provenance; the core gap routine does not apply them.

Call tier_sla_gap only when the first native Mbps-matching quantity has exact
unit Mbps, is finite and non-negative, and tier is explicit T1..T4; otherwise
capacity is unassessed. This is only a provisional capacity screen. Latency,
availability, affordability, peak/average load and topology/diverse-path
conformance remain unassessed. Do not call conformance with an invented graph.
Preserve caller source IDs/labels; presence is not registry or source validation.
Examples use only authored synthetic records with provisional fixture labels.

## Review findings and fixed point

All seven parliament voices were applied before all three editorial voices.
These are producer simulations against committed role files, not endorsements.

| Role | Challenge and resolution | Disposition |
|---|---|---|
| Network / Broadband Planner | Partial corpus cannot speak for a market. Export selected IDs/markets/scale, excluded IDs and coverage; all examples synthetic. | pass |
| Network Engineer | A capacity number cannot prove a buildable route or complete SLA. Limit tier screen to explicit Mbps and tier; mark other SLA and physical bases unassessed. | pass |
| Reliability & Operations Engineer | No peak/load or network topology is supplied. Do not simulate conformance or claim performance/reliability. | pass |
| Telecom Economist | Scores are not calibrated benefit-cost or take rate. Preserve provisional rubric and supplied ratings; no funding/build recommendation or economic promotion. | pass |
| Digital-Equity Advocate | Mean-level null can hide a tail; missing dimensions can fabricate one. Show both native mean/tail outputs and complete/partial coverage, preserving member IDs. | pass |
| Resilience & Security Engineer | No path/failure evidence exists. Mark resilience/topology unassessed beyond caller ratings; keep own data in tab; bound worker and file work. | pass |
| Incumbent-ISP & Right-of-Way Realist | Scores do not establish access rights, prices or market feasibility. Carry the PF-05 authority boundary in page and exports; no intervention proposal. | pass |
| Citation Auditor | Source IDs are caller assertions. Register authored synthetic fixture provenance; preserve quantities/labels and original text; report ignored/malformed frontmatter. | pass |
| Scope Keeper | Reject silent cross-scale mixing and promotion. Single explicit scale, named market set, no validated badge and no finding release approval in generated output. | pass |
| Numeracy Checker | Native absent-score zero and unit substring matching can mislead. Separate complete dimensions, reject invalid numbers, gate capacity units; exact native/WASM parity fixtures. | pass |

Critical/major findings on missing-score interpretation and fabricated SLA
conformance are resolved in the scope above. No unresolved critical/major finding.
Deferred: real source verification/calibration and finding review belong to the
next research/source work package, with downstream owner acceptance before use.
This WP is accepted solely to implement the specified browser consumer.

## Exit gates

L0: companion native tests, deterministic output, hold/reject/missing-data,
coverage, scale isolation, mean/tail/null and capacity-screen cases. L1: core
fmt/strict Clippy/locked tests/CLI help and retained public-finding PowerShell
check; companion fmt/strict host+WASM Clippy/tests; docs QA. L2: real-worker
native/WASM parity, own input, inert rendering, original UTF-8/CRLF, source/report
downloads, invalid files, file/worker deadlines, stale events, keyboard/mobile/
no-JS, clean pinned artifact and exact-default hosted/live custody.
Worker startup/compute deadlines 12/10 seconds; file recovery five seconds.
Native source/corpus/first-finding/research statuses must remain unchanged.
