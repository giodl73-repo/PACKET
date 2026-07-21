# PACKET

**Internet 2.0 — multi-scale connectivity infrastructure analysis and conceptual design.**

PACKET scores connectivity networks at international, national, regional, or
local scale, classifies service tiers, and finds evidence-backed gaps in
coverage, capacity, latency, availability, affordability, competition, and
path diversity.

> PACKET is a research and conceptual-design project. It is not an engineering
> plan, RF or optical design, regulatory filing, network build plan, or advocacy
> brief, and it claims no FCC, NTIA, carrier, or standards-body endorsement.

## What is implemented

| Crate | Responsibility |
|---|---|
| `packet-network` | Connectivity elements, paths, markets, and scale-aware relationships. |
| `packet-corpus` | Evidence-labelled corpus parsing and validation. |
| `packet-score` | DIM-01..13 score artifacts. |
| `packet-tier` | Tier-SLA classification and shortfall reporting. |
| `packet-gap` | Scale-filtered gap analysis and null-result reporting. |
| `packet-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

The first cited broadband-divide run reports both tested dimensions with 50%
of observations below the declared bar. That result is scoped to the cited run,
not a universal claim about every network or market.

## Quick start

```powershell
cargo run -p packet-cli -- --help
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
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).
