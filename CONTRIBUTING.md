# Contributing

Keep PACKET scale-aware, evidence-labelled, and explicit about the difference
between analysis and engineering, regulation, or advocacy.

Useful public contributions include regional source inventories, adoption or
affordability corrections, resilience and path-diversity review, market-scope
notes, and safer public language that prevents coverage, regulatory, build-plan,
endorsement, or advocacy drift. For local adaptations, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p packet-cli -- --help
```

Do not commit raw restricted datasets, credentials, local build state, or
uncited public claims.
