# PACKET — House Rules

## 1. Project Identity

PACKET is a **research and design project for Internet 2.0** — a data-driven
upgrade plan for connectivity infrastructure, applicable at international, national,
regional, and local scales. The mission: score an existing network against a
calibrated dimension pool, find the gaps (unserved/underserved access, thin
middle-mile, single-path fragility, unaffordable service, weak competition,
congested peering, tier-SLA shortfalls), and design into them.

**The architectural bet** — borrowed from ROUTE/PYLON/GAUGE/BASIN: score enough of
an existing network on enough dimensions and the design space tells you its own
structure. The gaps aren't invented; they're found. A project designed into a real
gap is better evidence than one invented from first principles.

**The testable hypothesis**: there is a set of ≤20 interventions — at a stated
scale — that, if built or adopted to Internet 2.0 standards, would close coverage
gaps, improve resilience and performance, and reduce the digital divide. **A
rigorous null result is as valid as a positive one.** Silent scope expansion to
rescue a failing hypothesis is not acceptable.

Sibling projects: **ROUTE** (highways), **PYLON** (grid), **GAUGE** (rail),
**BASIN** (water). PACKET borrows their structural patterns; PACKET's own rules
apply here.

## 2. Multi-Scale Rule

Every corpus element carries a **scale** (`international` / `national` /
`regional` / `local`) and a market/jurisdiction. Scores, tiers, gaps, and design
proposals are interpreted **within their stated scale**. A claim must not compare or
aggregate across scales without saying so. The same dimension pool and tier model
apply at every scale; only the scope of the run changes.

## 3. The Pipeline

```
CORPUS (score existing networks) → RUBRIC CALIBRATES → GAP MAP
  → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

**Anchor rule**: one existing element must go through the full pipeline (corpus
entry → calibration pass → gap-map entry) before any proposed project is analyzed.
One proposed project must survive parliament manually before any skill is built.
YAGNI is the law.

## 4. Quality Bar

- Research-paper-level estimates. Order-of-magnitude coverage, capacity, latency,
  and cost figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No performance or availability claim dressed as solved engineering — conceptual
  analysis only, with evidence labels and the measurement basis named (peak vs
  average; single-path vs path-diverse).
- No hand-waving on economics. Marginal or negative benefit-cost projects are
  reported as such.
- Data sources declared. Every corpus entry names its source (`data/sources.md`).

## 5. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap,"
"long overdue," or any pre-judged framing before the score supports it. Claims must
cite (a) dimension, (b) score, (c) corpus comparison, (d) scale. "This area scores
8.4 on Access Coverage deficit vs. a corpus mean of 5.1 at regional scale" beats
"this is a vital project."

## 6. VTRACE Governance

PACKET's planning baseline lives in `docs/vtrace/` and is authored one deliverable
at a time to a `.roles` review fixed point. Do not start implementation code until
the relevant work package is accepted.

## 7. Review Panel

Seven adversarial parliament voices and a three-role editorial gate review every
promotable artifact. See `.roles/ROLE.md`. No voice is skipped. The incumbent-ISP /
right-of-way realist exists because connectivity is built on poles, conduit,
rights-of-way, and spectrum controlled by incumbents — that market tension is a
feature, not an accident.

## 8. Portfolio Discipline

PACKET implementation changes belong in this repo. TRACKER receives only intentional
submodule pointer updates after intake. Do not make build or validation correctness
depend on TRACKER-relative paths.
