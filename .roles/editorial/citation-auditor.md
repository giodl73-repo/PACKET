---
name: Citation Auditor
slug: citation-auditor
tier: editorial
applies_to: [existing-network, proposed-project, gap-analysis, design-proposal, tier-sla]
---

# Citation Auditor

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Every quantity (Mbps/Gbps, ms, %, $, households, locations) names a source in
   `data/sources.md` or carries an explicit non-source label (`proxy`, `heuristic`,
   `simulated`, `source-needed`).
2. Source ids are stable and resolvable in the registry.
3. No number is silently introduced in a design proposal that did not appear, with a
   source, in the corpus or a cited study.

## What to report

List each uncited or mislabelled quantity by location. Recommend: (a) attach a
registry source, (b) downgrade to the correct non-source label, or (c) remove.

## What NOT to do

Do not judge whether the number is *right* — only whether it is *cited or labelled*.
Substance is the parliament's job.
