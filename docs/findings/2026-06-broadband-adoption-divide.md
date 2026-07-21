# Finding: US state broadband — a real divide the systemic gap test misses

Scale: **regional** (US states). Pipeline: corpus → score → gap, run end-to-end
by `packet-cli` on cited data. Reproduce:

```
packet-cli gap --scale regional --corpus corpus
```

## Corpus

10 US states modeled as `regional`-scale broadband markets (`corpus/us-*.md`),
spanning the documented digital divide. Two dimensions are assessed:

- **DIM-01 access/adoption adequacy** — derived from the **cited** ACS 2023
  household broadband subscription rate (Census ACS Table S2801,
  `acs-s2801-2023`) via the stated transform
  `adequacy = clamp(adoption_percent - 85, 0, 10)`.
- **DIM-02 availability adequacy** (100/20 Mbps) — **estimated**, anchored to the
  FCC National Broadband Map (~94% of US locations served, Dec 2024) and its
  documented rural-coverage deficits; no precise per-state served percentage is
  asserted. Treat as order-of-magnitude only.

DIM-03..DIM-13 are **not assessed** in this pass; the tool reports only assessed
dimensions so absent scores are not mistaken for gaps.

## Result

| Dimension | Corpus mean (adequacy 0–10) | Systemic gap region (mean < 5)? |
|---|---|---|
| DIM-01 adoption (cited) | 5.92 | no |
| DIM-02 availability (estimated) | 6.25 | no |

**Systemic gap regions: 0 — a null result.** Across these 10 states the mean
adequacy on both assessed dimensions clears the 5.0 bar, so the dimension-mean
gap test flags no systemic regional gap. Per the hypothesis, **a rigorous null
result is a valid outcome** and is reported as such.

## But the divide is real — per-entry deviation (DIM-01, cited)

| Market | Adoption % (ACS 2023) | DIM-01 adequacy | Deviation vs mean (5.92) |
|---|---|---|---|
| Mississippi | 86.5 | 1.50 | **−4.42** |
| West Virginia | 86.8 | 1.80 | −4.12 |
| Louisiana | 88.0 | 3.00 | −2.92 |
| Arkansas | 88.4 | 3.40 | −2.52 |
| New Mexico | 88.6 | 3.60 | −2.32 |
| New Jersey | 94.0 | 9.00 | +3.08 |
| Utah | 94.1 | 9.10 | +3.18 |
| Washington | 94.1 | 9.10 | +3.18 |
| California | 94.2 | 9.20 | +3.28 |
| Colorado | 94.5 | 9.50 | +3.58 |

Mississippi's adoption adequacy (1.50) sits **4.42 points below the corpus mean**
and **8.0 points below Colorado**; the five lowest states (MS, WV, LA, AR, NM)
cluster at 1.5–3.6 while the five highest sit at 9.0–9.5. DIM-02 (availability,
estimated) shows the same ordering.

## Methodological finding — and the fix

The original `find_gaps` test emitted a gap region only when a dimension's
**mean** adequacy across the corpus fell below 5.0. On a corpus split between
well- and under-served markets the mean clears the bar, so a **real, large
intra-corpus regional disparity returned a systemic null** — the test
under-detected within-scale inequity.

**This pass implements the fix.** `packet-gap` now also computes a
**tail (dispersion) gap**: for each assessed dimension it flags the under-served
tail when the worst-served quartile's mean falls below the adequacy bar, even if
the corpus mean does not. On this corpus the tail signal fires where the mean
test stays silent:

```
systemic gap regions (assessed): 0
tail gap regions (assessed): 2
tail-gap DIM-01 | tail_mean=2.10 | under-served: us-ar, us-la, us-ms, us-nm, us-wv
tail-gap DIM-02 | tail_mean=3.83 | under-served: us-ar, us-la, us-ms, us-nm, us-wv
```

So the corrected pipeline reports the honest dual result: **no systemic gap at
the corpus-mean level, but a detected tail gap** naming the five under-served
states (AR, LA, MS, NM, WV) on both assessed dimensions. The mean threshold
remains a blunt instrument for an inequitably distributed service; the tail
signal is the complement that catches it.

## Evidence labels & limits

- DIM-01 adequacy rests on a **cited** adoption percentage (ACS S2801 2023); the
  0–10 score is a transparent provisional transform.
- DIM-02 is **estimated**, not a per-state cited figure.
- **Adoption ≠ availability**: ACS measures whether households subscribe; FCC
  measures whether 100/20 service is available. They are kept as distinct
  dimensions and must not be conflated.
- Sources: `data/sources.md`.
