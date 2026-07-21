# Data Sources

Every corpus quantity names a `source_id` referenced here. Evidence labels:
`cited` = drawn from the named public source; `estimated` = order-of-magnitude
analytical estimate anchored to a cited source, not a direct figure.

| source_id | Source | What it provides | Accessed |
|---|---|---|---|
| `acs-s2801-2023` | U.S. Census Bureau, American Community Survey (ACS) 2023, Table S2801 "Types of Computers and Internet Subscriptions" | Percent of households with a broadband internet subscription, by state | 2026-06 (via public ACS summaries) |
| `fcc-nbm-2024` | FCC National Broadband Map / Broadband Data Collection, December 2024 update | Percent of US locations served with fixed 100/20 Mbps broadband (~94% nationally) and documented urban/rural coverage gaps | 2026-06 |

## Notes on rigor

- **Adoption vs. availability are different things.** `acs-s2801-2023` measures
  whether households *subscribe* (adoption); `fcc-nbm-2024` measures whether
  100/20 Mbps service is *available* at a location (availability). The corpus
  keeps these as distinct dimensions (DIM-01 adoption, DIM-02 availability).
- **DIM-01** adequacy scores are a transparent transform of the cited adoption
  percentage: `adequacy = clamp(adoption_percent - 85, 0, 10)`. The underlying
  percentage is `cited`; the derived 0–10 score is provisional analysis.
- **DIM-02** availability adequacy scores are `estimated`: the FCC map gives a
  national served rate and qualitative rural-gap evidence, but this corpus does
  not assert a precise per-state served percentage. Treat DIM-02 as
  order-of-magnitude only.
- Secondary aggregators (e.g. broadbandexpanded.com, overflowdata.com) were used
  to retrieve ACS values; the primary citation remains ACS Table S2801.
