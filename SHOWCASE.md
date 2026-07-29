# PACKET Showcase — Internet 2.0

**Who this is for:** someone you would hand the repo to for 15–30 minutes —
a **connectivity / broadband researcher** who knows coverage ≠ service, or a
**CLI implementer** running multi-scale corpus → gap.

**Posture:** research and conceptual-design lab. **Not** an RF/optical design,
regulatory filing, network build plan, or FCC / NTIA / carrier / standards-body
endorsement.

| Audience | Open this first | Time |
|---|---|---|
| Planner / researcher | [Broadband adoption divide finding](docs/findings/2026-06-broadband-adoption-divide.md) | 15–25 min |
| CLI implementer | README crate table + gap command | 10–20 min |
| Local adapter | [Adoption guide](docs/adoption/README.md) | 15–25 min |

## One-minute pitch

**Coverage is not connectivity.**

An address can be marked served and still be unaffordable, fragile, slow, or one
cut from isolation. PACKET scores networks at international, national, regional,
or local scale across coverage, capacity, latency, availability, affordability,
competition, and path diversity.

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Transferable rule: **availability is a bundle of service promises, not a binary
coverage flag.**

## Two doors

### A. Planner / researcher path

**Question PACKET answers well:** *Where do coverage, capacity, latency,
resilience, affordability, or competition fall short at the scale the decision
is actually made?*

| Step | What to look at | Why |
|---|---|---|
| 1 | README boundary callout | No regulatory-filing framing |
| 2 | [2026-06 broadband adoption divide](docs/findings/2026-06-broadband-adoption-divide.md) | Cited run; both tested dimensions at 50% below bar |
| 3 | [Adoption guide](docs/adoption/README.md) | Region/market adaptation |
| 4 | Reproduce | `cargo run -p packet-cli -- gap --scale regional --corpus corpus` |

**Headline (cite with scope):** the documented adoption-divide analysis reports
both tested dimensions with half of observations below the declared bar — scoped
to that run, not a universal market law.

**Do not say:** FCC map replacement, carrier build order, or funding award.

### B. CLI implementer path

| Crate | Responsibility |
|---|---|
| `packet-network` | Elements, paths, markets, scale-aware links |
| `packet-corpus` | Evidence-labelled corpus validation |
| `packet-score` | DIM-01..13 score artifacts |
| `packet-tier` | Tier-SLA classification |
| `packet-gap` | Scale-filtered gaps and nulls |
| `packet-cli` | Corpus / score / tier / gap front door |

```powershell
cargo run -p packet-cli -- corpus corpus/us-ca.md
cargo run -p packet-cli -- gap --scale regional --corpus corpus
cargo test --workspace
```

Cross-scale comparisons must say so; rigorous null remains valid.

## Claim packet (this showcase)

| Field | Value |
|---|---|
| Claim text | PACKET can be shown as Internet 2.0 multi-scale connectivity scoring with a cited adoption-divide finding and scale-aware CLI. |
| Audience | Broadband/connectivity researchers; CLI implementers. |
| Evidence | README; 2026-06 finding; adoption docs; CLI path. |
| Validation | Finding-scoped run; not regulatory or engineering certification. |
| Limitations | Showcase finding is dimension-scoped; not a complete national broadband scoreboard. |
| Non-claims | RF design, FCC/NTIA filing, carrier deployment plan, endorsements. |

## Where not to start

| Avoid… | Why |
|---|---|
| Binary coverage choropleth alone | Hides price, resilience, middle-mile, competition |
| Cross-scale claims without labels | Violates method contract |

## Related

- Family hub: [`../README.md`](../README.md)
- Product plan: [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
