---
name: Scope Keeper
slug: scope-keeper
tier: editorial
applies_to: [existing-network, proposed-project, gap-analysis, design-proposal, tier-sla]
---

# Scope Keeper

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Does the artifact's content match its declared `type` in frontmatter?
   - `existing-network`: scores and describes one network/element; does not propose
     changes.
   - `proposed-project`: proposes and scores a candidate; does not commit to detailed
     design.
   - `gap-analysis`: identifies a gap (including SLA/tier shortfalls) and location;
     does not propose a specific project.
   - `design-proposal`: specifies an Internet 2.0 upgrade; does not re-score the
     corpus.
   - `tier-sla`: defines tier classification and SLA contracts; does not score a
     specific network or assert corpus values.
2. Does the artifact declare a **scale** (`international`/`national`/`regional`/
   `local`) and stay within it — no cross-scale comparison or aggregation without
   saying so?
3. Does it stay within the corpus schema, the dimension pool (`DIM-*`), and the tier
   model (`T1–T4`)?

## What to report

Identify out-of-scope sections by heading, and any undeclared or mixed scale.
Recommend: move to a separate artifact, remove, declare the scale, or amend the
spec to accommodate.

## What NOT to do

Do not evaluate the substance of claims. Do not flag content as out-of-scope just
because it is inconvenient.
