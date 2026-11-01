# Reproduce the local corpus workbench

Accepted scope and full simulated role review are in
[WP-WEB-001](docs/vtrace/BROWSER_WORK_PACKAGE.md). This unpublished standalone
consumer uses existing packet-corpus, packet-score, packet-tier and packet-gap
APIs. The integrated core source, manifest, root lockfile and CLI stay unchanged.
No supported cross-repo provider contract or external consumer is added.

Install Rust 1.95.0 with rustfmt, Clippy and wasm32-unknown-unknown, Python 3.12,
Node 22 and wasm-bindgen-cli 0.2.127, then run:

```sh
cargo +1.95.0 test --workspace --locked
cargo +1.95.0 test --locked --manifest-path web/adapter/Cargo.toml --all-features
cargo +1.95.0 clippy --locked --manifest-path web/adapter/Cargo.toml --all-features --all-targets -- -D warnings
npm ci
npx playwright install chromium
python -B tools/build-pages.py
npm run test:pages
python -B tools/serve-pages.py
```

Open http://127.0.0.1:4177/PACKET/. The builder explicitly selects Rust 1.95.0.
The workflow pins/checksums bindings and uses the npm lockfile. Clean builds
archive the exact Git commit in source.zip and provenance; dirty previews are
labelled and include staged files. Only the tested master artifact deploys.
Dependency notices cover the resolved adapter closure, including proc macros,
and the Rust distribution's library notices. `.cache/site-inventory.json` hashes
every delivered file; native response fixtures and browser evidence are separate
workflow artifacts. TRACKER records deployed-file hashes and fresh live replay.

## Own input

The native envelope has `scale` and `corpus_json` strings. The corpus is
`{"documents":[{"name":"market.md","markdown":"..."}]}`. Unknown JSON fields
are rejected. Import individual Markdown files or open/edit a JSON bundle.
Original valid UTF-8 and CRLF remain in input downloads until editing,
successful replacement or example/reset. Imported documents remain verbatim
inside JSON strings. Current-request exports use current editor text.

Admission: 1 MiB envelope; 512 KiB corpus JSON; 1–32 documents, unique names up
to 128 UTF-8 bytes; 32 KiB/4,096 lines per document; 256 KiB combined Markdown.
Require closed leading frontmatter, native stable IDs, scale, kind, market and
explicit T1–T4 tier. Malformed or duplicate fields, unknown score dimensions,
non-finite/out-of-range scores, malformed quantities and native holds prevent
analysis. Unknown frontmatter keys and unknown evidence-label fallback are
reported without deleting originals. All non-quantity duplicate keys hold.
Any held/rejected record prevents analysis. No selected-scale records yields
insufficient data rather than adequacy. Labels/source IDs are caller assertions,
not source authentication or registry verification.

## Interpretation and authority

Native absent scores default to zero. The report preserves complete native gap
JSON; the summary uses only dimensions explicitly supplied by every selected
entry. Coverage counts and parsed records expose missing/partial dimensions.
Rubric v0 is provisional; native find_gaps retains weights for provenance without
applying them. Mean and tail signals stay separate, at exactly one requested
scale, with the selected IDs and market set named. No cross-scale aggregation.

Capacity screening calls tier_sla_gap only when the first native Mbps-matching
quantity has exact case-sensitive unit Mbps and non-negative finite value,
and tier is explicit. Other SLA terms, load basis and topology are unassessed;
no invented network is passed to conformance. Neither a native null result nor
a clear screen proves adequate connectivity or a complete SLA.

Examples are authored synthetic provisional fixtures, registered in
data/sources.md. They publish no new empirical finding. Generated reports always
deny public-finding release approval and downstream acceptance. User findings
still require PACKET's seven-voice parliament, editorial gates and downstream
owner acceptance before external use. The retained PF-05 authority gate remains.

## Recovery and qualification

Work runs in a worker with 12-second startup and 10-second compute deadlines.
Local reads recover after five seconds. Edit/reset/retry revisions fence stale
file/worker completions; errors retain previous drafts and independent downloads.
Resource bounds are admission limits, not heap quotas. Content is inert text.
Qualification covers own files/inputs, exact native/WASM responses, missing-score
interpretation, holds, scale isolation, mean/tail/null, capacity screening,
downloads, deadlines, late events, keyboard, mobile and no-JS access.
