# Wave: VTRACE Foundation

## Goal

Establish PACKET's VTRACE planning baseline before any implementation. Author the
VTRACE deliverables one at a time, each driven to a `.roles` review fixed point, so
later corpus, gap, and design work — and the implementation itself — trace back to
an explicit mission, requirements, and specification baseline.

## Thesis

PACKET is the Internet 2.0 build, fifth in the infrastructure-network portfolio
after ROUTE (highways), PYLON (grid), GAUGE (rail), and BASIN (water) — and, like
BASIN, explicitly multi-scale (local to international). Doing the V-model left side
first means the implementation is governed by accepted work packages. The invariant
this wave establishes: every downstream PACKET claim can name a parent `NEED-*` /
`REQ-*` / `SPEC-*` ID and a stated scale.

## Stage ledger

| Stage | File | Status | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|
| MISSION | `docs/vtrace/MISSION.md` | settled | full panel (real `.roles`) | 1 minor (incumbent right-of-way) — applied | fixed point reached | CONOPS |
| CONOPS | `docs/vtrace/CONOPS.md` | settled | full panel (real `.roles`) | 1 minor (source-label/scale custody) — applied | fixed point reached | REQUIREMENTS |
| REQUIREMENTS | `docs/vtrace/REQUIREMENTS.md` | settled | full panel (real `.roles`) | 1 minor (performance basis) — applied as REQ-007 | fixed point reached | SPECIFICATION_BASELINE |
| SPECIFICATION_BASELINE | `docs/vtrace/SPECIFICATION_BASELINE.md` | settled | full panel (real `.roles`) | 1 minor (coverage read as cost-free) — applied | pass_with_risk; fixed point | TRACE |
| TRACE … REVIEW | `docs/vtrace/*` | settled | full panel (real `.roles`) | FIND-001..003 closed; FIND-004 accepted | pass_with_risk | minimum slice complete |
| ARCHITECTURE … WORK_PACKAGES | `docs/vtrace/*` | settled | full panel (real `.roles`) | 1 minor (dep cycle) — applied | WP-001 ready; pass | implementation wave |

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | MISSION baseline | settled | `docs/vtrace/MISSION.md` authored (NEED-001..008 incl. tiering + multi-scale) and driven to a role-review fixed point. |
| 02 | CONOPS | settled | Actors, 7 scenarios (`OPS-001..007` incl. tier/SLA + scale run), role-review fixed point. |
| 03 | REQUIREMENTS | settled | 16 requirements (`REQ-001..016` incl. REQ-016 scale) traced to needs/CONOPS; fixed point. |
| 04 | SPECIFICATION_BASELINE | settled | DIM-01..13, scale model, measurement basis, T1–T4 tiers, SPEC-001..013; fixed point. |
| 05 | TRACE | settled | Requirement trace `REQ-001..016` with honest greenfield gaps; fixed point. |
| 06 | VERIFICATION | settled | VER matrix; process checks pass, implementation checks pending. |
| 07 | REVIEW | settled | 8-lane readiness gate; **pass_with_risk**; minimum VTRACE slice complete. |
| 08 | ARCHITECTURE | settled | 7 components (PKG-001..006 + review layer), scale in corpus+gap, downward-only deps; fixed point. |
| 09 | INTERFACES | settled | IF-001..006 (corpus incl. scale enum, CLI `--scale`); fixed point. |
| 10 | CODE_RIGOR | settled | CR-001..008 incl. typed scale + load/redundancy basis; fixed point. |
| 11 | IMPLEMENTATION_PLAN | settled | Bottom-up sequence WP-001..006; readiness `pass`. |
| 12 | WORK_PACKAGES | settled | Six runnable work packages for implementation automation; WP-001 `ready`; left side of the V complete. |

## Success criteria

- MISSION names users, operating context, constraints, non-goals, success criteria,
  tiering, and multi-scale applicability explicitly.
- Each VTRACE stage reaches a `.roles` fixed point before the next begins.
- Deferred items name a later stage or work package.
