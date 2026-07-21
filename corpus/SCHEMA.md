# Corpus Schema (IF-001)

A corpus entry is a Markdown file with a leading frontmatter block delimited by
lines containing exactly `---`. `packet_corpus::CorpusEntry::from_markdown`
parses the frontmatter; the body is free-form analysis prose.

## Frontmatter keys

| Key | Required | Meaning |
|---|---|---|
| `id` | yes | Stable identifier (empty id is rejected). |
| `type` / `kind` | no | Element type, e.g. `state-broadband-market`. |
| `scale` | no | One of `international`, `national`, `regional`, `local`. A missing scale is *held*, not rejected. |
| `market` | no | Market / jurisdiction label. |
| `tier` | no | Tier label (`T1`..`T4`). |
| `sla` | no | Optional SLA label. |
| `score.DIM-NN` | no | Provisional 0–10 dimension score for `DIM-NN` (NN = 01..13). |
| `quantity` | no (repeatable) | A measured value: `value \| unit \| label \| source_id`. |

## Quantity line

```
quantity: 86.5 | percent-households-with-broadband | cited | acs-s2801-2023
```

- `value` — a number.
- `unit` — a free-text unit string.
- `label` — an evidence label: `cited`, `validated`, `provisional`, or
  `estimated` (default).
- `source_id` — a key in `data/sources.md`; an empty source id marks the
  quantity **uncited** (which `validate()` holds).

## Scale model

| Scale | Scope |
|---|---|
| `international` | Submarine cables, Tier-1 transit, cross-border interconnection. |
| `national` | National backbone. |
| `regional` | State / middle-mile. US states are modeled here. |
| `local` | Metro / last-mile. |

## Validation (`validate()`)

- Empty `id` → **reject** (`MissingId`).
- Missing `scale` → **hold** (`MissingScale`).
- A quantity with no `source_id` → **hold** (`UncitedQuantity`).
- Evidence labels are preserved, never silently upgraded.
