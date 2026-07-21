# Interfaces

## Scope

Repo: PACKET

Interface type: target (greenfield). Controls PACKET's external and cross-layer
boundaries so future work packages cannot change them silently. IF-001..004 restate the
`SPECIFICATION_BASELINE.md` public contracts; IF-005..006 add the crate API and CLI
introduced in `ARCHITECTURE.md`. None are implemented yet.

## Interface Inventory

| ID | Interface | Type | Owner | Consumers | Compatibility Rule | Verification |
|---|---|---|---|---|---|---|
| IF-001 | Corpus entry file (incl. scale/market) | file (markdown + frontmatter) | PKG-002 | analysts, PKG-003/004/005, reviewers | Frontmatter keys additive; `id` immutable; `scale` from a fixed enum | future VER-004/016 / schema check |
| IF-002 | `data/sources.md` registry | file (registry) | PKG-002 / data steward | citation audit, all scored quantities | Source entries append/annotate; ids stable | future VER-003 / citation audit |
| IF-003 | Rubric version record | file | PKG-003 | scoring, calibration | Dimension set + weights versioned | future VER-006 / calibration record |
| IF-004 | Tier/SLA record | file | PKG-004 | tier classification, gap analysis | Tier set + SLA terms versioned; reassignment recorded | future VER-014 / schema check |
| IF-005 | `packet-network` library API | API (Rust crate) | PKG-001 | PKG-002..006 | Public types/functions semver; breaking change is change-control | future VER-007 / cargo test |
| IF-006 | `packet` CLI | CLI | PKG-006 | maintainers, agents, analysts | Subcommands/flags additive (incl. `--scale`); output schemas versioned | future VER-001 / command review |

## Interface Details

### IF-001: Corpus entry file

Purpose: one network/element as a reviewable, scored, labelled, **scale-tagged** record.

Inputs: frontmatter (`id`, `type`, `scale`, `market`, `termini`, `tier`, `sla`, source
rows) + a dimension-score block (DIM-01..13).

Outputs: a `validated`/`held`/`draft` corpus artifact joinable by `id`, filterable by
`scale`.

Errors: missing `id`, uncited quantity, or missing `scale` → held (REQ-005);
type/scope/scale drift → Scope Keeper finding.

Versioning: frontmatter keys additive; `id` semantics immutable; `scale` from a fixed
enum (international/national/regional/local); schema in `corpus/SCHEMA.md` (deferred).

Evidence: future VER-004/016; `proof check .` for doc integrity.

### IF-002: `data/sources.md` registry

Purpose: the single citation registry; every cited quantity resolves here.
Inputs: source entries (id, publisher, dataset, access, cadence).
Outputs: stable source ids referenced by corpus quantities.
Errors: cited quantity with no registry entry → Citation Auditor finding; coverage-map
service used as fact → must be labelled proxy.
Versioning: append/annotate only; ids stable. Evidence: future VER-003.

### IF-003: Rubric version record

Purpose: control the dimension pool and weights as calibration evolves.
Inputs: dimension set (DIM-01..13), weights, calibration rationale.
Outputs: a versioned rubric the corpus scores against.
Errors: retiring/adding a `DIM-*` or changing weights without a version bump →
change-control violation. Versioning: explicit version + rationale (REQ-006).
Evidence: future VER-006.

### IF-004: Tier/SLA record

Purpose: control the T1–T4 tier definitions and per-tier SLA terms.
Inputs: tier definitions, SLA terms (capacity, latency, availability, affordability),
element tier assignments.
Outputs: a versioned tier/SLA contract used by classification and gap analysis.
Errors: changing a tier/SLA term or an element's tier without a record →
change-control violation. Versioning: tier set + SLA terms versioned.
Evidence: future VER-014/015.

### IF-005: `packet-network` library API

Purpose: the graph primitive every layer builds on.
Inputs: nodes, links with attributes and identity.
Outputs: graph queries — connectivity, redundancy/diverse-path existence (DIM-04),
latency/path helpers.
Errors: typed errors for unknown/duplicate ids and bad input (no panics on expected bad
input). Versioning: semver; breaking public API change is change-control.
Evidence: future VER-007 (cargo test).

### IF-006: `packet` CLI

Purpose: orchestrate the pipeline and emit artifacts reproducibly, at a chosen scale.
Inputs: corpus/data paths, subcommands (e.g. corpus, score, tier-sla, gap), and a
`--scale` filter.
Outputs: regenerated corpus/score/tier/gap artifacts with labels and scale preserved.
Errors: non-zero exit + message on missing inputs, missing scale, or gate failure.
Versioning: subcommands/flags additive; output schemas versioned.
Evidence: future VER-001 (command review).

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| IFQ-001 | Exact corpus frontmatter schema and `corpus/SCHEMA.md` (incl. scale enum). | Defer to first corpus wave. |
| IFQ-002 | CLI output formats (JSON/CSV/markdown) per subcommand. | Defer to IMPLEMENTATION_PLAN. |
| IFQ-003 | Whether FLETCH owns acquisition behind IF-006 or a separate adapter. | Defer to intake. |
| IFQ-004 | Whether `--scale` accepts a nested path (international/national/regional/local). | Defer to DEF-005. |

## Role Review Notes

| Role Lens | Interface Impact | Disposition |
|---|---|---|
| Scope Keeper | IF-001..006 restate or extend controlled contracts; scale is a fixed enum; no scoring/design asserted. | pass |
| Configuration/change-control lens | Every interface has a compatibility rule and change-control trigger. | pass |
| Citation Auditor | IF-002 hardens the citation boundary; coverage-map proxy rule explicit. | pass |
| Network Engineer | IF-005 exposes connectivity/redundancy as typed queries with error handling, not panics. | pass |

Fixed-point note: no actionable finding required a change; interfaces are consistent with
SPEC public contracts and architecture boundaries. No unresolved critical/major finding.
Schema, output-format, and scale-nesting details deferred to IFQ-001..004.
