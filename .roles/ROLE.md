# PACKET — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of
corpus entries, gap findings, design proposals, tier/SLA definitions, and VTRACE
deliverables run against these files and record dispositions
(`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is
the output, not consensus. No voice is skipped. A good project survives all seven;
a weak one collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/network-planner.md` | Network / Broadband Planner | Coverage + public interest vs. provider-footprint framing |
| `parliament/network-engineer.md` | Network Engineer | Buildable routes/medium vs. map-fantasy throughput |
| `parliament/reliability-operations-engineer.md` | Reliability & Operations Engineer | Uptime/peak performance vs. oversubscription optimism |
| `parliament/telecom-economist.md` | Telecom Economist | Benefit-cost + take rate vs. overbuild/forecast inflation |
| `parliament/digital-equity-advocate.md` | Digital-Equity Advocate | Affordable adequate access vs. coverage-optimized bypass |
| `parliament/resilience-security-engineer.md` | Resilience & Security Engineer | Diverse-path/secure resilience vs. single-path cost |
| `parliament/incumbent-isp-realist.md` | Incumbent-ISP & Right-of-Way Realist | Poles/conduit/spectrum/market power vs. free-access & easy-overbuild assumptions |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (Mbps/ms/%/$); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring, gap
analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/rural-household.md` | Rural Household | Any adequate service, affordability, reliability |
| `stakeholders/underserved-urban-household.md` | Underserved Urban Household | Affordability, adoption, adequate speed |
| `stakeholders/small-business.md` | Small Business | Reliable symmetric service, uptime, support |
| `stakeholders/anchor-institution.md` | Anchor Institution | High-capacity reliable service; community access |
| `stakeholders/content-cloud-provider.md` | Content / Cloud Provider | Peering quality, latency, interconnection |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for PACKET research outputs. See
`panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from
parliament and editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or
tier/SLA definition is being settled, the relevant subset of this panel is applied
and dispositions are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major actionable
finding remains and every deferred item names a later stage or work package.
