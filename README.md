# PACKET

**Internet 2.0 — multi-scale connectivity infrastructure analysis and conceptual design.**

**Coverage is not connectivity.**

An address can be marked served and still be unaffordable, fragile, slow, or
one cut away from isolation. PACKET scores networks at international, national,
regional, or local scale across coverage, capacity, latency, availability,
affordability, competition, and path diversity.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> PACKET is a research and conceptual-design project. It is not an engineering
> plan, RF or optical design, regulatory filing, network build plan, or advocacy
> brief, and it claims no FCC, NTIA, carrier, or standards-body endorsement.

## Use PACKET

PACKET is public and open to use as a reference model, cited connectivity
finding, diagnostic pattern, review discipline, or local adaptation starting
point.

If you want to apply it to a region, broadband market, middle-mile question,
resilience problem, affordability gap, or connectivity service question, start
with [`docs/adoption/README.md`](docs/adoption/README.md). It lays out safe
reuse, first adaptation steps, contribution targets, and claim boundaries.

## Why this matters

Broadband programs can count passings while missing price, reliability,
middle-mile, competition, and route diversity. PACKET makes those promises
explicit and evaluates them at the scale where the decision is actually being
made.

The transferable principle is: **availability is a bundle of service promises,
not a binary coverage flag.**

## What is implemented

| Crate | Responsibility |
|---|---|
| `packet-network` | Connectivity elements, paths, markets, and scale-aware relationships. |
| `packet-corpus` | Evidence-labelled corpus parsing and validation. |
| `packet-score` | DIM-01..13 score artifacts. |
| `packet-tier` | Tier-SLA classification and shortfall reporting. |
| `packet-gap` | Scale-filtered gap analysis and null-result reporting. |
| `packet-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

## Evidence

The first cited
[broadband adoption divide analysis](docs/findings/2026-06-broadband-adoption-divide.md)
reports both tested dimensions with 50% of observations below the declared bar.

That result is scoped to the cited run, not a universal claim about every
network or market.

## Quick start

```powershell
cargo run -p packet-cli -- corpus corpus/us-ca.md
cargo run -p packet-cli -- gap --scale regional --corpus corpus
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Every element carries a scale and market. Cross-scale comparisons must say so,
and a rigorous null result remains valid.

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md) — scope, product shape, and next work.
- [`docs/adoption/`](docs/adoption) — open reuse, local adaptation, and review path.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).
