# Pulse 04: SPECIFICATION_BASELINE

Settled. Authored `docs/vtrace/SPECIFICATION_BASELINE.md` (target/provisional): the
**scale model** (SCALE-01..03: international/national/regional/local with within-scale
interpretation), the connectivity dimension pool `DIM-01..13`, the measurement basis
(SPEC-MB-01/02: load peak-vs-average + redundancy single-vs-diverse), the **T1–T4 tier
model** (International/Tier-1 Backbone → National/Regional → Metro/Access → Last-Mile)
with capacity/latency/availability/affordability SLA terms, controlled specs
`SPEC-001..013`, public contracts `IF-001..004`, and the spec gate (`pass_with_risk`).

Role-review fixed point (real `.roles`): one finding applied — coverage (DIM-01) risked
reading as access-cost-free, resolved by the T4 affordability SLA term + SPEC-007 (incumbent
right-of-way). Pool weights, SLA thresholds, and scale nesting (DEF-005) explicitly
provisional.

Validation: `proof check .`, `git diff --check`. Next: TRACE.
